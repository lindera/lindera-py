use std::str::FromStr;

use lindera::token_filter::TokenFilterLoader;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::json;

use lindera::character_filter::CharacterFilterLoader;
use lindera::mode::Mode;
use lindera::tokenizer::Tokenizer;

use crate::dictionary::{PyDictionary, PyUserDictionary};
use crate::token::PyToken;
use crate::util::pydict_to_value;

#[pyclass(name = "Tokenizer")]
pub struct PyTokenizer {
    inner: Tokenizer,
}

#[pymethods]
impl PyTokenizer {
    #[new]
    #[pyo3(signature = (mode, dictionary, user_dictionary=None))]
    fn new(
        mode: &str,
        dictionary: PyDictionary,
        user_dictionary: Option<PyUserDictionary>,
    ) -> PyResult<Self> {
        let m = Mode::from_str(mode)
            .map_err(|err| PyValueError::new_err(format!("Failed to create mode: {}", err)))?;
        let u = user_dictionary.map(|d| d.inner);
        Ok(Self {
            inner: Tokenizer::new(m, dictionary.inner, u),
        })
    }

    #[pyo3(signature = (name, **args))]
    fn append_character_filter(
        &mut self,
        name: &str,
        args: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let character_filter_args = match args {
            Some(a) => pydict_to_value(a)?,
            None => json!({}),
        };

        let filter = CharacterFilterLoader::load_from_value(name, &character_filter_args).map_err(
            |err| PyValueError::new_err(format!("Failed to load character filter: {}", err)),
        )?;
        self.inner.append_character_filter(filter);

        Ok(())
    }

    #[pyo3(signature = (name, **args))]
    fn append_token_filter(
        &mut self,
        name: &str,
        args: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let token_filter_args = match args {
            Some(a) => pydict_to_value(a)?,
            None => json!({}),
        };

        let filter =
            TokenFilterLoader::load_from_value(name, &token_filter_args).map_err(|err| {
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
