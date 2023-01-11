use pyo3::prelude::*;
use std::{env, str};

#[pyclass]
#[derive(Default)]
struct LoggingStdout {
    stdout: String,
}

#[pymethods]
impl LoggingStdout {
    fn write(&mut self, data: &str) {
        self.stdout.push_str(data);
    }
    fn get_stdout(&self) -> Vec<String> {
        self.stdout.split('\n').map(|s| s.to_string()).collect()
    }
}

pub fn process(cmd: &str) -> Vec<String> {
    env::set_var("PYTHONPATH", "python-lib:venv");
    Python::with_gil(|py| -> PyResult<Vec<String>> {
        let sys = py.import("sys")?;
        // PyModule::from_code(py, include_str!("../python-lib/main.py"), "main.py", "python-lib")?;
        sys.setattr("stdout", LoggingStdout::default().into_py(py))?;
        match py.run(cmd, None, None) {
            Ok(_) => (),
            Err(e) => return Ok(vec![e.to_string(), String::new()]),
        }
        let get_stdout: Py<PyAny> = sys.getattr("stdout")?.getattr("get_stdout")?.into();
        let stdout: Vec<String> = get_stdout.call0(py)?.extract(py)?;
        dbg!(stdout.clone());
        Ok(stdout)
    })
    .unwrap()
}

// #[derive(Default)]
// pub struct Processor {

// }

// impl Processor {
//     pub fn process() {

//     }
// }
