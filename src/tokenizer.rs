use std::path::Path;
use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use lindera::mode::Mode;
use lindera::tokenizer::{Tokenizer, TokenizerBuilder};

use crate::segmenter::PySegmenter;
use crate::token::PyToken;
use crate::util::pydict_to_value;

pub type PyDictRef<'a> = &'a Bound<'a, PyDict>;

#[pyclass(name = "TokenizerBuilder")]
pub struct PyTokenizerBuilder {
    pub inner: TokenizerBuilder,
}

#[pymethods]
impl PyTokenizerBuilder {
    #[new]
    #[pyo3(signature = ())]
    fn new() -> PyResult<Self> {
        let inner = TokenizerBuilder::new().map_err(|err| {
            PyValueError::new_err(format!("Failed to create TokenizerBuilder: {err}"))
        })?;

        Ok(Self { inner })
    }

    #[pyo3(signature = (file_path))]
    #[allow(clippy::wrong_self_convention)]
    fn from_file(&self, file_path: &str) -> PyResult<Self> {
        let inner = TokenizerBuilder::from_file(Path::new(file_path)).map_err(|err| {
            PyValueError::new_err(format!("Failed to load config from file: {err}"))
        })?;

        Ok(Self { inner })
    }

    #[pyo3(signature = (mode))]
    fn set_mode<'a>(mut slf: PyRefMut<'a, Self>, mode: &str) -> PyResult<PyRefMut<'a, Self>> {
        let m = Mode::from_str(mode)
            .map_err(|err| PyValueError::new_err(format!("Failed to create mode: {err}")))?;

        slf.inner.set_segmenter_mode(&m);

        Ok(slf)
    }

    #[pyo3(signature = (path))]
    fn set_dictionary<'a>(mut slf: PyRefMut<'a, Self>, path: &str) -> PyResult<PyRefMut<'a, Self>> {
        slf.inner.set_segmenter_dictionary(path);

        Ok(slf)
    }

    #[pyo3(signature = (uri))]
    fn set_user_dictionary<'a>(
        mut slf: PyRefMut<'a, Self>,
        uri: &str,
    ) -> PyResult<PyRefMut<'a, Self>> {
        slf.inner.set_segmenter_user_dictionary(uri);
        Ok(slf)
    }

    // Character filter and token filter integration
    #[pyo3(signature = (kind, args=None))]
    fn append_character_filter<'a>(
        mut slf: PyRefMut<'a, Self>,
        kind: &str,
        args: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let filter_args = if let Some(dict) = args {
            pydict_to_value(dict)?
        } else {
            serde_json::Value::Object(serde_json::Map::new())
        };

        slf.inner.append_character_filter(kind, &filter_args);

        Ok(slf)
    }

    #[pyo3(signature = (kind, args=None))]
    fn append_token_filter<'a>(
        mut slf: PyRefMut<'a, Self>,
        kind: &str,
        args: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let filter_args = if let Some(dict) = args {
            pydict_to_value(dict)?
        } else {
            serde_json::Value::Object(serde_json::Map::new())
        };

        slf.inner.append_token_filter(kind, &filter_args);

        Ok(slf)
    }

    #[pyo3(signature = ())]
    fn build(&self) -> PyResult<PyTokenizer> {
        let tokenizer = self
            .inner
            .build()
            .map_err(|err| PyValueError::new_err(format!("Failed to build tokenizer: {err}")))?;

        Ok(PyTokenizer { inner: tokenizer })
    }
}

#[pyclass(name = "Tokenizer")]
pub struct PyTokenizer {
    inner: Tokenizer,
}

#[pymethods]
impl PyTokenizer {
    #[new]
    #[pyo3(signature = (segmenter))]
    fn new(segmenter: PySegmenter) -> PyResult<Self> {
        Ok(Self {
            inner: Tokenizer::new(segmenter.inner),
        })
    }

    #[pyo3(signature = (config))]
    #[allow(clippy::wrong_self_convention)]
    fn from_config(&self, config: &Bound<'_, PyDict>) -> PyResult<Self> {
        let config_value = pydict_to_value(config)?;
        let tokenizer = Tokenizer::from_config(&config_value)
            .map_err(|err| PyValueError::new_err(format!("Failed to create tokenizer: {err}")))?;

        Ok(Self { inner: tokenizer })
    }

    #[pyo3(signature = (text))]
    fn tokenize(&self, text: &str) -> PyResult<Vec<PyToken>> {
        // Tokenize the processed text
        let mut tokens = self
            .inner
            .tokenize(text)
            .map_err(|err| PyValueError::new_err(format!("Failed to tokenize text: {err}")))?;

        // Convert to PyToken
        let py_tokens: Vec<PyToken> = tokens
            .iter_mut()
            .map(|t| PyToken {
                text: t.text.to_string(),
                byte_start: t.byte_start,
                byte_end: t.byte_end,
                position: t.position,
                position_length: t.position_length,
                details: t.details().iter().map(|d| d.to_string()).collect(),
            })
            .collect();

        Ok(py_tokens)
    }
}
