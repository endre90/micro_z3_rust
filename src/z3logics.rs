use z3_sys::*;

pub fn and_z3(ctx: &Z3_context, args: &Vec<Z3_ast>) -> Z3_ast {
    unsafe {
        Z3_mk_and(*ctx, args.len() as u32, args.as_ptr())
    }
}

pub fn or_z3(ctx: &Z3_context, args: &Vec<Z3_ast>) -> Z3_ast {
    unsafe {
        Z3_mk_or(*ctx, args.len() as u32, args.as_ptr())
    }
}

pub fn disctinct_z3(ctx: &Z3_context, args: &Vec<Z3_ast>) -> Z3_ast {
    unsafe {
        Z3_mk_distinct(*ctx, args.len() as u32, args.as_ptr())
    }
}

pub fn not_z3(ctx: &Z3_context, arg: Z3_ast) -> Z3_ast {
    unsafe {
        Z3_mk_not(*ctx, arg)
    }
}

pub fn pbeq_z3(ctx: &Z3_context, args: &Vec<Z3_ast>, sum: i32) -> Z3_ast {
    unsafe {
        Z3_mk_pbeq(*ctx, args.len() as u32, args.as_ptr(), vec![1; args.len()].as_ptr(), sum)
    }
}