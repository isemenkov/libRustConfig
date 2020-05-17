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

use std::mem::MaybeUninit;
use std::path;

pub struct Config {
    config : Box<raw::config_t>,
    root_element : Option<Box<raw::config_setting_t>>
}

impl Config {
    
    pub fn new() -> Config {
        
        let mut cfg = MaybeUninit::<raw::config_t>::uninit();
        unsafe { raw::config_init(cfg.as_mut_ptr()); }
        let cfg = unsafe { cfg.assume_init(); };
        
        Config {
            config : unsafe { Box::from_raw(cfg) },
            root_element : None
        }
    }
    
    pub fn load_from_file(&mut self, file_name : &path::Path) -> () {
        if file_name.exists() {
            unsafe { raw::config_read_file(self.config.as_mut_ptr(), 
                file_name.as_os_str().to_str().unwrap().as_ptr() as *const i8); 
            }
        }
    }
    
    pub fn load_from_string(&mut self, config_string : String) -> () {
        unsafe {
            raw::config_read_string(self.config.as_mut_ptr(), 
                config_string.as_ptr() as *const i8);
        }
    }
}

impl Drop for Config {
    fn drop (&mut self) {
        unsafe { raw::config_destroy(self.config.as_mut_ptr()); }
    }
}