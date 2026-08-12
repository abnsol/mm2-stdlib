use crate::python::registry;
use eval_ffi::{EvalError, ExprSink, ExprSource, SourceItem};
use pyo3::prelude::*;
use pyo3::types::PyTuple;

/// Convert a Rust value to a Python object (extend as needed).
fn to_py(py: Python<'_>, v: impl IntoPy<PyObject>) -> PyObject {
    v.into_py(py)
}

fn write_text_result(text: &str, sink: &mut ExprSink) -> Result<(), EvalError> {
    println!("the result from python is {}", text);
    sink.write(SourceItem::Symbol(text.as_bytes().into()))
        .map_err(|_| EvalError::from("sink write failed"))
}

/// Write a Python result into the sink (minimal version).
fn py_to_sink(py: Python<'_>, obj: Bound<'_, PyAny>, sink: &mut ExprSink) -> Result<(), EvalError> {
    if let Ok(v) = obj.extract::<bool>() {
        let s = if v { "true" } else { "false" };
        write_text_result(s, sink)?;
        return Ok(());
    }
    if let Ok(v) = obj.extract::<i64>() {
        let num_str = v.to_string();
        write_text_result(&num_str, sink)?;
        return Ok(());
    }
    if let Ok(v) = obj.extract::<f64>() {
        let num_str = v.to_string();
        write_text_result(&num_str, sink)?;
        return Ok(());
    }
    if let Ok(v) = obj.extract::<String>() {
        write_text_result(&v, sink)?;
        return Ok(());
    }
    Err(EvalError::from("unsupported Python return type"))
}

fn py_err(e: PyErr) -> EvalError {
    let _ = e;
    EvalError::from("python error during pure call")
}


/// Typed wrappers: known arity + types → easy consume + call Python.
macro_rules! py_wrap {
    ($name:ident ($x:ident : $tx:ty)) => {
        pub extern "C" fn $name(
            expr: *mut ExprSource,
            sink: *mut ExprSink,
        ) -> Result<(), EvalError> {
            let expr = unsafe { &mut *expr };
            let sink = unsafe { &mut *sink };

            let items = expr
                .consume_head_check(stringify!($name).as_bytes())
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": bad head")))?;
            if items != 1 {
                return Err(EvalError::from(concat!(stringify!($name), " takes 1 argument")));
            }

            let $x = expr
                .consume::<$tx>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg1 type mismatch")))?;

            Python::with_gil(|py| {
                let func = registry::get(py, stringify!($name))
                    .ok_or_else(|| EvalError::from(concat!(stringify!($name), " not registered")))?;
                let args = PyTuple::new(py, [to_py(py, $x)])
                    .map_err(py_err)?;
                let result = func
                    .bind(py)
                    .call1(args)
                    .map_err(py_err)?;
                py_to_sink(py, result, sink)
            })
        }
    };

    ($name:ident ($x:ident : $tx:ty, $y:ident : $ty:ty)) => {
        pub extern "C" fn $name(
            expr: *mut ExprSource,
            sink: *mut ExprSink,
        ) -> Result<(), EvalError> {
            let expr = unsafe { &mut *expr };
            let sink = unsafe { &mut *sink };

            let items = expr
                .consume_head_check(stringify!($name).as_bytes())
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": bad head")))?;
            if items != 2 {
                return Err(EvalError::from(concat!(stringify!($name), " takes 2 arguments")));
            }

            let $x = expr
                .consume::<$tx>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg1 type mismatch")))?;
            let $y = expr
                .consume::<$ty>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg2 type mismatch")))?;

            Python::with_gil(|py| {
                let func = registry::get(py, stringify!($name))
                    .ok_or_else(|| EvalError::from(concat!(stringify!($name), " not registered")))?;
                let args = PyTuple::new(py, [to_py(py, $x), to_py(py, $y)])
                    .map_err(py_err)?;
                let result = func
                    .bind(py)
                    .call1(args)
                    .map_err(py_err)?;
                py_to_sink(py, result, sink)
            })
        }
    };

    ($name:ident ($x:ident : $tx:ty, $y:ident : $ty:ty, $z:ident : $tz:ty)) => {
        pub extern "C" fn $name(
            expr: *mut ExprSource,
            sink: *mut ExprSink,
        ) -> Result<(), EvalError> {
            let expr = unsafe { &mut *expr };
            let sink = unsafe { &mut *sink };

            let items = expr
                .consume_head_check(stringify!($name).as_bytes())
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": bad head")))?;
            if items != 3 {
                return Err(EvalError::from(concat!(stringify!($name), " takes 3 arguments")));
            }

            let $x = expr
                .consume::<$tx>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg1 type mismatch")))?;
            let $y = expr
                .consume::<$ty>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg2 type mismatch")))?;
            let $z = expr
                .consume::<$tz>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg3 type mismatch")))?;

            Python::with_gil(|py| {
                let func = registry::get(py, stringify!($name))
                    .ok_or_else(|| EvalError::from(concat!(stringify!($name), " not registered")))?;
                let args = PyTuple::new(py, [to_py(py, $x), to_py(py, $y), to_py(py, $z)])
                    .map_err(py_err)?;
                let result = func
                    .bind(py)
                    .call1(args)
                    .map_err(py_err)?;
                py_to_sink(py, result, sink)
            })
        }
    };

    ($name:ident ($x:ident : $tx:ty, $y:ident : $ty:ty, $z:ident : $tz:ty, $w:ident : $tw:ty)) => {
        pub extern "C" fn $name(
            expr: *mut ExprSource,
            sink: *mut ExprSink,
        ) -> Result<(), EvalError> {
            let expr = unsafe { &mut *expr };
            let sink = unsafe { &mut *sink };

            let items = expr
                .consume_head_check(stringify!($name).as_bytes())
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": bad head")))?;
            if items != 4 {
                return Err(EvalError::from(concat!(stringify!($name), " takes 4 arguments")));
            }

            let $x = expr
                .consume::<$tx>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg1 type mismatch")))?;
            let $y = expr
                .consume::<$ty>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg2 type mismatch")))?;
            let $z = expr
                .consume::<$tz>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg3 type mismatch")))?;
            let $w = expr
                .consume::<$tw>()
                .map_err(|_| EvalError::from(concat!(stringify!($name), ": arg4 type mismatch")))?;

            Python::with_gil(|py| {
                let func = registry::get(py, stringify!($name))
                    .ok_or_else(|| EvalError::from(concat!(stringify!($name), " not registered")))?;
                let args = PyTuple::new(
                    py,
                    [to_py(py, $x), to_py(py, $y), to_py(py, $z), to_py(py, $w)],
                )
                .map_err(py_err)?;
                let result = func
                    .bind(py)
                    .call1(args)
                    .map_err(py_err)?;
                py_to_sink(py, result, sink)
            })
        }
    };
}

py_wrap!(add(x: f32, y: f32));
py_wrap!(clamp(x: i64, lo: i64, hi: i64));
py_wrap!(py_add(x: f32, y: f32));