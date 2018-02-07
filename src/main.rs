mod task;
mod shell;

use std::io::{stdin, stdout, Write};

fn main() {
    let mut shell = shell::Shell::new();
    let mut user_input = String::new();

    loop {
        print!(": ");
        stdout().flush().unwrap();
        user_input.clear();

        stdin().read_line(&mut user_input).unwrap();

        let task = if user_input.is_empty() { // EOF
            task::Task::Exit
        } else {
            user_input.as_str().trim().parse().unwrap()
        };

        shell.run(task);
    }
}
