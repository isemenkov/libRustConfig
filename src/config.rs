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

use std::{mem::MaybeUninit, path};
use std::ffi::{CStr, CString};

// Configuration file
pub struct Config {
    config : raw::config_t,
    pub root_element : Option<raw::config_setting_t>
}

// Option value type
#[derive(Debug, PartialEq)]
pub enum OptionType {
    IntegerType,
    Int64Type,
    FloatType,
    StringType,
    BooleanType
}

// Writer for configuration option
#[derive(Clone, Copy)]
pub struct OptionWriter {
    element : Option<raw::config_setting_t>
}

// Reader for configuration option
pub struct OptionReader {
    element : Option<raw::config_setting_t>
}

// Errors
#[derive(Debug, PartialEq)]
pub enum Errors {
    ParseError,
    FileNotExists,
    SaveError
}

// Result type
type Result<T> = std::result::Result<T, Errors>;

impl Config {
    
    // Constructor
    pub fn new() -> Config {
        let mut c = MaybeUninit::<raw::config_t>::uninit();
        let mut cfg = unsafe {
            raw::config_init(c.as_mut_ptr());
            c.assume_init()
        };
        
        let option = raw::config_root_setting(&mut cfg);
        let element = {    
            if option.is_null() {
                None
            } else {
                Some(unsafe {*option})
            }
        };
    
        Config {
            config : cfg,
            root_element : element
        }
    }
    
    // Load config file from file and parse it
    pub fn load_from_file(&mut self, file_name : &path::Path) -> Result<()> {
        if file_name.exists() {
            unsafe {
                let result = raw::config_read_file(
                    &mut self.config, 
                    CString::new(file_name.as_os_str().to_str().unwrap())
                        .unwrap().as_ptr()
                );
                
                if result == raw::CONFIG_TRUE {
                    self.root_element = 
                        Some(*raw::config_root_setting(&mut self.config));
                    Ok(())
                } else {
                    self.root_element = None;
                    Err(Errors::ParseError)
                }
            }
        } else {
            Err(Errors::FileNotExists)
        }
    }
    
    // Parse configuration from string
    pub fn load_from_string<S>(&mut self, config_string : S) -> Result<()>
        where S: Into<String> {
          
        let result = unsafe { 
            raw::config_read_string(
                &mut self.config, 
                CString::new(config_string.into()).unwrap().as_ptr())
        };
        
        if result == raw::CONFIG_TRUE {
            let option = raw::config_root_setting(&mut self.config);
            
            if option.is_null() {
                self.root_element = None;
                Err(Errors::ParseError)
            } else {
                self.root_element = Some(unsafe {*option});
                Ok(())
            }
        } else {
            self.root_element = None;
            Err(Errors::ParseError)
        }
    }
   
   // Save current config to file
    pub fn save_to_file(&mut self, file_name : &path::Path) -> Result<()> {
        let result = unsafe { raw::config_write_file(&mut self.config, 
            CString::new(file_name.as_os_str().to_str().unwrap())
                .unwrap().as_ptr())
        };
            
        if result == raw::CONFIG_TRUE {
            Ok(())
        } else {
            Err(Errors::SaveError)
        }
    }
    
    // Set current config include directory
    pub fn include_dir(&mut self, path : &path::Path) -> () {
        unsafe {
            raw::config_set_include_dir(&mut self.config, 
                CString::new(path.as_os_str().to_str().unwrap())
                    .unwrap().as_ptr())
        }
    }
        
    // Read value from path
    pub fn value<S>(&mut self, path : S) -> Option<OptionReader>
        where S: Into<String> {
        
        if self.root_element.is_none() {
            return None
        } else {
            OptionReader::new(self.root_element).value(path)
        }
    }
    
