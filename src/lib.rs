#[cfg(any(
    feature = "ipadic-filter",
    feature = "unidic-filter",
    feature = "ko-dic-filter",
    feature = "cc-cedict-filter",
))]
pub mod analyzer;
pub mod dictionary;
pub mod tokenizer;

#[cfg(any(
    feature = "ipadic-filter",
    feature = "unidic-filter",
    feature = "ko-dic-filter",
    feature = "cc-cedict-filter",
))]
use analyzer::PyAnalyzer;
use dictionary::{PyDictionaryConfig, PyUserDictionaryConfig};
use pyo3::prelude::*;

use lindera_filter::token::FilteredToken;

use crate::{
    dictionary::{build_dictionary, build_user_dictionary},
    tokenizer::{PyTokenizer, PyTokenizerConfig},
};

#[pyclass(name = "Token")]
struct PyToken {
    #[pyo3(get)]
    text: String,
    #[pyo3(get)]
    details: Vec<String>,
}

impl From<FilteredToken> for PyToken {
    fn from(token: FilteredToken) -> Self {
        PyToken {
            text: token.text.to_string(),
            details: token.details,
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn lindera_py(_py: Python, m: &PyModule) -> PyResult<()> {
    #[cfg(any(
        feature = "ipadic-filter",
        feature = "unidic-filter",
        feature = "ko-dic-filter",
        feature = "cc-cedict-filter",
    ))]
    m.add_class::<PyAnalyzer>()?;
    m.add_class::<PyTokenizer>()?;
    m.add_class::<PyDictionaryConfig>()?;
    m.add_class::<PyUserDictionaryConfig>()?;
    m.add_class::<PyTokenizerConfig>()?;
    m.add_function(wrap_pyfunction!(build_dictionary, m)?)?;
    m.add_function(wrap_pyfunction!(build_user_dictionary, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
