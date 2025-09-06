use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::fmt;

#[pyclass(name = "LinderaError")]
#[derive(Debug, Clone)]
pub struct PyLinderaError {
    message: String,
}

#[pymethods]
impl PyLinderaError {
    #[new]
    pub fn new(message: String) -> Self {
        PyLinderaError { message }
    }

    #[getter]
    pub fn message(&self) -> &str {
        &self.message
    }

    fn __str__(&self) -> String {
        self.message.clone()
    }

    fn __repr__(&self) -> String {
        format!("LinderaError('{}')", self.message)
    }
}

impl fmt::Display for PyLinderaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PyLinderaError {}

impl From<PyLinderaError> for PyErr {
    fn from(err: PyLinderaError) -> PyErr {
        PyException::new_err(err.message)
    }
}
