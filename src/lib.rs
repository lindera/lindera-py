use std::fs::File;
use std::io::{BufReader, Read};

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use lindera::analyzer::Analyzer;
use lindera::FilteredToken;

#[pyclass(name = "Analyzer")]
struct PyAnalyzer {
    inner: Analyzer,
}

#[pymethods]
impl PyAnalyzer {
    #[new]
    fn new(config_path: Option<&str>) -> PyResult<Self> {
        let mut config_bytes = vec![];
        if let Some(config_path) = config_path {
            let mut config_file = BufReader::new(File::open(config_path)?);
            config_file.read_to_end(&mut config_bytes)?;
        }

        Ok(Self {
            inner: Analyzer::from_slice(&config_bytes).unwrap(),
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

#[pyclass(name = "Token")]
struct PyToken {
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    details: Vec<String>,
}

impl From<FilteredToken> for PyToken {
    fn from(token: FilteredToken) -> Self {
        PyToken {
            text: token.text.to_string(),
            details: token.details,
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn lindera_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyAnalyzer>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
