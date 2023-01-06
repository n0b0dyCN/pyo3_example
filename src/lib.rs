use pyo3::prelude::*;

#[pyclass]
pub struct A;

macro_rules! impl_method {
    ($fn:ident) => {
        pub fn $fn(&self) -> PyResult<()> {
            println!("{}", stringify!($fn));
            Ok(())
        }
    };
}

#[pymethods]
impl A {
    pub fn a(&self) -> PyResult<()> {
        println!("a");
        Ok(())
    }

    impl_method!(b);
    impl_method!(c);
}

#[pymodule]
fn pyo3_example(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<A>()?;
    Ok(())
}