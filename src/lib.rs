use pyo3::prelude::*;

mod vault;
mod crypto;

use vault::*;

#[pymodule]
fn vault_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vault>()?;
    m.add_class::<Data>()?;
    m.add_function(wrap_pyfunction!(load_vault, m)?)?;
    m.add_function(wrap_pyfunction!(save_vault, m)?)?;
    Ok(())
} 