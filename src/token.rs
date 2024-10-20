use pyo3::prelude::*;

#[pyclass(name = "Token")]
pub struct PyToken {
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub byte_start: usize,
    #[pyo3(get)]
    pub byte_end: usize,
    #[pyo3(get)]
    pub position: usize,
    #[pyo3(get)]
    pub position_length: usize,
    #[pyo3(get)]
    pub details: Vec<String>,
}
