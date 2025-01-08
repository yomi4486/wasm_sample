use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
fn main() -> Result<(), PyErr> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let code = "def add(a, b): return a + b";
    let locals = [("__name__", "__main__")].into_py_dict(py);
    py.run(code, None, Some(locals))?;

    let add_fn = locals.get_item("add").unwrap();
    let result: i32 = add_fn.call1((3, 5))?.extract()?;

    println!("Result: {}", result);

    Ok(())
}