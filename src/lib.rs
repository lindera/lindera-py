pub mod analyzer;
pub mod tokenizer;

use std::{path::PathBuf, str::FromStr};

use analyzer::PyAnalyzer;
use pyo3::{exceptions::PyValueError, prelude::*};

use lindera::{
    dictionary::{DictionaryConfig, UserDictionaryConfig},
    DictionaryKind, FilteredToken,
};
use tokenizer::{PyTokenizer, PyTokenizerConfig};

#[derive(Clone)]
#[pyclass(name = "DictionaryConfig")]
struct PyDictionaryConfig {
    inner: DictionaryConfig,
}

#[pymethods]
impl PyDictionaryConfig {
    #[new]
    fn new(kind: Option<&str>, path: Option<&str>) -> PyResult<Self> {
        let k = match kind {
            Some(kind_str) => Some(
                DictionaryKind::from_str(kind_str)
                    .map_err(|_err| PyValueError::new_err("Invalid kind"))?,
            ),
            None => None,
        };
        let p = match path {
            Some(path_str) => Some(PathBuf::from(path_str)),
            None => None,
        };

        Ok(Self {
            inner: DictionaryConfig { kind: k, path: p },
        })
    }
}

#[derive(Clone)]
#[pyclass(name = "UserDictionaryConfig")]
struct PyUserDictionaryConfig {
    inner: UserDictionaryConfig,
}

#[pymethods]
impl PyUserDictionaryConfig {
    #[new]
    fn new(path: &str, kind: Option<&str>) -> PyResult<Self> {
        let p = PathBuf::from(path);
        let k = match kind {
            Some(kind_str) => Some(
                DictionaryKind::from_str(kind_str)
                    .map_err(|_err| PyValueError::new_err("Invalid kind"))?,
            ),
            None => None,
        };

        Ok(Self {
            inner: UserDictionaryConfig { path: p, kind: k },
        })
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
    m.add_class::<PyTokenizer>()?;
    m.add_class::<PyDictionaryConfig>()?;
    m.add_class::<PyUserDictionaryConfig>()?;
    m.add_class::<PyTokenizerConfig>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
