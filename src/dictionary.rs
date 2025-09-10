use std::path::Path;

use pyo3::{exceptions::PyValueError, prelude::*};

use lindera::dictionary::{
    Dictionary, DictionaryBuilder, Metadata, UserDictionary,
    load_dictionary as lindera_load_dictionary,
    load_user_dictionary as lindera_load_user_dictionary,
};

use crate::metadata::PyMetadata;

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

    pub fn metadata(&self) -> PyMetadata {
        PyMetadata::from(self.inner.metadata.clone())
    }

    fn __str__(&self) -> String {
        "Dictionary".to_string()
    }

    fn __repr__(&self) -> String {
        "Dictionary()".to_string()
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
#[pyo3(signature = (input_dir, output_dir, metadata))]
pub fn build_dictionary(input_dir: &str, output_dir: &str, metadata: PyMetadata) -> PyResult<()> {
    let input_path = Path::new(input_dir);
    let output_path = Path::new(output_dir);

    if !input_path.exists() {
        return Err(PyValueError::new_err(format!(
            "Input directory does not exist: {input_dir}"
        )));
    }

    let builder = DictionaryBuilder::new(metadata.into());

    builder
        .build_dictionary(input_path, output_path)
        .map_err(|e| PyValueError::new_err(format!("Failed to build dictionary: {e}")))?;

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
            "Input file does not exist: {input_file}"
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
        .map_err(|e| PyValueError::new_err(format!("Failed to build user dictionary: {e}")))?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (uri))]
pub fn load_dictionary(uri: &str) -> PyResult<PyDictionary> {
    lindera_load_dictionary(uri)
        .map_err(|e| PyValueError::new_err(format!("Failed to load dictionary from '{uri}': {e}")))
        .map(PyDictionary::new)
}

#[pyfunction]
#[pyo3(signature = (uri, metadata))]
pub fn load_user_dictionary(uri: &str, metadata: PyMetadata) -> PyResult<PyUserDictionary> {
    let meta: Metadata = metadata.into();
    lindera_load_user_dictionary(uri, &meta)
        .map_err(|e| {
            PyValueError::new_err(format!("Failed to load user dictionary from '{uri}': {e}"))
        })
        .map(PyUserDictionary::new)
}
