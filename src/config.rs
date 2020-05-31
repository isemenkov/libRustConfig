/******************************************************************************/
/*                               libRustConfig                                */
/*                   rust wrapper around libconfig library                    */
/*                  https://github.com/hyperrealm/libconfig                   */
/*                                                                            */
/* Copyright (c) 2020                                       Ivan Semenkov     */
/* https://github.com/isemenkov/librustconfig               ivan@semenkov.pro */
/*                                                          Ukraine           */
/******************************************************************************/
/*                                                                            */
/* Permission is hereby granted,  free of charge,  to any person obtaining a  */
/* copy of this software and associated documentation files (the "Software"), */
/* to deal in the Software without restriction, including without limitation  */
/* the rights to use, copy,  modify, merge, publish, distribute,  sublicense, */
/* and/or  sell copies  of the Software,  and to permit persons  to whom  the */
/* Software  is furnished to  do  so,  subject to  the following  conditions: */
/*                                                                            */
/* The above copyright notice and this permission notice shall be included in */
/* all copies or substantial portions of the Software.                        */
/*                                                                            */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR */
/* IMPLIED,  INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF  MERCHANTABILITY, */
/* FITNESS  FOR A PARTICULAR PURPOSE  AND NONINFRINGEMENT. IN  NO EVENT SHALL */
/* THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER */
/* LIABILITY,  WHETHER IN AN ACTION  OF CONTRACT,  TORT OR OTHERWISE, ARISING */
/* FROM,  OUT OF  OR IN  CONNECTION WITH  THE SOFTWARE  OR THE  USE OR  OTHER */
/* DEALINGS IN THE SOFTWARE.                                                  */
/*                                                                            */
/******************************************************************************/

use libconfig_sys as raw;

use std::mem;
use std::path;

// Works with configurations 
pub struct Config {
    config : raw::config_t,
    root_element : Option<raw::config_setting_t>
}

pub struct ConfigOption {
    element : Option<raw::config_setting_t>
}

impl Config {
    
    // Constructor
    pub fn new() -> Config {
        let mut c = mem::MaybeUninit::<raw::config_t>::uninit();
        let cfg = unsafe {
            raw::config_init(c.as_mut_ptr());
            c.assume_init()
        };
        
        Config {
            config : cfg,
            root_element : None
        }
    }
    
    // Load config file from file and parse it
    pub fn load_from_file(&mut self, file_name : &path::Path) -> () {
        if file_name.exists() {
            unsafe {
                let result = raw::config_read_file(&mut self.config, 
                file_name.as_os_str().to_str().unwrap().as_ptr() as *const i8);
                
                if result == raw::CONFIG_TRUE {
                    self.root_element = 
                        Some(*raw::config_root_setting(&mut self.config));
                }
            }
        }
    }
    
    // Parse configuration from string
    pub fn load_from_string(&mut self, config_string : String) -> () {
        unsafe {
            let result = raw::config_read_string(&mut self.config, 
                config_string.as_ptr() as *const i8);
            
            if result == raw::CONFIG_TRUE {
                self.root_element = 
                    Some(*raw::config_root_setting(&mut self.config));
            }
        }
    }
    
    pub fn value(&mut self, path : String) -> ConfigOption {
        unsafe {
            let option = raw::config_setting_lookup(&mut self.root_element.unwrap(), 
                path.as_ptr() as *const i8);
                
            ConfigOption {
                element : Some(*option)
            }
        }
    }
}

// Destructor
impl Drop for Config {
    fn drop (&mut self) {
        unsafe { raw::config_destroy(&mut self.config); }
    }
}

impl ConfigOption {
    
    pub fn new() -> ConfigOption {
        ConfigOption {
            element : None
        }
    }
    
    pub fn is_section(&mut self) -> bool {
        let result =
            raw::config_setting_is_group(&self.element.unwrap());
        if result == raw::CONFIG_TRUE {
            true
        } else {
            false
        }      
    }
    
    pub fn as_integer(&mut self) -> i32 {
        unsafe {
            let result =
                raw::config_setting_get_int(&mut self.element.unwrap());
            result
        }
    }
    
}