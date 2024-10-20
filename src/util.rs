use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;

pub fn pydict_to_value(py_dict: &Bound<'_, PyDict>) -> PyResult<Value> {
    let mut map = serde_json::Map::new();
    for (key, value) in py_dict.iter() {
        let key_str: String = key.extract()?; // Convert the key to a String
        let value_json: Value = python_to_json(&value)?; // Convert the value to a JSON Value
        map.insert(key_str, value_json);
    }
    Ok(Value::Object(map))
}

fn python_to_json(obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    if let Ok(py_dict) = obj.downcast::<PyDict>() {
        pydict_to_value(py_dict)
    } else if let Ok(py_list) = obj.downcast::<PyList>() {
        let mut list = Vec::new();
        for elem in py_list.iter() {
            list.push(python_to_json(&elem)?);
        }
        Ok(Value::Array(list))
    } else if let Ok(py_str) = obj.extract::<String>() {
        Ok(Value::String(py_str))
    } else if let Ok(py_int) = obj.extract::<i64>() {
        Ok(Value::Number(serde_json::Number::from(py_int)))
    } else if let Ok(py_float) = obj.extract::<f64>() {
        Ok(Value::Number(
            serde_json::Number::from_f64(py_float).unwrap(),
        ))
    } else if obj.is_none() {
        Ok(Value::Null)
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported Python object",
        ))
    }
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
