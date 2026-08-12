mod registry;
mod adapter;

use eval::{EvalScope, FuncType};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::ffi::CString;

/// Called from Python: register a callable under a pure-function name.
#[pyfunction]
fn register_py_fn(name: &str, func: Py<PyAny>) -> PyResult<()> {
    registry::insert(name.to_string(), func);
    Ok(())
}

/// Hook into MORK's pure EvalScope.
pub fn register(scope: &mut EvalScope) {
    Python::with_gil(|py| {
        let python_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/python");

        let mork_python = PyModule::new(py, "mork_python")
            .expect("failed to create embedded mork_python module");
        mork_python
            .add_function(wrap_pyfunction!(register_py_fn, &mork_python).expect("failed to add register_py_fn to mork_python"))
            .expect("failed to populate embedded mork_python module");

        let sys = PyModule::import(py, "sys").expect("failed to import sys");
        let modules = sys
            .getattr("modules")
            .expect("failed to access sys.modules")
            .downcast_into::<PyDict>()
            .expect("sys.modules was not a dict");
        modules
            .set_item("mork_python", mork_python)
            .expect("failed to register embedded mork_python module");

        let code = CString::new(format!(
            r#"
import sys
sys.path.insert(0, r"{python_dir}")
import pure_functions
"#
        ))
        .expect("python bootstrap code contained an interior NUL byte");

        py.run(&code, None, None)
            .expect("failed to import pure_functions from fixed package path");
    });

    // Adapters MORK will call
    scope.add_func("add", adapter::add, FuncType::Pure);
    scope.add_func("clamp", adapter::clamp, FuncType::Pure);
    scope.add_func("py_add", adapter::py_add, FuncType::Pure);

}