use pyo3::{prelude::*, types::IntoPyDict};
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
    env::set_var("PYTHONPATH", "./:venv");
    Python::with_gil(|py| -> PyResult<Vec<String>> {
        let sys = py.import("sys")?;

        let python_lib = py.import("python-lib")?;
        let locals = Some(python_lib.dict());

        sys.setattr("stdout", LoggingStdout::default().into_py(py))?;
        let result = match py.eval(cmd, None, locals) {
            Ok(result) => result,
            Err(e) => return Ok(vec![e.to_string(), String::new()]),
        };
        println!("{}", result);
        let get_stdout: Py<PyAny> = sys.getattr("stdout")?.getattr("get_stdout")?.into();
        let mut stdout: Vec<String> = get_stdout.call0(py)?.extract(py)?;
        stdout.pop();
        if !result.is_none() {
            stdout.push(result.to_string());
        }
        stdout.push(String::new());
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
