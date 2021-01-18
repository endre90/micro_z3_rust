use std::ffi::{CString};
use z3_sys::*;

pub fn new_context_z3(cfg: &Z3_config) -> Z3_context {
    unsafe {
        Z3_mk_context(*cfg)
    }
}

pub fn get_model_size_z3() -> u64 {
    unsafe {
        Z3_get_estimated_alloc_size()
    }
}

pub fn params_z3(ctx: &Z3_context) -> Z3_params {
    unsafe { 
        let params = Z3_mk_params(*ctx);
        Z3_params_inc_ref(*ctx, params);
        params
     }
}

pub fn add_bool_param_z3(ctx: &Z3_context, params: &Z3_params, name: &str, value: bool) -> () {
    unsafe { 
        let pname = CString::new(name).unwrap(); 
        let pname_symbol = Z3_mk_string_symbol(*ctx, pname.as_ptr());  
        Z3_params_set_bool(
            *ctx, 
            *params, 
            pname_symbol,
            value
        )
    }
}

pub fn add_uint_param_z3(ctx: &Z3_context, params: &Z3_params, name: &str, value: ::std::os::raw::c_uint) -> () {
    unsafe { 
        let pname = CString::new(name).unwrap(); 
        let pname_symbol = Z3_mk_string_symbol(*ctx, pname.as_ptr());  
        Z3_params_set_uint(
            *ctx, 
            *params, 
            pname_symbol,
            value
        )
    }
}