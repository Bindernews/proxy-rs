extern crate hyper;
extern crate toml;
extern crate serde;
extern crate url;

mod config;

use std::ffi::{CStr};
use std::os::raw::c_char;



pub extern "C" fn init_proxy_rus(config_string: *const c_char) -> bool {
    unsafe {
        match CStr::from_ptr(config_string).to_str() {
            Ok(v) => {
                return init_proxy_internal(v);
            },
            Err(_) => {
                return false;
            },
        }
    }
}

fn init_proxy_internal(config_string: &str) -> bool {
    false
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
