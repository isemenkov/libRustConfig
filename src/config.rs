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

// Configuration file
pub struct Config {
    config : raw::config_t,
    root_element : Option<raw::config_setting_t>
}

// Option value type
#[derive(Debug, PartialEq)]
pub enum OptionType {
    UndefinedType,
    IntegerType,
    Int64Type,
    FloatType,
    StringType,
    BooleanType
}

pub struct OptionReader {
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
                let result = raw::config_read_file(
                    &mut self.config, 
                    file_name.as_os_str().to_str().unwrap().as_ptr() as *const i8
                );
                
                if result == raw::CONFIG_TRUE {
                    self.root_element = 
                        Some(*raw::config_root_setting(&mut self.config));
                } else {
                    self.root_element = None
                }
            }
        }
    }
    
    // Parse configuration from string
    pub fn load_from_string(&mut self, config_string : String) -> () {
        unsafe {
            let result = raw::config_read_string(
                &mut self.config, 
                config_string.as_ptr() as *const i8
            );
            
            if result == raw::CONFIG_TRUE {
                self.root_element = 
                    Some(*raw::config_root_setting(&mut self.config));
            } else {
                self.root_element = None
            }
        }
    }
    
    pub fn value(&mut self, path : String) -> OptionReader {
        unsafe {
            let option = raw::config_setting_lookup(
                &mut self.root_element.unwrap(), 
                path.as_ptr() as *const i8
            );
                
            OptionReader {
                element : Some(*option)
            }
        }
    }
}

// Destructor
impl Drop for Config {
    fn drop (&mut self) {
        unsafe { 
            raw::config_destroy(&mut self.config); 
        }
    }
}

impl OptionReader {
    
    // Constructor
    pub fn new() -> OptionReader {
        OptionReader {
            element : None
        }
    }
    
    // Return true if element is root
    pub fn is_root(&self) -> bool {
        let result =
            raw::config_setting_is_root(&self.element.unwrap());
        
        result == raw::CONFIG_TRUE
    }
     
    // Return true if element is section group
    pub fn is_section(&self) -> bool {
        let result =
            raw::config_setting_is_group(&self.element.unwrap());
        
        result == raw::CONFIG_TRUE      
    }
    
    // Return true if element is array
    pub fn is_array(&self) -> bool {
        let result =
            raw::config_setting_is_array(&self.element.unwrap());
        
        result == raw::CONFIG_TRUE
    }
    
    // Return true if element is list
    pub fn is_list(&self) -> bool {
        let result =
            raw::config_setting_is_list(&self.element.unwrap());
        
        result == raw::CONFIG_TRUE
    }
    
    // Return option element parent item
    pub fn parent(&self) -> OptionReader {
        unsafe {
            let result =
                raw::config_setting_parent(&self.element.unwrap());
            
            OptionReader {
                element : Some(*result)
            }
        }
    }
    
    // Return option value type
    pub fn value_type(&self) -> OptionType {
        let result =
            raw::config_setting_type(&self.element.unwrap());
        
        match result as i16 {
            raw::CONFIG_TYPE_INT => { OptionType::IntegerType },
            raw::CONFIG_TYPE_INT64 => { OptionType::Int64Type },
            raw::CONFIG_TYPE_FLOAT => { OptionType::FloatType },
            raw::CONFIG_TYPE_STRING => { OptionType::StringType },
            raw::CONFIG_TYPE_BOOL => { OptionType::BooleanType },
            _ => { OptionType::UndefinedType }
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