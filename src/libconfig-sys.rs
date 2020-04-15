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

mod libconfig-sys;

extern crate libc;

use libc::{c_int};
use std::{ptr, mem};

pub const CONFIG_TYPE_NONE                                              = 0;
pub const CONFIG_TYPE_GROUP                                             = 1;
pub const CONFIG_TYPE_INT                                               = 2;
pub const CONFIG_TYPE_INT64                                             = 3;
pub const CONFIG_TYPE_FLOAT                                             = 4;
pub const CONFIG_TYPE_STRING                                            = 5;
pub const CONFIG_TYPE_BOOL                                              = 6;
pub const CONFIG_TYPE_ARRAY                                             = 7;
pub const CONFIG_TYPE_LIST                                              = 8;

pub const CONFIG_FORMAT_DEFAULT                                         = 0;
pub const CONFIG_FORMAT_HEX                                             = 1;

pub const CONFIG_OPTION_AUTOCONVERT                                     = 0x01;
pub const CONFIG_OPTION_SEMICOLON_SEPARATORS                            = 0x02;
pub const CONFIG_OPTION_COLON_ASSIGNMENT_FOR_GROUPS                     = 0x04;
pub const CONFIG_OPTION_COLON_ASSIGNMENT_FOR_NON_GROUPS                 = 0x08;
pub const CONFIG_OPTION_OPEN_BRACE_ON_SEPARATE_LINE                     = 0x10;

pub const CONFIG_TRUE                                                   = 1;
pub const CONFIG_FALSE                                                  = 0;

