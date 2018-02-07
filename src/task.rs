use std::vec::Vec;
use std::path::PathBuf;
use std::ffi::OsString;
use std::str::FromStr;

pub enum Task {
    None,
    Cd(Option<PathBuf>),
    Exit,
    Status,
    Exe { name: OsString, args: Vec<OsString> },
}

impl FromStr for Task {
    type Err = !;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut words = source.split_whitespace();

        let instance = if let Some(word) = words.next() {
            match word {
                "cd"     => Task::Cd(words.next().map(PathBuf::from)),
                "exit"   => Task::Exit,
                "status" => Task::Status,
                name     => Task::Exe {
                    name: OsString::from(name),
                    args: words.map(OsString::from).collect()
                }
            }
        } else {
            Task::None
        };

        Ok(instance)
    }
}
