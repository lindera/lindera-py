pub mod dictionary;
pub mod token;
pub mod tokenizer;
pub mod util;

use pyo3::prelude::*;

use crate::dictionary::{load_dictionary, load_user_dictionary, PyDictionary, PyUserDictionary};
use crate::token::PyToken;
use crate::tokenizer::PyTokenizer;

#[pymodule]
fn lindera(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyToken>()?;
    module.add_class::<PyDictionary>()?;
    module.add_class::<PyUserDictionary>()?;
    module.add_class::<PyTokenizer>()?;

    module.add_function(wrap_pyfunction!(load_dictionary, module)?)?;
    module.add_function(wrap_pyfunction!(load_user_dictionary, module)?)?;

    Ok(())
}
