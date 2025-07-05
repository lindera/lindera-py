use std::borrow::Cow;
use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use lindera::mode::Mode;
use lindera::segmenter::Segmenter;

use crate::dictionary::{PyDictionary, PyUserDictionary};
use crate::token::PyToken;
use crate::util::pydict_to_value;

#[pyclass(name = "Segmenter")]
#[derive(Clone)]
pub struct PySegmenter {
    pub inner: Segmenter,
}

#[pymethods]
impl PySegmenter {
    #[new]
    #[pyo3(signature = (mode, dictionary, user_dictionary=None))]
    fn new(
        mode: &str,
        dictionary: PyDictionary,
        user_dictionary: Option<PyUserDictionary>,
    ) -> PyResult<Self> {
        let m = Mode::from_str(mode)
            .map_err(|err| PyValueError::new_err(format!("Failed to create mode: {err}")))?;
        let d = dictionary.inner;
        let u = user_dictionary.map(|d| d.inner);

        let segmenter = Segmenter::new(m, d, u);

        Ok(Self { inner: segmenter })
    }

    #[pyo3(signature = (config))]
    #[allow(clippy::wrong_self_convention)]
    fn from_config(&self, config: &Bound<'_, PyDict>) -> PyResult<Self> {
        let config_value = pydict_to_value(config)?;
        let segmenter = Segmenter::from_config(&config_value)
            .map_err(|err| PyValueError::new_err(format!("Failed to create tokenizer: {err}")))?;

        Ok(Self { inner: segmenter })
    }

    #[pyo3(signature = (text))]
    fn segment(&self, text: &str) -> PyResult<Vec<PyToken>> {
        let mut tokens = self
            .inner
            .segment(Cow::Borrowed(text))
            .map_err(|err| PyValueError::new_err(format!("Failed to tokenize text: {err}")))?;

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
