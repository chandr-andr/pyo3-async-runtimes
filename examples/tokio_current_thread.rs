use pyo3::prelude::*;

#[pyo3_asyncio::tokio::main(flavor = "current_thread")]
async fn main() -> PyResult<()> {
    let fut = Python::with_gil(|py| {
        let asyncio = py.import("asyncio")?;

        // convert asyncio.sleep into a Rust Future
        pyo3_asyncio::into_future(
            pyo3_asyncio::tokio::task_event_loop().unwrap().as_ref(py),
            asyncio.call_method1("sleep", (1.into_py(py),))?,
        )
    })?;

    println!("sleeping for 1s");
    fut.await?;
    println!("done");

    Ok(())
}
