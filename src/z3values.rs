use z3_sys::*;

pub fn new_bool_value_z3(ctx: &Z3_context, val: bool) -> Z3_ast {
    unsafe {
        match val {
            true => Z3_mk_true(*ctx),
            false => Z3_mk_false(*ctx)
        }
    }
}