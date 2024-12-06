pub mod dictionary;
pub mod segmenter;
pub mod token;
pub mod tokenizer;
pub mod util;

use pyo3::prelude::*;

use crate::dictionary::{load_dictionary, load_user_dictionary, PyDictionary, PyUserDictionary};
use crate::segmenter::PySegmenter;
use crate::token::PyToken;
use crate::tokenizer::{PyTokenizer, PyTokenizerBuilder};

#[pymodule]
fn lindera_py(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyToken>()?;
    module.add_class::<PyDictionary>()?;
    module.add_class::<PyUserDictionary>()?;
    module.add_class::<PyTokenizerBuilder>()?;
    module.add_class::<PyTokenizer>()?;
    module.add_class::<PySegmenter>()?;

    module.add_function(wrap_pyfunction!(load_dictionary, module)?)?;
    module.add_function(wrap_pyfunction!(load_user_dictionary, module)?)?;

    Ok(())
}
