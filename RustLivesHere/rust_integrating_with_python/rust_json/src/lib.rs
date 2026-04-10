use pyo3::prelude::*;
use pyo3::types::PyModule;

#[derive(Debug, serde::Deserialize)]
struct Data {
	name: String,
	value: i32,
}

#[pyfunction]
fn sum(input: &str) -> PyResult<i32> {
	let parsed: Data = serde_json::from_str(input)
		.map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

	Ok(parsed.name.len() as i32 + parsed.value)

}

#[pymodule]
fn rust_json(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(sum, m)?)?;

	Ok(())
}
