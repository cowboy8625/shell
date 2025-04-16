#![allow(unused)]
use std::{fs, io, path::Path, process};

use crate::color::TerminalColor;

pub type CommandError = String;

pub type CommandResult = Result<(), CommandError>;

pub trait Command {
    fn execute(&self, args: &[&str]) -> CommandResult;
}

pub struct Exit;
impl Command for Exit {
    fn execute(&self, _args: &[&str]) -> CommandResult {
        process::exit(0);
    }
}

pub struct Print;
impl Command for Print {
    fn execute(&self, args: &[&str]) -> CommandResult {
        println!("{}", args.join(" "));
        Ok(())
    }
}

pub struct List;
impl Command for List {
    fn execute(&self, args: &[&str]) -> CommandResult {
        fn list_entries(path: &Path, args: &[&str]) -> io::Result<()> {
            let show_hidden = args
                .iter()
                .any(|arg| arg.starts_with("-") && arg.contains("a"));
            let recursive = args
                .iter()
                .any(|arg| arg.starts_with("-") && arg.contains("r"));
            let mut output = String::new();
            let list_of_dirs = fs::read_dir(path)?;
            let total = fs::read_dir(path)?.count();
            for (i, entry) in list_of_dirs.enumerate() {
                let entry = entry?;
                let path = entry.path();

                let starts_with_dot = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| s.starts_with('.'))
                    .unwrap_or(false);

                if !show_hidden && starts_with_dot {
                    continue;
                }

                let item = path.display().to_string();
                let value = item.strip_prefix("./").unwrap_or(item.as_str());

                if path.is_dir() {
                    output += &value.color().fg_blue().bold().to_string();
                } else {
                    output += &value.to_string();
                }

                if i < total - 1 {
                    output += " ";
                }

                if recursive && path.is_dir() {
                    list_entries(&path, args)?;
                }
            }
            println!("{}", output);
            Ok(())
        }

        let raw_path = args.iter().find(|s| !s.starts_with("-"));
        let path = Path::new(raw_path.map_or(".", |s| s));
        list_entries(path, args).map_err(|err| err.to_string())?;
        Ok(())
    }
}
