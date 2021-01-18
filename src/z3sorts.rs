use std::ffi::{CStr, CString};
use z3_sys::*;

pub fn sort_to_string_z3(ctx: &Z3_context, what: &Z3_sort) -> String {
    unsafe {
        CStr::from_ptr(Z3_sort_to_string(*ctx, *what)).to_str().unwrap().to_owned()
    }
}

pub fn new_bool_sort_z3(ctx: &Z3_context) -> Z3_sort {
    unsafe { 
        Z3_mk_bool_sort(*ctx)
    }
}

pub fn new_enum_sort_z3(ctx: &Z3_context, name: &str, elems: &Vec<&str>) -> (Z3_sort, Vec<Z3_ast>) {
    unsafe { 
        let enum_name_symbol = Z3_mk_string_symbol(*ctx, CString::new(name).unwrap().as_ptr());  

        let mut enum_names: Vec<Z3_symbol> = vec![std::ptr::null_mut(); elems.len()];
        let mut enum_consts: Vec<Z3_func_decl> = vec![std::ptr::null_mut(); elems.len()];
        let mut enum_testers: Vec<Z3_func_decl> = vec![std::ptr::null_mut(); elems.len()];
        let mut enum_asts: Vec<Z3_ast> = vec![std::ptr::null_mut(); elems.len()];

        for s in elems {
            let index = elems.iter().position(|&r| r == *s).unwrap();
            enum_names[index] = Z3_mk_string_symbol(*ctx, CString::new(*s).unwrap().as_ptr());
        }

        let enum1 = Z3_mk_enumeration_sort(*ctx, enum_name_symbol, enum_names.len() as u32, enum_names.as_ptr(), enum_consts.as_mut_ptr(), enum_testers.as_mut_ptr());
         
        for c in &enum_names {
            let index = enum_names.iter().position(|&r| r == *c).unwrap();
            enum_asts[index] = Z3_mk_app(*ctx, enum_consts[index], 0, [].as_ptr());
        }

        (enum1, enum_asts)
    }
}