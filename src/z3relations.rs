use z3_sys::*;

pub fn eq_z3(ctx: &Z3_context, left: &Z3_ast, right: &Z3_ast) -> Z3_ast {
    unsafe {
        Z3_mk_eq(*ctx, *left, *right)
    }
}

pub fn neq_z3(ctx: &Z3_context, left: &Z3_ast, right: &Z3_ast) -> Z3_ast {
    unsafe {
        Z3_mk_not(*ctx, Z3_mk_eq(*ctx, *left, *right)) 
    }
}