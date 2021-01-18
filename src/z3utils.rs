use std::ffi::CStr;
use z3_sys::*;

pub fn new_fresh_model_z3(ctx: &Z3_context) -> Z3_model {
    unsafe {
        Z3_mk_model(*ctx)
    }
}

pub fn ast_to_string_z3(ctx: &Z3_context, what: &Z3_ast) -> String {
    unsafe {
        CStr::from_ptr(Z3_ast_to_string(*ctx, *what)).to_str().unwrap().to_owned()
    }
}

pub fn ast_vector_to_string_z3(ctx: &Z3_context, what: &Z3_ast_vector) -> String {
    unsafe {
        CStr::from_ptr(Z3_ast_vector_to_string(*ctx, *what)).to_str().unwrap().to_owned()
    }
}

pub fn model_to_string_z3(ctx: &Z3_context, what: &Z3_model) -> String {
    unsafe {
        CStr::from_ptr(Z3_model_to_string(*ctx, *what)).to_str().unwrap().to_owned()
    }
}

pub fn model_get_num_consts_z3(ctx: &Z3_context, what: &Z3_model) -> ::std::os::raw::c_uint {
    unsafe {
        Z3_model_get_num_consts(*ctx, *what)
    }
}

pub fn model_get_consts_decl_z3(ctx: &Z3_context, what: &Z3_model, index: ::std::os::raw::c_uint) -> Z3_func_decl {
    unsafe {
        Z3_model_get_const_decl(*ctx, *what, index)
    }
}

pub fn model_get_decl_name_z3(ctx: &Z3_context, decl: &Z3_func_decl) -> Z3_symbol {
    unsafe {
        Z3_get_decl_name(*ctx, *decl)
    }
}

pub fn model_get_const_interp_z3(ctx: &Z3_context, what: &Z3_model, decl: &Z3_func_decl) -> Z3_ast {
    unsafe {
        Z3_model_get_const_interp(*ctx, *what, *decl)
    }
}

pub fn get_symbol_string_z3(ctx: &Z3_context, symbol: &Z3_symbol) -> Z3_string {
    unsafe {
        Z3_get_symbol_string(*ctx, *symbol)
    }
}

pub fn z3_string_to_string_z3(cstr: Z3_string) -> String {
    unsafe {
        CStr::from_ptr(cstr).to_str().unwrap().to_owned()
    }
}

pub fn z3_ast_vector_to_vector_ast_z3(ctx: &Z3_context, ast_vec: &Z3_ast_vector) -> Vec<Z3_ast> {
    let mut vec: Vec<Z3_ast> = vec!();
        unsafe {
            let size = Z3_ast_vector_size(*ctx, *ast_vec);
            for i in 0..size {
                vec.push(Z3_ast_vector_get(*ctx, *ast_vec, i));
            }
        }
        vec
}

pub fn sort_to_ast_z3(ctx: &Z3_context, what: &Z3_sort) -> Z3_ast {
    unsafe {
        Z3_sort_to_ast(*ctx, *what)
    }
}