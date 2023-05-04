use std::str::FromStr;

use pyo3::{exceptions::PyValueError, prelude::*};

use lindera_core::mode::Mode;
use lindera_filter::token::FilteredToken;
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};

use crate::{
    dictionary::{PyDictionaryConfig, PyUserDictionaryConfig},
    PyToken,
};

#[derive(Clone)]
#[pyclass(name = "TokenizerConfig")]
pub struct PyTokenizerConfig {
    inner: TokenizerConfig,
}

#[pymethods]
impl PyTokenizerConfig {
    #[new]
    fn new(
        dic_config: PyDictionaryConfig,
        mode: &str,
        user_dic_config: Option<PyUserDictionaryConfig>,
    ) -> PyResult<Self> {
        let m = Mode::from_str(mode).map_err(|_err| PyValueError::new_err("Invalid mode"))?;

        Ok(Self {
            inner: TokenizerConfig {
                dictionary: dic_config.inner,
                mode: m,
                user_dictionary: user_dic_config.map(|x| x.inner),
            },
        })
    }
}

#[pyclass(name = "Tokenizer")]
pub struct PyTokenizer {
    inner: Tokenizer,
}

#[pymethods]
impl PyTokenizer {
    #[new]
    fn new(config: PyTokenizerConfig) -> PyResult<Self> {
        Ok(Self {
            inner: Tokenizer::from_config(config.inner)
                .map_err(|_err| PyValueError::new_err("Invalid config"))?,
        })
    }

    fn tokenize(&self, text: &str) -> PyResult<Vec<PyToken>> {
        let tokens = self
            .inner
            .tokenize(text)
            .map_err(|_err| PyValueError::new_err("Tokenize error"))?;

        let mut py_tokens = Vec::new();
        for token in tokens.clone().iter_mut() {
            py_tokens.push(PyToken::from(FilteredToken {
                text: token.text.to_string(),
                byte_start: token.byte_start,
                byte_end: token.byte_end,
                position: token.position,
                position_length: token.position_length,
                details: token
                    .get_details()
                    .ok_or_else(|| PyValueError::new_err("Invalid token details"))?
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            }));
        }

        Ok(py_tokens)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
