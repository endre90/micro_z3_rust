use std::ffi::{CStr};
use z3_sys::*;

pub fn stats(ctx: &Z3_context, slv: &Z3_solver) -> String {
    unsafe {
        let stats = Z3_solver_get_statistics(*ctx, *slv);
        Z3_stats_inc_ref(*ctx, stats);
        CStr::from_ptr(Z3_stats_to_string(*ctx, stats.to_owned()))
            .to_str()
            .unwrap()
            .to_owned()
    }
}
