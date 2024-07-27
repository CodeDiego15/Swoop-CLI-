use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use termion::raw::IntoRawMode;
use termion::{event::Key, input::TermRead};
use colored::*;

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(stdout);
    let mut command_buffer = String::new();

    write!(stdout, "{}> ", get_prompt()?)?;
    stdout.flush()?;

    for key in stdin.keys() {
        match key? {
            Key::Char('\n') => {
                let command = command_buffer.trim();
                if !command.is_empty() {
                    if command.starts_with("cd ") {
                        change_directory(&command[3..])?;
                    } else if command == "ls" {
                        list_directory_contents(&mut stdout)?;
                    } else {
                        execute_command(command, &mut stdout)?;
                    }
                }
                command_buffer.clear();
                write!(stdout, "\r\n{}> ", get_prompt()?)?;
            },
            Key::Char(c) => {
                command_buffer.push(c);
                write!(stdout, "{}", c)?;
            },
            Key::Backspace => {
                if command_buffer.pop().is_some() {
                    write!(stdout, "\x08 \x08")?;
                }
            },
            _ => {},
        }
        stdout.flush()?;
    }

    Ok(())
}

fn get_prompt() -> io::Result<String> {
    let current_dir = env::current_dir()?;
    let dir_name = current_dir.file_name().unwrap_or_default().to_str().unwrap_or("unknown");
    Ok(format!("{}", dir_name.blue()))
}

fn change_directory(path: &str) -> io::Result<()> {
    let new_path = if path == "~" {
        env::home_dir().unwrap_or_else(|| env::current_dir().unwrap())
    } else {
        std::path::PathBuf::from(path)
    };
    env::set_current_dir(new_path)?;
    Ok(())
}

fn list_directory_contents(stdout: &mut impl Write) -> io::Result<()> {
    let paths = fs::read_dir(env::current_dir()?)?;
    let mut contents = Vec::new();

    for path in paths {
        let path = path?.path();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        contents.push(file_name);
    }

    contents.sort();
    let output = contents.join(" - ");
    writeln!(stdout, "\r\n{}", output.green())?;

    Ok(())
}

fn execute_command(command: &str, stdout: &mut impl Write) -> io::Result<()> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }
    let (program, args) = parts.split_first().unwrap();

    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                stdout.write_all(&output.stdout)?;
            }
            if !output.stderr.is_empty() {
                writeln!(stdout, "{}", String::from_utf8_lossy(&output.stderr).red())?;
            }
        }
        Err(err) => {
            writeln!(stdout, "{}", format!("Command execution failed: {}", err).red())?;
        }
    }

    Ok(())
}
