# libRustConfig

It is rust bindings and wrapper around [libconfig](https://github.com/hyperrealm/libconfig) library. Library for processing configuration files. 

### Bindings

[libconfig-sys](https://github.com/isemenkov/librustconfig/tree/master/libconfig-sys) crate contains the libconfig translated headers to use this library in Rust programs.

### Wrapper

[libconfig](https://github.com/isemenkov/librustconfig/tree/master/src) crate contains the libconfig safe wrapper.

#### Usage example

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

if !cfg.value("section1").unwrap().is_section().unwrap() {
    panic!("Value must be a group!");
}

let _int_val = cfg.value("section1.integer_Value").unwrap().as_int32();
if int_val.is_none() {
    panic!("Can't read integer_value from configuration");
}

let _bool_val = cfg.value("section1.boolean_value").unwrap().as_bool_default(false);

match cfg.value("section1.int64_value").unwrap().value_type().unwrap() {
    OptionType::Int64Type => { /* ... do something ... */ }
    _ => { /* ... do nothing ... */ }
}

let group = cfg.create_section("group");
if group.is_none() {
    panic!("Can't create new group section!");
}

if group.unwrap().write_string("value", "string value").is_none() {
    panic!("Can't write string value!");
}

if cfg.save_to_file(Path::new("config.cfg")).is_err() {
    panic!("Can't save configuration to file!");
}
```







