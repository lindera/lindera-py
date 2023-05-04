use std::path::Path;

use pyo3::{exceptions::PyValueError, prelude::*};

use lindera_analyzer::analyzer::Analyzer;

use crate::PyToken;

#[pyclass(name = "Analyzer")]
pub struct PyAnalyzer {
    inner: Analyzer,
}

#[pymethods]
impl PyAnalyzer {
    #[new]
    fn new(config_path: &str) -> PyResult<Self> {
        Ok(Self {
            inner: Analyzer::from_file(Path::new(config_path)).unwrap(),
        })
    }

    fn analyze(&self, text: &str) -> PyResult<Vec<PyToken>> {
        let mut text = text.to_string();
        let tokens = self
            .inner
            .analyze(&mut text)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?
            .into_iter()
            .map(|x| PyToken::from(x))
            .collect();

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
