use std::ffi::{CString};
use z3_sys::*;

pub fn new_config_z3() -> Z3_config {
    unsafe {Z3_mk_config()}
}

pub fn set_param_z3(cfg: &Z3_config, param: &str, value: &str) -> () {
    unsafe {
        Z3_set_param_value(
            *cfg, 
            CString::new(param).unwrap().as_ptr(), 
            CString::new(value).unwrap().as_ptr()
        )
    }
}

pub fn global_param_set_z3(param: &str, value: &str) -> () {
    unsafe {
        Z3_global_param_set(
            CString::new(param).unwrap().as_ptr(), 
            CString::new(value).unwrap().as_ptr()
        )
    }
}

pub fn global_param_reset_z3() -> () {
    unsafe {
        Z3_global_param_reset_all()
    }
}