use std::path::PathBuf;

pub enum Task {
    None,
    Cd(Option<PathBuf>),
    Exit,
    Status,
    Exe { name: String, args: Vec<String> },
}

impl<S: AsRef<str>> From<S> for Task {
    fn from(text: S) -> Self {
        let mut words = text.as_ref().split_whitespace();

        let first_word = match words.next() {
            Some(word) => word,
            None => return Task::None,
        };

        let mut remaining = words;

        match first_word {
            "cd"     => Task::Cd(remaining.next().map(PathBuf::from)),
            "exit"   => Task::Exit,
            "status" => Task::Status,
            text     => Task::Exe {
                name: text.to_string(),
                args: remaining.map(str::to_string).collect()
            }
        }
    }
}
