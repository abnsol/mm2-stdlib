use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

static PYTHON_FNS: Lazy<Mutex<HashMap<String, Py<PyAny>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn insert(name: String, func: Py<PyAny>) {
    PYTHON_FNS.lock().unwrap().insert(name, func);
}


pub fn get(py: Python<'_>, name: &str) -> Option<Py<PyAny>> {
        PYTHON_FNS.lock().unwrap().get(name).map(|f| f.clone_ref(py))
}