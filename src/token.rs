use pyo3::prelude::*;

// Simplified Token representation for Python
#[pyclass(name = "Token")]
#[derive(Clone)]
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

#[pymethods]
impl PyToken {
    #[new]
    pub fn new(
        text: String,
        byte_start: usize,
        byte_end: usize,
        position: usize,
        position_length: usize,
        details: Vec<String>,
    ) -> Self {
        PyToken {
            text,
            byte_start,
            byte_end,
            position,
            position_length,
            details,
        }
    }

    fn __str__(&self) -> String {
        format!("Token(text='{}', pos={})", self.text, self.position)
    }

    fn __repr__(&self) -> String {
        format!(
            "Token(text='{}', byte_start={}, byte_end={}, position={}, details={:?})",
            self.text, self.byte_start, self.byte_end, self.position, self.details
        )
    }
}
