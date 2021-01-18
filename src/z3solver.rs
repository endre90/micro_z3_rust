use std::ffi::{CStr, CString};
use z3_sys::*;

pub fn new_solver_z3(ctx: &Z3_context) -> Z3_solver {
    unsafe {
        let slv = Z3_mk_solver(*ctx);
        Z3_solver_inc_ref(*ctx, slv);
        slv
    }
}

pub fn new_solver_for_logic_z3(ctx: &Z3_context, logic: &str) -> Z3_solver {
    unsafe {
        let logic_name = CString::new(logic).unwrap(); 
        let logic_name_symbol = Z3_mk_string_symbol(*ctx, logic_name.as_ptr());  
        let slv = Z3_mk_solver_for_logic(*ctx, logic_name_symbol);
        Z3_solver_inc_ref(*ctx, slv);
        slv
    }
}

pub fn solver_set_params_z3(ctx: &Z3_context, slv: &Z3_solver, params: &Z3_params) -> () {
    unsafe {
        Z3_solver_set_params(*ctx, *slv, *params)
    }
}

pub fn solver_assert_z3(ctx: &Z3_context, slv: &Z3_solver, cst: &Z3_ast) -> () {
    unsafe {
        Z3_solver_assert(*ctx, *slv, *cst)
    }
}

pub fn solver_push_z3(ctx: &Z3_context, slv: &Z3_solver) -> () {
    unsafe {
        Z3_solver_push(*ctx, *slv)
    }
}

pub fn solver_pop_z3(ctx: &Z3_context, slv: &Z3_solver, p: u32) -> () {
    unsafe {
        Z3_solver_pop(*ctx, *slv, p)
    }
}

pub fn solver_check_z3(ctx: &Z3_context, slv: &Z3_solver) -> Z3_lbool {
    unsafe {
        Z3_solver_check(*ctx, *slv)
    }
}

pub fn solver_get_model_z3(ctx: &Z3_context, slv: &Z3_solver) -> Z3_model {
    unsafe {
        Z3_solver_get_model(*ctx, *slv)
    }
}

pub fn solver_get_assertions_z3(ctx: &Z3_context, slv: &Z3_solver) -> Z3_ast_vector {
    unsafe {
        Z3_solver_get_assertions(*ctx, *slv)
    }
}

pub fn solver_to_string_z3(ctx: &Z3_context, slv: &Z3_solver) -> String {
    unsafe {
        CStr::from_ptr(Z3_solver_to_string(*ctx, *slv)).to_str().unwrap().to_owned()
    }
}

#[test]
fn solver_test_1() {
    use super::*;
    let cfg = new_config_z3();
    let ctx = new_context_z3(&cfg);
    // let slv = new_solver_z3(&ctx);
    let slv = new_solver_for_logic_z3(&ctx, "QF_FD");

    let xy_domain = vec!("a", "b", "c");
    let xy_sort = new_enum_sort_z3(&ctx, "xy_sort", &xy_domain);
    let xy_elems = &xy_sort.1;
    let a_index = xy_domain.iter().position(|r| r == &"a").unwrap_or_default();
    let b_index = xy_domain.iter().position(|r| r == &"b").unwrap_or_default();
    let a = xy_elems[a_index];
    let b = xy_elems[b_index];

    let bool_sort = new_bool_sort_z3(&ctx);

    let x = new_var_z3(&ctx, &xy_sort.0, "x");
    let y = new_var_z3(&ctx, &xy_sort.0, "y");
    let z = new_var_z3(&ctx, &bool_sort, "z");

    let x_a = eq_z3(&ctx, &x, &a);
    let not_y_b = neq_z3(&ctx, &y, &b);

    let and1 = and_z3(&ctx, &vec!(x_a, not_y_b, z));

    solver_assert_z3(&ctx, &slv ,&and1);

    solver_check_z3(&ctx, &slv);
    let model = solver_get_model_z3(&ctx, &slv);
    let model_string = model_to_string_z3(&ctx, &model);

    println!("{:?}", model_string);
}