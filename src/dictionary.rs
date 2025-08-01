use std::fs;
use std::path::Path;
use std::str::FromStr;

use pyo3::{exceptions::PyValueError, prelude::*};

use lindera::dictionary::{
    Dictionary, DictionaryKind, UserDictionary, load_dictionary_from_kind,
    load_dictionary_from_path, load_user_dictionary_from_bin, load_user_dictionary_from_csv,
    resolve_builder, resolve_metadata,
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
#[pyo3(signature = (kind, input_dir, output_dir))]
pub fn build_dictionary(kind: &str, input_dir: &str, output_dir: &str) -> PyResult<()> {
    let dict_kind =
        DictionaryKind::from_str(kind).map_err(|_err| PyValueError::new_err("Invalid kind"))?;

    let metadata = resolve_metadata(dict_kind.clone())
        .map_err(|err| PyValueError::new_err(format!("Failed to resolve metadata: {err}")))?;

    let builder = resolve_builder(dict_kind)
        .map_err(|err| PyValueError::new_err(format!("Failed to resolve builder: {err}")))?;

    // Ensure output directory exists
    fs::create_dir_all(output_dir).map_err(|err| {
        PyValueError::new_err(format!("Failed to create output directory: {err}"))
    })?;

    builder
        .build_dictionary(&metadata, Path::new(input_dir), Path::new(output_dir))
        .map_err(|err| PyValueError::new_err(format!("Failed to build dictionary: {err}")))?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (kind, input_file, output_dir))]
pub fn build_user_dictionary(kind: &str, input_file: &str, output_dir: &str) -> PyResult<()> {
    let dict_kind =
        DictionaryKind::from_str(kind).map_err(|_err| PyValueError::new_err("Invalid kind"))?;

    let metadata = resolve_metadata(dict_kind.clone())
        .map_err(|err| PyValueError::new_err(format!("Failed to resolve metadata: {err}")))?;

    let builder = resolve_builder(dict_kind)
        .map_err(|err| PyValueError::new_err(format!("Failed to resolve builder: {err}")))?;

    // Ensure output directory exists
    fs::create_dir_all(output_dir).map_err(|err| {
        PyValueError::new_err(format!("Failed to create output directory: {err}"))
    })?;

    // Determine output file name based on input file
    // If the input file has no name, we cannot determine the output file name.
    // In that case, we return an error.
    // e.g., /path/to/input/file.txt -> /path/to/output/file.bin
    let output_file = if let Some(filename) = Path::new(input_file).file_name() {
        let mut output_file = Path::new(output_dir).join(filename);
        output_file.set_extension("bin");
        output_file
    } else {
        return Err(PyValueError::new_err("Failed to determine output filename"));
    };

    builder
        .build_user_dictionary(&metadata, Path::new(input_file), output_file.as_path())
        .map_err(|err| PyValueError::new_err(format!("Failed to build user dictionary: {err}")))?;

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (kind=None, path=None))]
pub fn load_dictionary(kind: Option<&str>, path: Option<&str>) -> PyResult<PyDictionary> {
    match (kind, path) {
        (Some(kind_str), None) => {
            let k = DictionaryKind::from_str(kind_str)
                .map_err(|_err| PyValueError::new_err("Invalid kind"))?;
            let dictionary = load_dictionary_from_kind(k).map_err(|err| {
                PyValueError::new_err(format!("Failed to load dictionary: {err}"))
            })?;

            Ok(PyDictionary { inner: dictionary })
        }
        (None, Some(path_str)) => {
            let p = Path::new(path_str);
            let dictionary = load_dictionary_from_path(p).map_err(|err| {
                PyValueError::new_err(format!("Failed to load dictionary: {err}"))
            })?;

            Ok(PyDictionary { inner: dictionary })
        }
        _ => Err(PyValueError::new_err("Invalid arguments")),
    }
}

#[pyfunction]
#[pyo3(signature = (path, kind=None))]
pub fn load_user_dictionary(path: &str, kind: Option<&str>) -> PyResult<PyUserDictionary> {
    let p = Path::new(path);
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
                    PyValueError::new_err(format!("Failed to load user dictionary: {err}"))
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
                "Dictionary type must be None if Binary file specified",
            )),
            None => {
                let user_dictionary = load_user_dictionary_from_bin(p).map_err(|err| {
                    PyValueError::new_err(format!("Failed to load user dictionary: {err}"))
                })?;

                Ok(PyUserDictionary {
                    inner: user_dictionary,
                })
            }
        },
        _ => Err(PyValueError::new_err(format!(
            "Unsupported file: path:{path}, kind:{kind:?}"
        ))),
    }
}
