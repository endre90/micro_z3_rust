use std::ffi::{CString};
use z3_sys::*;

pub fn new_var_z3(ctx: &Z3_context, sort: &Z3_sort, name: &str) -> Z3_ast {
    unsafe {
        Z3_mk_const(
            *ctx, 
            Z3_mk_string_symbol(
                *ctx, 
                CString::new(name).unwrap().as_ptr()
            ),
            *sort
        )
    }
}
