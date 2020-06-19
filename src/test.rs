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
use std::path::Path;
use std::fs;

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
        .as_int32().unwrap(), -12);
    
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
        .as_float64().unwrap(), 0.99991, 0.00001);
        
    assert_eq!(cfg.value("section1.string_value").unwrap()
        .value_type().unwrap(), OptionType::StringType);
    assert_eq!(cfg.value("section1.string_value").unwrap()
        .as_string().unwrap(), "test string");
}

#[test]
fn test_create_section() {
    let mut cfg = Config::new();
    let root = cfg.create_section("root_section").unwrap();
    let group = root.create_section("group").unwrap();
    let mut _val_i32 = group.write_int32("test", 123);
    let mut _val_i64 = group.write_int64("test2", 100000002);
    let mut _val_f64 = group.write_float64("test3", 1.00023);
    let mut _val_bool = group.write_bool("test4", true);
    let mut _val_str = group.write_string("test5", "string string");
    
    assert_eq!(cfg.save_to_file(Path::new("test.cfg")).is_ok(), true);
    assert_eq!(Path::new("test.cfg").exists(), true);
    
    assert_eq!(cfg.load_from_file(Path::new("test.cfg")).is_ok(), true);
    assert_eq!(cfg.value("root_section").is_some(), true);
    assert_eq!(cfg.value("root_section").unwrap().is_section().unwrap(), true);
    assert_eq!(cfg.value("root_section.group").is_some(), true);
    assert_eq!(cfg.value("root_section.group").unwrap().is_root().unwrap(), 
        false);
    assert_eq!(cfg.value("root_section.group").unwrap().is_section().unwrap(), 
        true);
    assert_eq!(cfg.value("root_section.group.test").unwrap()
        .as_int32().unwrap(), 123);
    assert_eq!(cfg.value("root_section.group.test2").unwrap()
        .as_int64().unwrap(), 100000002);
    assert_delta!(cfg.value("root_section.group.test3").unwrap()
        .as_float64().unwrap(), 1.00023, 0.00001);
    assert_eq!(cfg.value("root_section.group.test4").unwrap()
        .as_bool().unwrap(), true);
    assert_eq!(cfg.value("root_section.group.test5").unwrap()
        .as_string().unwrap(), "string string");

    assert_eq!(fs::remove_file(Path::new("test.cfg")).is_ok(), true);
}
