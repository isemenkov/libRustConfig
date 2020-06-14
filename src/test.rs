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

use crate::config::{Config, OptionType};

macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) { panic!(); }
    }
}

#[test]
fn test_parse_config_string() {
    let mut cfg = Config::new();
    assert_eq!(cfg.load_from_string(
        "section1 : { 
            integer_value = -12; 
            boolean_value = false;
            long_integer_value = 99991L;
            float_value = 0.99991;
            string_value = \"test string\";
        };"
    ).is_ok(), true);
    
    assert!(cfg.value("section1").unwrap().is_root().unwrap());
    assert!(cfg.value("section1").unwrap().is_section().unwrap());
    
    assert_eq!(cfg.value("section1.integer_value").unwrap()
        .value_type().unwrap(), OptionType::IntegerType);
    assert_eq!(cfg.value("section1.integer_value").unwrap()
        .as_integer().unwrap(), -12);
    
    assert_eq!(cfg.value("section1.boolean_value").unwrap()
        .value_type().unwrap(), OptionType::BooleanType);
    assert_eq!(cfg.value("section1.boolean_value").unwrap() 
        .as_bool().unwrap(), false);
    
    assert_eq!(cfg.value("section1.long_integer_value").unwrap()
        .value_type().unwrap(), OptionType::Int64Type);
    assert_eq!(cfg.value("section1.long_integer_value").unwrap() 
        .as_int64().unwrap(), 99991);
     
    assert_eq!(cfg.value("section1.float_value").unwrap() 
        .value_type().unwrap(), OptionType::FloatType);
    assert_delta!(cfg.value("section1.float_value").unwrap() 
        .as_float().unwrap(), 0.99991, 0.00001);
        
    assert_eq!(cfg.value("section1.string_value").unwrap() 
        .value_type().unwrap(), OptionType::StringType);
    assert_eq!(cfg.value("section1.string_value").unwrap() 
        .as_string().unwrap(), "test string");
}

#[test]
fn test_create_section() {
    let mut cfg = Config::new();
    assert_eq!(cfg.create_section("root_section").is_some(), true);
    
    assert_eq!(cfg.value("root_section").is_some(), true);
    assert_eq!(cfg.value("root_section").unwrap().is_root().unwrap(), true);
    assert_eq!(cfg.value("root_section").unwrap().is_section().unwrap(), true);
}
