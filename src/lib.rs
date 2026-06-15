use pyo3::prelude::*;

mod dlx;

#[pyfunction]
#[pyo3(signature = (columns, rows, clues=None))]
fn solve_dlx(
    columns: Vec<bool>,
    rows: Vec<Vec<u8>>,
    clues: Option<Vec<usize>>,
) -> PyResult<Vec<usize>> {
    let clues = clues.unwrap_or_default();

    let mut solver =
        dlx::DLX::new(columns, rows, clues);

    match solver.solve() {
        Some(solution) => Ok(solution),
        None => Ok(vec![]),
    }
}

#[pymodule]
fn rust_dlx_lib(
    m: &Bound<'_, PyModule>,
) -> PyResult<()> {
    m.add_function(
        wrap_pyfunction!(solve_dlx, m)?
    )?;

    Ok(())
}
