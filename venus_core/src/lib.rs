#![deny(clippy::pedantic, clippy::all, clippy::nursery, clippy::perf)]

use pyo3::{
    Bound, PyResult, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};
use utils::etc::open_browser;
use utils::logging::debug;
use utils::path::{FilePath, get_user_home};

pub mod utils {
    pub mod etc;
    pub mod logging;
    pub mod path;
}

pub mod mime_types {
    pub mod bytes_read;
    pub mod extension;
    pub mod file_mime;
    pub mod magic;
    pub mod signatures;
}

#[pymodule]
/// # Errors
/// Add function failed.
pub fn venus_core(module: &Bound<PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(debug, module)?)?;
    module.add_function(wrap_pyfunction!(open_browser, module)?)?;
    module.add_function(wrap_pyfunction!(get_user_home, module)?)?;
    module.add_class::<FilePath>()?;

    Ok(())
}
