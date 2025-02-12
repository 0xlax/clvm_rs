use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use super::lazy_node::LazyNode;
use super::run_generator::{PySpend, PySpendBundleConditions, __pyo3_get_function_run_generator2};
use super::run_program::{
    __pyo3_get_function_deserialize_and_run_program2, __pyo3_get_function_run_chia_program,
    __pyo3_get_function_serialized_length,
};
use clvmr::chia_dialect::{NO_NEG_DIV, NO_UNKNOWN_OPS};
use clvmr::gen::flags::{COND_ARGS_NIL, COND_CANON_INTS, NO_UNKNOWN_CONDS};

pub const MEMPOOL_MODE: u32 =
    NO_NEG_DIV | COND_CANON_INTS | NO_UNKNOWN_CONDS | NO_UNKNOWN_OPS | COND_ARGS_NIL;

#[pymodule]
fn clvm_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(deserialize_and_run_program2, m)?)?;
    m.add_function(wrap_pyfunction!(run_generator2, m)?)?;
    m.add_function(wrap_pyfunction!(run_chia_program, m)?)?;
    m.add("NO_NEG_DIV", NO_NEG_DIV)?;
    m.add("COND_CANON_INTS", COND_CANON_INTS)?;
    m.add("COND_ARGS_NIL", COND_ARGS_NIL)?;
    m.add("NO_UNKNOWN_CONDS", NO_UNKNOWN_CONDS)?;
    m.add("NO_UNKNOWN_OPS", NO_UNKNOWN_OPS)?;
    m.add("MEMPOOL_MODE", MEMPOOL_MODE)?;
    m.add_class::<LazyNode>()?;
    m.add_class::<PySpendBundleConditions>()?;
    m.add_class::<PySpend>()?;

    m.add_function(wrap_pyfunction!(serialized_length, m)?)?;

    Ok(())
}
