use std::collections::HashMap;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyNone, PyString};
use serde_json::{json, Value};

pub fn pyany_to_value(value: &Bound<'_, PyAny>) -> PyResult<Value> {
    if value.is_instance_of::<PyString>() {
        Ok(Value::from(value.extract::<String>()?))
    } else if value.is_instance_of::<PyBool>() {
        Ok(Value::from(value.extract::<bool>()?))
    } else if value.is_instance_of::<PyFloat>() {
        Ok(Value::from(value.extract::<f64>()?))
    } else if value.is_instance_of::<PyInt>() {
        Ok(Value::from(value.extract::<i64>()?))
    } else if value.is_instance_of::<PyList>() {
        pylist_to_value(&value.extract::<Bound<'_, PyList>>()?)
    } else if value.is_instance_of::<PyDict>() {
        pydict_to_value(&value.extract::<Bound<'_, PyDict>>()?)
    } else if value.is_instance_of::<PyNone>() {
        Ok(Value::Null)
    } else {
        Err(PyErr::new::<PyTypeError, _>(format!(
            "Unsupported Python object: {:?}",
            value
        )))
    }
}

fn pylist_to_value(pylist: &Bound<'_, PyList>) -> PyResult<Value> {
    let mut vec: Vec<Value> = Vec::new();
    for value in pylist.into_iter() {
        vec.push(pyany_to_value(&value)?);
    }
    Ok(vec.into())
}

pub fn pydict_to_value(pydict: &Bound<'_, PyDict>) -> PyResult<Value> {
    let mut map: HashMap<String, Value> = HashMap::new();
    for (key, value) in pydict.into_iter() {
        map.insert(key.extract::<String>()?, pyany_to_value(&value)?);
    }
    Ok(json!(map))
}

#[cfg(test)]
mod tests {
    // use pyo3::types::IntoPyDict;
    // use serde_json::json;

    // use super::*;

    // #[test]
    // fn test_pydict_to_value() {
    //     Python::with_gil(|py| {
    //         let py_dict = [("key1", "value1"), ("key2", "value2")].into_py_dict_bound(py);
    //         let value = pydict_to_value(&py_dict).unwrap();
    //         let expected = json!({"key1": "value1", "key2": "value2"});
    //         assert_eq!(value, expected);
    //     });
    // }

    // #[test]
    // fn test_python_to_json_with_dict() {
    //     Python::with_gil(|py| {
    //         let py_dict = [("key1", "value1"), ("key2", "value2")].into_py_dict_bound(py);
    //         let value = python_to_json(&py_dict).unwrap();
    //         let expected = json!({"key1": "value1", "key2": "value2"});
    //         assert_eq!(value, expected);
    //     });
    // }

    // #[test]
    // fn test_python_to_json_with_list() {
    //     Python::with_gil(|py| {
    //         let binding = vec!["value1", "value2"].into_py(py);
    //         let py_list = binding.downcast_bound::<PyAny>(py).unwrap();
    //         let value = python_to_json(py_list).unwrap();
    //         let expected = json!(["value1", "value2"]);
    //         assert_eq!(value, expected);
    //     });
    // }

    // #[test]
    // fn test_python_to_json_with_string() {
    //     Python::with_gil(|py| {
    //         let binding = "value1".to_string().into_py(py);
    //         let py_str = binding.downcast_bound::<PyAny>(py).unwrap();
    //         let value = python_to_json(py_str).unwrap();
    //         let expected = json!("value");
    //         assert_eq!(value, expected);
    //     });
    // }

    // #[test]
    // fn test_python_to_json_with_int() {
    //     Python::with_gil(|py| {
    //         let binding = 42_i64.into_py(py);
    //         let py_int = binding.downcast_bound::<PyAny>(py).unwrap();
    //         let value = python_to_json(py_int).unwrap();
    //         let expected = json!(42);
    //         assert_eq!(value, expected);
    //     });
    // }

    // #[test]
    // fn test_python_to_json_with_float() {
    //     Python::with_gil(|py| {
    //         let binding = 3.14_f64.into_py(py);
    //         let py_float = binding.downcast_bound::<PyAny>(py).unwrap();
    //         let value = python_to_json(py_float).unwrap();
    //         let expected = json!(3.14);
    //         assert_eq!(value, expected);
    //     });
    // }

    // #[test]
    // fn test_python_to_json_with_none() {
    //     Python::with_gil(|py| {
    //         let binding = py.None();
    //         let py_none = binding.downcast_bound::<PyAny>(py).unwrap();
    //         let value = python_to_json(py_none).unwrap();
    //         let expected = json!(null);
    //         assert_eq!(value, expected);
    //     });
    // }

    // #[test]
    // fn test_python_to_json_with_unsupported_type() {
    //     Python::with_gil(|py| {
    //         let py_tuple = (1, 2).into_py(py);
    //         let result = python_to_json(py_tuple);
    //         assert!(result.is_err());
    //     });
    // }
}
