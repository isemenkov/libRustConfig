# libRustConfig

It is rust bindings and wrapper around [libconfig](https://github.com/hyperrealm/libconfig) library. Library for processing configuration files. 


### Table of contents

* [Requierements](#requirements)
* [Installation](#installation)
* [Usage](#usage)
* [Bindings](#bindings)
* [Wrapper](#wrapper)
* [Usage example](#usage-example)
  * [Create](#create)
  * [Insert](#insert)
  * [Insert group](#insert-group)
  * [Search](#search)
  * [Search default](#search-default)
  * [Iterate](#iterate)
  * [Save](#save)


### Requirements

* [Rust Compiler](https://www.rust-lang.org/)
* [Cargo package manager](https://www.rust-lang.org/)

Library is writing used latest stable Rust Compiler (rustc 1.46.0 (04488afe3 2020-08-24)).



### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
librustconfig = "0.1.*"
```



### Usage

Add to your crate root:

```rust
extern crate config;
```



### Bindings

[libconfig-sys](https://github.com/isemenkov/librustconfig/tree/master/libconfig-sys) crate contains the libconfig translated headers to use this library in Rust programs.



### Wrapper

[libconfig](https://github.com/isemenkov/librustconfig/tree/master/src) crate contains the libconfig safe wrapper.



#### Usage example

##### Create

```rust
use libconfig::config::{Config, OptionType};
use std::path::Path;

let mut cfg = Config::new();
if cfg.load_from_string(
	"section1 : {
		integer_value = -12;
    	boolean_value = true;
    	int64_value = 99999L;
    	float_value = 0.9999991;
    	string_value = \"test string value \";
    }";
).is_err() {
    panic!("Can\t load configuration from string value!");
}
```

##### Insert

```rust
let group = cfg.create_section("group");
if group.is_none() {
    panic!("Can't create new group section!");
}

if group.unwrap().write_string("value", "string value").is_none() {
    panic!("Can't write string value!");
}
```

##### Insert group

```rust
let array = group.create_array("array_list");
if array.is_none() {
    panic!("Can't create new array option group!");
}

if array.write_int32(12).is_none() {
    panic!("Can't write array element value!");
}
```

##### Search

```rust
if !cfg.value("section1").unwrap().is_section().unwrap() {
    panic!("Value must be a group!");
}

let _int_val = cfg.value("section1.integer_Value").unwrap().as_int32();
if int_val.is_none() {
    panic!("Can't read integer_value from configuration");
}

match cfg.value("section1.int64_value").unwrap().value_type().unwrap() {
    OptionType::Int64Type => { /* ... do something ... */ }
    _ => { /* ... do nothing ... */ }
}
```

##### Search default

```rust
let _bool_val = cfg.value("section1.boolean_value").unwrap().as_bool_default(false);
```

##### Iterate

```rust
for arr_val in cfg.value("group.array_list").unwrap().as_array() {
    if arr_val.as_int32().in_none() {
        panic!("Can't read array item value!");
    }
    /* ... do something with array item ... */
}
```

##### Save

```rust
if cfg.save_to_file(Path::new("config.cfg")).is_err() {
    panic!("Can't save configuration to file!");
}
```







