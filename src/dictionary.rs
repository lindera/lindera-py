use std::path::Path;
use std::str::FromStr;

use pyo3::{exceptions::PyValueError, prelude::*};

use lindera::dictionary::{
    Dictionary, DictionaryBuilder, DictionaryKind, Metadata, UserDictionary,
};

#[pyclass(name = "Dictionary")]
#[derive(Clone)]
pub struct PyDictionary {
    pub inner: Dictionary,
}

#[pymethods]
impl PyDictionary {
    pub fn metadata_name(&self) -> String {
        self.inner.metadata.name.clone()
    }

    pub fn metadata_encoding(&self) -> String {
        self.inner.metadata.encoding.clone()
    }

    fn __str__(&self) -> String {
        "Dictionary".to_string()
    }

    fn __repr__(&self) -> String {
        format!("Dictionary()")
    }
}

impl PyDictionary {
    // Internal helper function to create PyDictionary from Lindera Dictionary
    pub fn new(dictionary: Dictionary) -> Self {
        Self { inner: dictionary }
    }
}

#[pyclass(name = "UserDictionary")]
#[derive(Clone)]
pub struct PyUserDictionary {
    pub inner: UserDictionary,
}

#[pymethods]
impl PyUserDictionary {
    fn __str__(&self) -> String {
        "UserDictionary".to_string()
    }

    fn __repr__(&self) -> String {
        "UserDictionary()".to_string()
    }
}

impl PyUserDictionary {
    // Internal helper function to create PyUserDictionary from Lindera UserDictionary
    pub fn new(user_dictionary: UserDictionary) -> Self {
        Self {
            inner: user_dictionary,
        }
    }
}

#[pyfunction]
#[pyo3(signature = (_kind, input_dir, output_dir, metadata=None))]
pub fn build_dictionary(
    _kind: &str,
    input_dir: &str,
    output_dir: &str,
    metadata: Option<crate::metadata::PyMetadata>,
) -> PyResult<()> {
    let input_path = Path::new(input_dir);
    let output_path = Path::new(output_dir);

    if !input_path.exists() {
        return Err(PyValueError::new_err(format!(
            "Input directory does not exist: {}",
            input_dir
        )));
    }

    // Use provided metadata or create default
    let meta = match metadata {
        Some(py_metadata) => {
            // Convert PyMetadata to Lindera Metadata
            let lindera_meta: Metadata = py_metadata.into();
            lindera_meta
        }
        None => Metadata::default(),
    };

    let builder = DictionaryBuilder::new(meta);

    builder
        .build_dictionary(input_path, output_path)
        .map_err(|e| PyValueError::new_err(format!("Failed to build dictionary: {}", e)))?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (_kind, input_file, output_dir, metadata=None))]
pub fn build_user_dictionary(
    _kind: &str,
    input_file: &str,
    output_dir: &str,
    metadata: Option<crate::metadata::PyMetadata>,
) -> PyResult<()> {
    let input_path = Path::new(input_file);
    let output_path = Path::new(output_dir);

    if !input_path.exists() {
        return Err(PyValueError::new_err(format!(
            "Input file does not exist: {}",
            input_file
        )));
    }

    // Use provided metadata or create default
    let meta = match metadata {
        Some(py_metadata) => {
            let lindera_meta: Metadata = py_metadata.into();
            lindera_meta
        }
        None => Metadata::default(),
    };

    let builder = DictionaryBuilder::new(meta);

    // Build user dictionary from CSV
    builder
        .build_user_dictionary(input_path, output_path)
        .map_err(|e| PyValueError::new_err(format!("Failed to build user dictionary: {}", e)))?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (kind=None, path=None))]
pub fn load_dictionary(kind: Option<&str>, path: Option<&str>) -> PyResult<PyDictionary> {
    match (kind, path) {
        // Load embedded dictionary by kind
        (Some(kind_str), None) => {
            let dict_kind = DictionaryKind::from_str(kind_str).map_err(|e| {
                PyValueError::new_err(format!("Invalid dictionary kind '{}': {}", kind_str, e))
            })?;

            let dictionary =
                lindera::dictionary::load_embedded_dictionary(dict_kind).map_err(|e| {
                    PyValueError::new_err(format!("Failed to load embedded dictionary: {}", e))
                })?;

            Ok(PyDictionary::new(dictionary))
        }

        // Load dictionary from file path
        (_, Some(path_str)) => {
            let dictionary = lindera::dictionary::load_dictionary(path_str).map_err(|e| {
                PyValueError::new_err(format!(
                    "Failed to load dictionary from '{}': {}",
                    path_str, e
                ))
            })?;

            Ok(PyDictionary::new(dictionary))
        }

        // Load default embedded dictionary (IPADIC)
        (None, None) => {
            #[cfg(feature = "embedded-ipadic")]
            {
                let dictionary = lindera::dictionary::load_embedded_dictionary(
                    DictionaryKind::IPADIC,
                )
                .map_err(|e| {
                    PyValueError::new_err(format!("Failed to load default dictionary: {}", e))
                })?;
                Ok(PyDictionary::new(dictionary))
            }

            #[cfg(not(feature = "embedded-ipadic"))]
            {
                Err(PyValueError::new_err(
                    "No dictionary kind or path specified, and no default embedded dictionary available",
                ))
            }
        }
    }
}

#[pyfunction]
#[pyo3(signature = (path, metadata=None))]
pub fn load_user_dictionary(
    path: &str,
    metadata: Option<crate::metadata::PyMetadata>,
) -> PyResult<PyUserDictionary> {
    let user_dict_path = Path::new(path);

    if !user_dict_path.exists() {
        return Err(PyValueError::new_err(format!(
            "User dictionary file does not exist: {}",
            path
        )));
    }

    // Use provided metadata or create default
    let meta = match metadata {
        Some(py_metadata) => {
            let lindera_meta: Metadata = py_metadata.into();
            lindera_meta
        }
        None => Metadata::default(),
    };

    let user_dictionary = if path.ends_with(".csv") {
        // Load from CSV file
        lindera::dictionary::load_user_dictionary_from_csv(&meta, user_dict_path).map_err(|e| {
            PyValueError::new_err(format!("Failed to load user dictionary from CSV: {}", e))
        })?
    } else {
        // Load from binary file
        lindera::dictionary::load_user_dictionary_from_bin(user_dict_path).map_err(|e| {
            PyValueError::new_err(format!("Failed to load user dictionary from binary: {}", e))
        })?
    };

    Ok(PyUserDictionary::new(user_dictionary))
}
