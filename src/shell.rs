use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process;
use std::error::Error;

use super::task;

pub struct Shell {
    prev_status: Option<process::ExitStatus>,
    homedir: PathBuf,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            prev_status: None,
            homedir: env::home_dir().unwrap_or(PathBuf::from("/")),
        }
    }

    fn cd(&self, opt_path: Option<PathBuf>) {
        let path = opt_path.as_ref().unwrap_or(&self.homedir);

        if let Err(e) = env::set_current_dir(path) {
            println!("Error changing directory: {:?}", e.kind());
        }
    }

    fn status(&self) {
        if let Some(status) = self.prev_status {
            println!("{}", status.code().unwrap());
        } else {
            println!("No previous status code");
        }
    }

    fn execute(&mut self, name: OsString, args: Vec<OsString>) {
        let spawn_result = process::Command::new(name).args(args).spawn();

        let mut child = match spawn_result {
            Ok(child) => child,
            Err(e) => return println!("Error executing: {}", e.description())
        };

        self.prev_status = child.wait().ok(); // Child -> Result -> Option
    }

    pub fn run(&mut self, task: task::Task) {
        use task::Task::*;
        match task {
            None     => (),
            Exit     => process::exit(0),
            Cd(path) => self.cd(path),
            Status   => self.status(),
            Exe { name, args } => self.execute(name, args),
        };
    }
}
