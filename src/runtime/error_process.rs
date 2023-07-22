use serde::Serialize;

use super::process::Process;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorProcess {
    pub message: String,
    pub stack: Option<String>,
}

// ErrorProcess factory
impl ErrorProcess {
    pub fn from_e<D: std::fmt::Display>(e: D) -> Self {
        let err: String = e.to_string();

        ErrorProcess {
            message: err,
            stack: None,
        }
    }

    pub fn process_in_use(proc: Process) -> Process {
        proc.make_error(ErrorProcess {
            message: "proses latar belakang sedang berjalan".into(),
            stack: None,
        })
    }
}
