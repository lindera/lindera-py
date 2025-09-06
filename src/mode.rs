use lindera::mode::{Mode as LinderaMode, Penalty as LinderaPenalty};
use pyo3::prelude::*;

#[pyclass(name = "Mode")]
#[derive(Debug, Clone, Copy)]
pub enum PyMode {
    Normal,
    Decompose,
}

#[pymethods]
impl PyMode {
    #[new]
    #[pyo3(signature = (mode_str=None))]
    pub fn new(mode_str: Option<&str>) -> PyResult<Self> {
        match mode_str {
            Some("decompose") | Some("Decompose") => Ok(PyMode::Decompose),
            Some("normal") | Some("Normal") | None => Ok(PyMode::Normal),
            Some(s) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid mode: {s}. Must be 'normal' or 'decompose'"
            ))),
        }
    }

    fn __str__(&self) -> &str {
        match self {
            PyMode::Normal => "normal",
            PyMode::Decompose => "decompose",
        }
    }

    fn __repr__(&self) -> String {
        format!("Mode.{self:?}")
    }

    #[getter]
    pub fn name(&self) -> &str {
        self.__str__()
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, PyMode::Normal)
    }

    pub fn is_decompose(&self) -> bool {
        matches!(self, PyMode::Decompose)
    }
}

impl From<PyMode> for LinderaMode {
    fn from(mode: PyMode) -> Self {
        match mode {
            PyMode::Normal => LinderaMode::Normal,
            PyMode::Decompose => LinderaMode::Decompose(LinderaPenalty::default()),
        }
    }
}

impl From<LinderaMode> for PyMode {
    fn from(mode: LinderaMode) -> Self {
        match mode {
            LinderaMode::Normal => PyMode::Normal,
            LinderaMode::Decompose(_) => PyMode::Decompose,
        }
    }
}

#[pyclass(name = "Penalty")]
#[derive(Debug, Clone, Copy)]
pub struct PyPenalty {
    kanji_penalty_length_threshold: usize,
    kanji_penalty_length_penalty: i32,
    other_penalty_length_threshold: usize,
    other_penalty_length_penalty: i32,
}

#[pymethods]
impl PyPenalty {
    #[new]
    #[pyo3(signature = (kanji_penalty_length_threshold=None, kanji_penalty_length_penalty=None, other_penalty_length_threshold=None, other_penalty_length_penalty=None))]
    pub fn new(
        kanji_penalty_length_threshold: Option<usize>,
        kanji_penalty_length_penalty: Option<i32>,
        other_penalty_length_threshold: Option<usize>,
        other_penalty_length_penalty: Option<i32>,
    ) -> Self {
        PyPenalty {
            kanji_penalty_length_threshold: kanji_penalty_length_threshold.unwrap_or(2),
            kanji_penalty_length_penalty: kanji_penalty_length_penalty.unwrap_or(3000),
            other_penalty_length_threshold: other_penalty_length_threshold.unwrap_or(7),
            other_penalty_length_penalty: other_penalty_length_penalty.unwrap_or(1700),
        }
    }

    #[getter]
    pub fn get_kanji_penalty_length_threshold(&self) -> usize {
        self.kanji_penalty_length_threshold
    }

    #[setter]
    pub fn set_kanji_penalty_length_threshold(&mut self, value: usize) {
        self.kanji_penalty_length_threshold = value;
    }

    #[getter]
    pub fn get_kanji_penalty_length_penalty(&self) -> i32 {
        self.kanji_penalty_length_penalty
    }

    #[setter]
    pub fn set_kanji_penalty_length_penalty(&mut self, value: i32) {
        self.kanji_penalty_length_penalty = value;
    }

    #[getter]
    pub fn get_other_penalty_length_threshold(&self) -> usize {
        self.other_penalty_length_threshold
    }

    #[setter]
    pub fn set_other_penalty_length_threshold(&mut self, value: usize) {
        self.other_penalty_length_threshold = value;
    }

    #[getter]
    pub fn get_other_penalty_length_penalty(&self) -> i32 {
        self.other_penalty_length_penalty
    }

    #[setter]
    pub fn set_other_penalty_length_penalty(&mut self, value: i32) {
        self.other_penalty_length_penalty = value;
    }

    fn __str__(&self) -> String {
        format!(
            "Penalty(kanji_threshold={}, kanji_penalty={}, other_threshold={}, other_penalty={})",
            self.kanji_penalty_length_threshold,
            self.kanji_penalty_length_penalty,
            self.other_penalty_length_threshold,
            self.other_penalty_length_penalty
        )
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

impl From<PyPenalty> for LinderaPenalty {
    fn from(penalty: PyPenalty) -> Self {
        LinderaPenalty {
            kanji_penalty_length_threshold: penalty.kanji_penalty_length_threshold,
            kanji_penalty_length_penalty: penalty.kanji_penalty_length_penalty,
            other_penalty_length_threshold: penalty.other_penalty_length_threshold,
            other_penalty_length_penalty: penalty.other_penalty_length_penalty,
        }
    }
}

impl From<LinderaPenalty> for PyPenalty {
    fn from(penalty: LinderaPenalty) -> Self {
        PyPenalty {
            kanji_penalty_length_threshold: penalty.kanji_penalty_length_threshold,
            kanji_penalty_length_penalty: penalty.kanji_penalty_length_penalty,
            other_penalty_length_threshold: penalty.other_penalty_length_threshold,
            other_penalty_length_penalty: penalty.other_penalty_length_penalty,
        }
    }
}