    // Create new group section
    pub fn create_section<S>(&mut self, path : S) -> Option<OptionWriter>
        where S: Into<String> {
        
        if self.root_element.is_none() {
            None
        } else {
            OptionWriter::new(self.root_element).create_section(path)
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

impl OptionWriter {
    
    // Constructor
    fn new(elem : Option<raw::config_setting_t>) -> OptionWriter {
        OptionWriter {
            element : elem
        }
    }
    
    // Create new group section
    pub fn create_section<S>(&mut self, path : S) -> Option<OptionWriter> 
        where S: Into<String> {
            
        if self.element.is_none() {
            return None
        }
        
        let option = unsafe {
            raw::config_setting_add(&mut self.element.unwrap(), 
                CString::new(path.into()).unwrap().as_ptr(), 
                raw::CONFIG_TYPE_GROUP as i32)
        };
        
        if option.is_null() {
            None
        } else {
            Some(OptionWriter::new(Some(unsafe {*option})))
        }
    }
    
    // Add new integer value to current group
    pub fn write_integer<S>(&mut self, name : S, value : i32) -> 
        Option<OptionWriter> where S: Into<String> {
            
        if self.element.is_none() {
            return None
        };
        
        let option = unsafe {
            raw::config_setting_add(&mut self.element.unwrap(),
                CString::new(name.into()).unwrap().as_ptr(),
                raw::CONFIG_TYPE_INT as i32)
        };
        
        if option.is_null() {
            None
        } else {
            let result = unsafe {
                raw::config_setting_set_int(option, value)  
            };
            
            if result == raw::CONFIG_TRUE {
                Some(*self)
            } else {
                None
            }
        }
    }
}

impl OptionReader {
    
    // Constructor
    fn new(elem : Option<raw::config_setting_t>) -> OptionReader {
        OptionReader {
            element : elem
        }
    }
    
    // Return true if element is root
    pub fn is_root(&self) -> Option<bool> {
        if self.element.is_none() {
            return None;
        }
        
        let result = raw::config_setting_is_root(&self.element.unwrap());
        Some(result == raw::CONFIG_TRUE)
    }
     
    // Return true if element is section group
    pub fn is_section(&self) -> Option<bool> {
        if self.element.is_none() {
            return None;
        }
        
        let result = raw::config_setting_is_group(&self.element.unwrap());
        Some(result == raw::CONFIG_TRUE)      
    }
    
    // Return true if element is array
    pub fn is_array(&self) -> Option<bool> {
        if self.element.is_none() {
            return None
        }
        
        let result = raw::config_setting_is_array(&self.element.unwrap());
        Some(result == raw::CONFIG_TRUE)
    }
    
    // Return true if element is list
    pub fn is_list(&self) -> Option<bool> {
        if self.element.is_none() {
            return None
        }
        
        let result = raw::config_setting_is_list(&self.element.unwrap());
        Some(result == raw::CONFIG_TRUE)
    }
    
    // Return option element parent item
    pub fn parent(&self) -> Option<OptionReader> {
        if self.element.is_none() {
            return None
        }
        
        let result = raw::config_setting_parent(&self.element.unwrap());
        
        if result.is_null() {
            None
        } else {
            Some(OptionReader::new(Some(unsafe {*result})))
        }
    }
    
    // Return option value type
    pub fn value_type(&self) -> Option<OptionType> {
        if self.element.is_none() {
            return None
        }
        
        let result = raw::config_setting_type(&self.element.unwrap());
        match result as i16 {
            raw::CONFIG_TYPE_INT => { Some(OptionType::IntegerType) },
            raw::CONFIG_TYPE_INT64 => { Some(OptionType::Int64Type) },
            raw::CONFIG_TYPE_FLOAT => { Some(OptionType::FloatType) },
            raw::CONFIG_TYPE_STRING => { Some(OptionType::StringType) },
            raw::CONFIG_TYPE_BOOL => { Some(OptionType::BooleanType) },
            _ => { None }
        }
    }
    
    // Read value from path
    pub fn value<S>(&mut self, path : S) -> Option<OptionReader>
        where S: Into<String> {
        
        if self.element.is_none() {
            return None
        }
        
        let option = unsafe { raw::config_setting_lookup(
            &mut self.element.unwrap(), CString::new(path.into())
                .unwrap().as_ptr())
        };
         
        if option.is_null() {
            None          
        } else {
            Some(OptionReader::new(Some(unsafe {*option})))
        }  
    }
    
    // Present option value as i32
    pub fn as_integer(&mut self) -> Option<i32> {
        if self.element.is_none() {
            return None
        }
        
        let result = unsafe { 
            raw::config_setting_get_int(&mut self.element.unwrap()) 
        };
        Some(result)
    }
    
    // Present option value as i32, return def if value not found
    pub fn as_integer_default (&mut self, def : i32) -> i32 {
        match self.as_integer() {
            Some(x) => { x },
            None => { def }
        }
    }
    
    // Present option value as i64
    pub fn as_int64(&mut self) -> Option<i64> {
        if self.element.is_none() {
            return None
        }
        
        let result = unsafe {
            raw::config_setting_get_int64(&mut self.element.unwrap())
        };
        Some(result)
    }
    
    // Present option value as i64, return def if value not exists
    pub fn as_int64_default(&mut self, def : i64) -> i64 {
        match self.as_int64() {
            Some(x) => { x },
            None => { def }
        }
    }
    
    // Present option value as f64
    pub fn as_float(&mut self) -> Option<f64> {
        if self.element.is_none() {
            return None
        }
        
        let result = unsafe {
            raw::config_setting_get_float(&mut self.element.unwrap())
        };
        Some(result)
    }
    
    // Present option value as f64, return def if value not exists
    pub fn as_float_default(&mut self, def : f64) -> f64 {
        match self.as_float() {
            Some(x) => { x },
            None => { def }
        }
    }
    
    // Present option value as bool
    pub fn as_bool(&mut self) -> Option<bool> {
        if self.element.is_none() {
            return None
        }
        
        let result = unsafe {
            raw::config_setting_get_bool(&mut self.element.unwrap())
        };
        Some(result == raw::CONFIG_TRUE)
    }
    
    // Present option value as bool, return def if value not exists
    pub fn as_bool_default(&mut self, def : bool) -> bool {
        match self.as_bool() {
            Some(x) => { x },
            None => { def }
        }
    }
    
    // Present option value as string
    pub fn as_string(&mut self) -> Option<String> {
        if self.element.is_none() {
            return None
        }
        
        let result = unsafe {
            CStr::from_ptr(raw::config_setting_get_string(
                &mut self.element.unwrap()))
        };
        Some(result.to_str().unwrap().to_string())
    }
    
    // Present option value as string, return def if value not exists
    pub fn as_string_default(&mut self, def : String) -> String {
        match self.as_string() {
            Some(x) => { x },
            None => { def } 
        }
    }
}