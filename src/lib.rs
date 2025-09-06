pub mod character_filter;
pub mod dictionary;
pub mod error;
pub mod metadata;
pub mod mode;
pub mod schema;
pub mod segmenter;
pub mod token;
pub mod token_filter;
pub mod tokenizer;
pub mod util;

use pyo3::prelude::*;

use crate::character_filter::{PyCharacterFilter, PyCharacterFilterKind};
use crate::dictionary::{PyDictionary, PyUserDictionary};
use crate::error::PyLinderaError;
use crate::metadata::{PyCompressionAlgorithm, PyMetadata};
use crate::mode::{PyMode, PyPenalty};
use crate::schema::{PyFieldDefinition, PyFieldType, PySchema};
use crate::segmenter::PySegmenter;
use crate::token::PyToken;
use crate::token_filter::{PyTokenFilter, PyTokenFilterKind};
use crate::tokenizer::{PyTokenizer, PyTokenizerBuilder};

#[pyfunction]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[pymodule]
fn lindera(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyToken>()?;
    module.add_class::<PyDictionary>()?;
    module.add_class::<PyUserDictionary>()?;
    module.add_class::<PyTokenizerBuilder>()?;
    module.add_class::<PyTokenizer>()?;
    module.add_class::<PySegmenter>()?;
    module.add_class::<PyLinderaError>()?;
    module.add_class::<PyMode>()?;
    module.add_class::<PyPenalty>()?;
    module.add_class::<PyCharacterFilter>()?;
    module.add_class::<PyCharacterFilterKind>()?;
    module.add_class::<PyTokenFilter>()?;
    module.add_class::<PyTokenFilterKind>()?;
    module.add_class::<PyMetadata>()?;
    module.add_class::<PySchema>()?;
    module.add_class::<PyFieldDefinition>()?;
    module.add_class::<PyFieldType>()?;
    module.add_class::<PyCompressionAlgorithm>()?;

    // Dictionary functions
    module.add_function(wrap_pyfunction!(
        crate::dictionary::build_dictionary,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        crate::dictionary::build_user_dictionary,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        crate::dictionary::load_dictionary,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(
        crate::dictionary::load_user_dictionary,
        module
    )?)?;

    module.add_function(wrap_pyfunction!(version, module)?)?;
    Ok(())
}
