use std::path::Path;
use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::json;

use lindera::character_filter::CharacterFilterLoader;
use lindera::dictionary::DictionaryKind;
use lindera::mode::Mode;
use lindera::token_filter::TokenFilterLoader;
use lindera::tokenizer::{Tokenizer, TokenizerBuilder};

use crate::segmenter::PySegmenter;
use crate::token::PyToken;
use crate::util::pydict_to_value;

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
            PyValueError::new_err(format!("Failed to create TokenizerBuilder: {}", err))
        })?;

        Ok(Self { inner })
    }

    #[pyo3(signature = (file_path))]
    #[allow(clippy::wrong_self_convention)]
    fn from_file(&self, file_path: &str) -> PyResult<Self> {
        let inner = TokenizerBuilder::from_file(Path::new(file_path)).map_err(|err| {
            PyValueError::new_err(format!("Failed to load config from file: {}", err))
        })?;

        Ok(Self { inner })
    }

    #[pyo3(signature = (mode))]
    fn set_mode<'a>(mut slf: PyRefMut<'a, Self>, mode: &str) -> PyResult<PyRefMut<'a, Self>> {
        let m = Mode::from_str(mode)
            .map_err(|err| PyValueError::new_err(format!("Failed to create mode: {}", err)))?;

        slf.inner.set_segmenter_mode(&m);

        Ok(slf)
    }

    #[pyo3(signature = (kind))]
    fn set_dictionary_kind<'a>(
        mut slf: PyRefMut<'a, Self>,
        kind: &str,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let k = DictionaryKind::from_str(kind)
            .map_err(|err| PyValueError::new_err(format!("Failed to create kind: {}", err)))?;

        slf.inner.set_segmenter_dictionary_kind(&k);

        Ok(slf)
    }

    #[pyo3(signature = (path))]
    fn set_dictionary_path<'a>(
        mut slf: PyRefMut<'a, Self>,
        path: &str,
    ) -> PyResult<PyRefMut<'a, Self>> {
        slf.inner.set_segmenter_dictionary_path(Path::new(path));

        Ok(slf)
    }

    #[pyo3(signature = (path))]
    fn set_user_dictionary_path<'a>(
        mut slf: PyRefMut<'a, Self>,
        path: &str,
    ) -> PyResult<PyRefMut<'a, Self>> {
        slf.inner
            .set_segmenter_user_dictionary_path(Path::new(path));

        Ok(slf)
    }

    #[pyo3(signature = (kind))]
    fn set_user_dictionary_kind<'a>(
        mut slf: PyRefMut<'a, Self>,
        kind: &str,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let k = DictionaryKind::from_str(kind)
            .map_err(|err| PyValueError::new_err(format!("Failed to create kind: {}", err)))?;

        slf.inner.set_segmenter_user_dictionary_kind(&k);

        Ok(slf)
    }

    #[pyo3(signature = (name, **args))]
    fn append_character_filter<'a>(
        mut slf: PyRefMut<'a, Self>,
        name: &str,
        args: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let character_filter_args = match args {
            Some(a) => pydict_to_value(a)?,
            None => json!({}),
        };

        slf.inner
            .append_character_filter(name, &character_filter_args);

        Ok(slf)
    }

    #[pyo3(signature = (name, **args))]
    fn append_token_filter<'a>(
        mut slf: PyRefMut<'a, Self>,
        name: &str,
        args: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        let token_filter_args = match args {
            Some(a) => pydict_to_value(a)?,
            None => json!({}),
        };

        slf.inner.append_token_filter(name, &token_filter_args);

        Ok(slf)
    }

    #[pyo3(signature = ())]
    fn build(&self) -> PyResult<PyTokenizer> {
        self.inner
            .build()
            .map_err(|err| PyValueError::new_err(format!("Failed to build tokenizer: {}", err)))
            .map(|t| PyTokenizer { inner: t })
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
            .map_err(|err| PyValueError::new_err(format!("Failed to create tokenizer: {}", err)))?;

        Ok(Self { inner: tokenizer })
    }

    #[pyo3(signature = (name, **args))]
    fn append_character_filter(
        &mut self,
        name: &str,
        args: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let value = match args {
            Some(pydict) => pydict_to_value(pydict)?,
            None => json!({}),
        };

        let filter = CharacterFilterLoader::load_from_value(name, &value).map_err(|err| {
            PyValueError::new_err(format!("Failed to load character filter: {}", err))
        })?;
        self.inner.append_character_filter(filter);

        Ok(())
    }

    #[pyo3(signature = (name, **args))]
    fn append_token_filter(
        &mut self,
        name: &str,
        args: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let value = match args {
            Some(pydict) => pydict_to_value(pydict)?,
            None => json!({}),
        };

        let filter = TokenFilterLoader::load_from_value(name, &value).map_err(|err| {
            PyValueError::new_err(format!("Failed to load token filter: {}", err))
        })?;
        self.inner.append_token_filter(filter);

        Ok(())
    }

    #[pyo3(signature = (text))]
    fn tokenize(&self, text: &str) -> PyResult<Vec<PyToken>> {
        let mut tokens = self
            .inner
            .tokenize(text)
            .map_err(|err| PyValueError::new_err(format!("Failed to tokenize text: {}", err)))?;

        Ok(tokens
            .iter_mut()
            .map(|t| PyToken {
                #[allow(clippy::suspicious_to_owned)]
                text: t.text.to_owned().to_string(),
                byte_start: t.byte_start,
                byte_end: t.byte_end,
                position: t.position,
                position_length: t.position_length,
                details: t.details().iter().map(|d| d.to_string()).collect(),
            })
            .collect())
    }
}
