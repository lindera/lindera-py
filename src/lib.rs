// pub mod character_filter;
pub mod dictionary;
pub mod segmenter;
pub mod token;
// pub mod token_filter;
pub mod tokenizer;
pub mod util;

use pyo3::prelude::*;

// use crate::character_filter::japanese_iteration_mark::PyJapaneseIterationMarkCharacterFilter;
// use crate::character_filter::mapping::PyMappingCharacterFilter;
// use crate::character_filter::regex::PyRegexCharacterFilter;
// use crate::character_filter::unicode_normalize::PyUnicodeNormalizeCharacterFilter;
// use crate::character_filter::PyCharacterFilter;
use crate::dictionary::{load_dictionary, load_user_dictionary, PyDictionary, PyUserDictionary};
use crate::segmenter::PySegmenter;
use crate::token::PyToken;
// use crate::token_filter::japanese_base_form::PyJapaneseBaseFormTokenFilter;
// use crate::token_filter::japanese_compound_word::PyJapaneseCompoundWordTokenFilter;
// use crate::token_filter::japanese_kana::PyJapaneseKanaTokenFilter;
// use crate::token_filter::japanese_katakana_stem::PyJapaneseKatakanaStemTokenFilter;
// use crate::token_filter::japanese_keep_tags::PyJapaneseKeepTagsTokenFilter;
// use crate::token_filter::japanese_number::PyJapaneseNumberTokenFilter;
// use crate::token_filter::japanese_reading_form::PyJapaneseReadingFormTokenFilter;
// use crate::token_filter::japanese_stop_tags::PyJapaneseStopTagsTokenFilter;
// use crate::token_filter::keep_words::PyKeepWordsTokenFilter;
// use crate::token_filter::korean_keep_tags::PyKoreanKeepTagsTokenFilter;
// use crate::token_filter::korean_reading_form::PyKoreanReadingFormTokenFilter;
// use crate::token_filter::korean_stop_tags::PyKoreanStopTagsTokenFilter;
// use crate::token_filter::length::PyLengthTokenFilter;
// use crate::token_filter::lowercase::PyLowercaseTokenFilter;
// use crate::token_filter::mapping::PyMappingTokenFilter;
// use crate::token_filter::remove_diacritical_mark::PyRemoveDiacriticalMarkTokenFilter;
// use crate::token_filter::stop_words::PyStopWordsTokenFilter;
// use crate::token_filter::uppercase::PyUppercaseTokenFilter;
// use crate::token_filter::PyTokenFilter;
use crate::tokenizer::{PyTokenizer, PyTokenizerBuilder};

#[pymodule]
fn lindera(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyToken>()?;
    module.add_class::<PyDictionary>()?;
    module.add_class::<PyUserDictionary>()?;
    module.add_class::<PyTokenizerBuilder>()?;
    module.add_class::<PyTokenizer>()?;
    module.add_class::<PySegmenter>()?;
    // module.add_class::<PyCharacterFilter>()?;
    // module.add_class::<PyTokenFilter>()?;
    // module.add_class::<PyJapaneseIterationMarkCharacterFilter>()?;
    // module.add_class::<PyMappingCharacterFilter>()?;
    // module.add_class::<PyRegexCharacterFilter>()?;
    // module.add_class::<PyUnicodeNormalizeCharacterFilter>()?;
    // module.add_class::<PyJapaneseBaseFormTokenFilter>()?;
    // module.add_class::<PyJapaneseCompoundWordTokenFilter>()?;
    // module.add_class::<PyJapaneseKanaTokenFilter>()?;
    // module.add_class::<PyJapaneseKatakanaStemTokenFilter>()?;
    // module.add_class::<PyJapaneseKeepTagsTokenFilter>()?;
    // module.add_class::<PyJapaneseNumberTokenFilter>()?;
    // module.add_class::<PyJapaneseReadingFormTokenFilter>()?;
    // module.add_class::<PyJapaneseStopTagsTokenFilter>()?;
    // module.add_class::<PyKeepWordsTokenFilter>()?;
    // module.add_class::<PyKoreanKeepTagsTokenFilter>()?;
    // module.add_class::<PyKoreanReadingFormTokenFilter>()?;
    // module.add_class::<PyKoreanStopTagsTokenFilter>()?;
    // module.add_class::<PyLengthTokenFilter>()?;
    // module.add_class::<PyLowercaseTokenFilter>()?;
    // module.add_class::<PyMappingTokenFilter>()?;
    // module.add_class::<PyRemoveDiacriticalMarkTokenFilter>()?;
    // module.add_class::<PyStopWordsTokenFilter>()?;
    // module.add_class::<PyUppercaseTokenFilter>()?;

    module.add_function(wrap_pyfunction!(load_dictionary, module)?)?;
    module.add_function(wrap_pyfunction!(load_user_dictionary, module)?)?;

    Ok(())
}
