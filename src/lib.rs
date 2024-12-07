use nokhwa::{native_api_backend, query};
use pyo3::prelude::*;

#[pyfunction]
fn devices() -> PyResult<Vec<String>> {
    let backend = native_api_backend().ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to get backend")
    })?;

    let devices = query(backend).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Error querying devices: {:?}",
            e
        ))
    })?;

    let device_names: Vec<String> = devices.iter().map(|device| device.human_name()).collect();

    Ok(device_names)
}

#[pymodule]
fn camera_devices(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(devices, m)?)?;
    Ok(())
}
