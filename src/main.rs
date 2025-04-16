#![allow(unused)]
mod color;
mod command;

use color::TerminalColor;
use std::io::Write;
use std::{fs, io, path::Path, process};

const SIGINT: i32 = 2;
type SignalHandler = extern "C" fn(i32);

extern "C" {
    fn signal(signum: i32, handler: SignalHandler) -> SignalHandler;
}

extern "C" fn handle_sigint(_: i32) {
    process::exit(0);
}

const PROMPT: &str = "❯ ";

fn main() {
    unsafe {
        // SAFETY: We are calling the `signal` function from libc, which is an external C function
        // that installs a signal handler for SIGINT (Ctrl+C). The function signature and the constant
        // `SIGINT` are correct for POSIX systems. Our handler function (`handle_sigint`) matches the
        // expected C signature: `extern "C" fn(i32)`, and performs only signal-safe operations
        // (a simple `println!` and `process::exit`).
        //
        // This usage is safe because:
        // - We do not access shared memory or mutable state from the signal handler.
        // - `process::exit` terminates immediately, so no further execution continues from the signal.
        // - We register the signal handler once, before any multi-threading starts.
        signal(SIGINT, handle_sigint);
    }

    let commands: std::collections::HashMap<&str, Box<dyn command::Command>> =
        std::collections::HashMap::from([
            ("exit", Box::new(command::Exit) as Box<dyn command::Command>),
            (
                "print",
                Box::new(command::Print) as Box<dyn command::Command>,
            ),
            ("ls", Box::new(command::List) as Box<dyn command::Command>),
        ]);

    loop {
        print!("{}", PROMPT.color().fg_green());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        let read_bytes = std::io::stdin().read_line(&mut input).unwrap();
        if read_bytes == 0 {
            break;
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command_name = &parts[0];
        let args = &parts[1..];
        let Some(command) = commands.get(command_name) else {
            println!("{}: command not found", command_name);
            continue;
        };
        let Err(err) = command.execute(args) else {
            continue;
        };
        println!("{err}");
    }
}
