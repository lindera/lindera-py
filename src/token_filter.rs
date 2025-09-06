use pyo3::prelude::*;
use std::collections::HashMap;

use crate::token::PyToken;

#[pyclass(name = "TokenFilterKind")]
#[derive(Debug, Clone)]
pub enum PyTokenFilterKind {
    JapaneseBaseForm,
    JapaneseCompoundWord,
    JapaneseKana,
    JapaneseKatakanaStem,
    JapaneseKeepTags,
    JapaneseNumber,
    JapaneseReadingForm,
    JapaneseStopTags,
    KeepWords,
    KoreanKeepTags,
    KoreanReadingForm,
    KoreanStopTags,
    Length,
    Lowercase,
    Mapping,
    RemoveDiacriticalMark,
    StopWords,
    Uppercase,
}

#[pymethods]
impl PyTokenFilterKind {
    fn __str__(&self) -> &str {
        match self {
            PyTokenFilterKind::JapaneseBaseForm => "japanese_base_form",
            PyTokenFilterKind::JapaneseCompoundWord => "japanese_compound_word",
            PyTokenFilterKind::JapaneseKana => "japanese_kana",
            PyTokenFilterKind::JapaneseKatakanaStem => "japanese_katakana_stem",
            PyTokenFilterKind::JapaneseKeepTags => "japanese_keep_tags",
            PyTokenFilterKind::JapaneseNumber => "japanese_number",
            PyTokenFilterKind::JapaneseReadingForm => "japanese_reading_form",
            PyTokenFilterKind::JapaneseStopTags => "japanese_stop_tags",
            PyTokenFilterKind::KeepWords => "keep_words",
            PyTokenFilterKind::KoreanKeepTags => "korean_keep_tags",
            PyTokenFilterKind::KoreanReadingForm => "korean_reading_form",
            PyTokenFilterKind::KoreanStopTags => "korean_stop_tags",
            PyTokenFilterKind::Length => "length",
            PyTokenFilterKind::Lowercase => "lowercase",
            PyTokenFilterKind::Mapping => "mapping",
            PyTokenFilterKind::RemoveDiacriticalMark => "remove_diacritical_mark",
            PyTokenFilterKind::StopWords => "stop_words",
            PyTokenFilterKind::Uppercase => "uppercase",
        }
    }

    fn __repr__(&self) -> String {
        format!("TokenFilterKind.{:?}", self)
    }
}

#[pyclass(name = "TokenFilter")]
#[derive(Clone)]
pub struct PyTokenFilter {
    kind: PyTokenFilterKind,
    config: HashMap<String, String>, // Simplified config storage
}

#[pymethods]
impl PyTokenFilter {
    #[new]
    pub fn new(kind: String, config: Option<HashMap<String, String>>) -> PyResult<Self> {
        let filter_kind = match kind.as_str() {
            "japanese_base_form" => PyTokenFilterKind::JapaneseBaseForm,
            "japanese_compound_word" => PyTokenFilterKind::JapaneseCompoundWord,
            "japanese_kana" => PyTokenFilterKind::JapaneseKana,
            "japanese_katakana_stem" => PyTokenFilterKind::JapaneseKatakanaStem,
            "japanese_keep_tags" => PyTokenFilterKind::JapaneseKeepTags,
            "japanese_number" => PyTokenFilterKind::JapaneseNumber,
            "japanese_reading_form" => PyTokenFilterKind::JapaneseReadingForm,
            "japanese_stop_tags" => PyTokenFilterKind::JapaneseStopTags,
            "keep_words" => PyTokenFilterKind::KeepWords,
            "korean_keep_tags" => PyTokenFilterKind::KoreanKeepTags,
            "korean_reading_form" => PyTokenFilterKind::KoreanReadingForm,
            "korean_stop_tags" => PyTokenFilterKind::KoreanStopTags,
            "length" => PyTokenFilterKind::Length,
            "lowercase" => PyTokenFilterKind::Lowercase,
            "mapping" => PyTokenFilterKind::Mapping,
            "remove_diacritical_mark" => PyTokenFilterKind::RemoveDiacriticalMark,
            "stop_words" => PyTokenFilterKind::StopWords,
            "uppercase" => PyTokenFilterKind::Uppercase,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown token filter kind: {}",
                    kind
                )));
            }
        };

        Ok(PyTokenFilter {
            kind: filter_kind,
            config: config.unwrap_or_default(),
        })
    }

    // Static methods for common filters
    #[staticmethod]
    pub fn japanese_base_form() -> PyResult<Self> {
        Ok(PyTokenFilter {
            kind: PyTokenFilterKind::JapaneseBaseForm,
            config: HashMap::new(),
        })
    }

    #[staticmethod]
    pub fn lowercase() -> PyResult<Self> {
        Ok(PyTokenFilter {
            kind: PyTokenFilterKind::Lowercase,
            config: HashMap::new(),
        })
    }

    #[staticmethod]
    pub fn uppercase() -> PyResult<Self> {
        Ok(PyTokenFilter {
            kind: PyTokenFilterKind::Uppercase,
            config: HashMap::new(),
        })
    }

    #[staticmethod]
    pub fn length(min: Option<usize>, max: Option<usize>) -> PyResult<Self> {
        let mut config = HashMap::new();
        config.insert("min".to_string(), min.unwrap_or(1).to_string());
        config.insert("max".to_string(), max.unwrap_or(usize::MAX).to_string());

        Ok(PyTokenFilter {
            kind: PyTokenFilterKind::Length,
            config,
        })
    }

    #[staticmethod]
    pub fn stop_words(words: Vec<String>) -> PyResult<Self> {
        let mut config = HashMap::new();
        config.insert("words".to_string(), words.join(","));

        Ok(PyTokenFilter {
            kind: PyTokenFilterKind::StopWords,
            config,
        })
    }

    pub fn apply(&self, tokens: Vec<PyToken>) -> PyResult<Vec<PyToken>> {
        // Simplified implementation - in real usage this would call Lindera filters
        match self.kind {
            PyTokenFilterKind::Lowercase => Ok(tokens
                .into_iter()
                .map(|mut token| {
                    token.text = token.text.to_lowercase();
                    token
                })
                .collect()),
            PyTokenFilterKind::Uppercase => Ok(tokens
                .into_iter()
                .map(|mut token| {
                    token.text = token.text.to_uppercase();
                    token
                })
                .collect()),
            PyTokenFilterKind::Length => {
                let min = self
                    .config
                    .get("min")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                let max = self
                    .config
                    .get("max")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(usize::MAX);

                Ok(tokens
                    .into_iter()
                    .filter(|token| {
                        let len = token.text.chars().count();
                        len >= min && len <= max
                    })
                    .collect())
            }
            PyTokenFilterKind::StopWords => {
                let stop_words: std::collections::HashSet<String> = self
                    .config
                    .get("words")
                    .map(|s| s.split(',').map(|w| w.to_string()).collect())
                    .unwrap_or_default();

                Ok(tokens
                    .into_iter()
                    .filter(|token| !stop_words.contains(&token.text))
                    .collect())
            }
            _ => {
                // Placeholder for other filters
                Ok(tokens)
            }
        }
    }

    #[getter]
    pub fn kind(&self) -> PyTokenFilterKind {
        self.kind.clone()
    }

    #[getter]
    pub fn config(&self) -> HashMap<String, String> {
        self.config.clone()
    }

    fn __str__(&self) -> String {
        format!("TokenFilter(kind={})", self.kind.__str__())
    }

    fn __repr__(&self) -> String {
        format!(
            "TokenFilter(kind={:?}, config={:?})",
            self.kind, self.config
        )
    }
}
