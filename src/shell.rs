use std::{
    borrow::Cow,
    env,
    io,
    ffi::OsStr,
    path::Path,
    process::{Command, exit, ExitStatus},
};

#[cfg(windows)]
const ROOT_DIR: &str = "C:\\";

#[cfg(unix)]
const ROOT_DIR: &str = "/";

use crate::task::Task;

pub struct Shell {
    prev_status: Option<ExitStatus>,
    homedir: Cow<'static, Path>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            prev_status: None,
            homedir: dirs::home_dir()
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed(Path::new(ROOT_DIR))),
        }
    }

    pub fn run(&mut self, task: Task) {
        use Task::*;
        match task {
            None   => {}
            Exit   => exit(0),

            Status => {
                let code = self.prev_status
                    .and_then(|status| status.code())
                    .unwrap_or(0);

                println!("{}", code);
            }

            Cd(path) => self.cd(path)
                .unwrap_or_else(|err| eprintln!("Error changing directory: {}", err)),

            Exe { name, args } => self.execute(name, args)
                .unwrap_or_else(|err| eprintln!("Error executing: {}", err)),
        }
    }

    fn cd<P: AsRef<Path>>(&self, arg: Option<P>) -> io::Result<()> {
        let path: &Path = match &arg {
            Some(path) => P::as_ref(path),
            None => Cow::as_ref(&self.homedir),
        };

        env::set_current_dir(path)
    }

    fn execute(
        &mut self,
        name: impl AsRef<OsStr>,
        args: impl IntoIterator<Item=impl AsRef<OsStr>>,
    ) -> io::Result<()>
    {
        let mut child = Command::new(name).args(args).spawn()?;
        let exit_status = child.wait()?;
        self.prev_status = Some(exit_status);

        Ok(())
    }
}
