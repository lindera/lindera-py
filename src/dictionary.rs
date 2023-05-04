use std::{path::PathBuf, str::FromStr};

use pyo3::{exceptions::PyValueError, prelude::*};

use lindera_dictionary::{
    build_dictionary as lindera_build_dictionary,
    build_user_dictionary as lindera_build_user_dictionary, DictionaryConfig, DictionaryKind,
    UserDictionaryConfig,
};

#[derive(Clone)]
#[pyclass(name = "DictionaryConfig")]
pub struct PyDictionaryConfig {
    pub inner: DictionaryConfig,
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
pub struct PyUserDictionaryConfig {
    pub inner: UserDictionaryConfig,
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

#[pyfunction]
pub fn build_dictionary(kind: &str, input_dir: &str, output_dir: &str) -> PyResult<()> {
    lindera_build_dictionary(
        DictionaryKind::from_str(kind).map_err(|_err| PyValueError::new_err("Invalid kind"))?,
        &PathBuf::from(input_dir),
        &PathBuf::from(output_dir),
    )
    .map_err(|_err| PyValueError::new_err("Failed to build dictionary"))
}

#[pyfunction]
pub fn build_user_dictionary(kind: &str, input_file: &str, output_dir: &str) -> PyResult<()> {
    lindera_build_user_dictionary(
        DictionaryKind::from_str(kind).map_err(|_err| PyValueError::new_err("Invalid kind"))?,
        &PathBuf::from(input_file),
        &PathBuf::from(output_dir),
    )
    .map_err(|_err| PyValueError::new_err("Failed to build user dictionary"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
