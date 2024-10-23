use std::path::PathBuf;
use std::str::FromStr;

use pyo3::{exceptions::PyValueError, prelude::*};

use lindera::dictionary::{
    load_dictionary_from_kind, load_dictionary_from_path, load_user_dictionary_from_bin,
    load_user_dictionary_from_csv, Dictionary, DictionaryKind, UserDictionary,
};
#[pyclass(name = "Dictionary")]
#[derive(Clone)]
pub struct PyDictionary {
    pub inner: Dictionary,
}

#[pyclass(name = "UserDictionary")]
#[derive(Clone)]
pub struct PyUserDictionary {
    pub inner: UserDictionary,
}

#[pyfunction]
#[pyo3(signature = (kind=None, path=None))]
pub fn load_dictionary(kind: Option<&str>, path: Option<&str>) -> PyResult<PyDictionary> {
    match (kind, path) {
        (Some(kind_str), None) => {
            let k = DictionaryKind::from_str(kind_str)
                .map_err(|_err| PyValueError::new_err("Invalid kind"))?;
            let dictionary = load_dictionary_from_kind(k).map_err(|err| {
                PyValueError::new_err(format!("Failed to load dictionary: {}", err))
            })?;

            Ok(PyDictionary { inner: dictionary })
        }
        (None, Some(path_str)) => {
            let p = PathBuf::from(path_str);
            let dictionary = load_dictionary_from_path(p.as_path()).map_err(|err| {
                PyValueError::new_err(format!("Failed to load dictionary: {}", err))
            })?;

            Ok(PyDictionary { inner: dictionary })
        }
        _ => Err(PyValueError::new_err("Invalid arguments")),
    }
}

#[pyfunction]
#[pyo3(signature = (path, kind=None))]
pub fn load_user_dictionary(path: &str, kind: Option<&str>) -> PyResult<PyUserDictionary> {
    let p = PathBuf::from(path);
    let ext = p
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| PyValueError::new_err("Invalid file path"))?;
    match ext {
        "csv" => match kind {
            Some(kind) => {
                let k = DictionaryKind::from_str(kind)
                    .map_err(|_err| PyValueError::new_err("Invalid kind"))?;
                let user_dictionary = load_user_dictionary_from_csv(k, p).map_err(|err| {
                    PyValueError::new_err(format!("Failed to load user dictionary: {}", err))
                })?;

                Ok(PyUserDictionary {
                    inner: user_dictionary,
                })
            }
            None => Err(PyValueError::new_err(
                "Dictionary type must be specified if CSV file specified",
            )),
        },
        "bin" => match kind {
            Some(_kind) => Err(PyValueError::new_err(
                "Dictionary type must be None if Binaly file specified",
            )),
            None => {
                let user_dictionary = load_user_dictionary_from_bin(p).map_err(|err| {
                    PyValueError::new_err(format!("Failed to load user dictionary: {}", err))
                })?;

                Ok(PyUserDictionary {
                    inner: user_dictionary,
                })
            }
        },
        _ => Err(PyValueError::new_err(format!(
            "Unsupported file: path:{}, kind:{:?}",
            path, kind
        ))),
    }
}
