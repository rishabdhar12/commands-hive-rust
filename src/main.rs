pub mod command;
use std::io::Write;

use rusqlite::Connection;
use command::Command;

use crate::sqlite::establish_connection;

pub mod sqlite;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match parse_args(&args) {
        Err(e) => println!("{}", e),
        Ok(arg) => {
            println!("arguments : {}", arg);
            match establish_connection() {
                Err(e) => {
                    println!("error establishing db connection : {}", e);
                    std::process::exit(1);
                }
                Ok(conn) => {
                    println!("db connection established");
                    execute(&arg, &conn);
                }
            }
        }
    }
}

fn execute(command: &str, conn: &Connection) {
    if command == "add" {
        match take_input() {
            Ok(command) => match command::Command::add_command(&command, &conn) {
                Ok(_) => {
                    println!("command added");
                    let _ = command::Command::list_command(conn);
                }
                Err(e) => {
                    println!("error while adding command: {}", e);
                }
            },
            Err(e) => println!("error: {}", e),
        }
    }

    if command == "list" {
        match command::Command::list_command(&conn) {
            Ok(_) => {},
            Err(e) => println!("error: {}", e),
        }
    }
}

fn take_input() -> Result<Command, std::io::Error> {
    let title = read_valid_input("enter title >")?;
    let desc = read_valid_input("enter description >")?;

    Ok(Command {
        id: None,
        title: title,
        description: desc,
    })
}

fn read_valid_input(prompt: &str) -> Result<String, std::io::Error> {
    loop {
        let mut input = String::new();

        print!("{}", prompt);
        std::io::stdout().flush()?;

        if std::io::stdin().read_line(&mut input)? == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "user cancelled the operation",
            ));
        }

        let input = input.trim();

        if input.is_empty() {
            println!("enter a word!");
        } else {
            return Ok(input.to_string());
        }
    }
}

fn parse_args(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("not enough args");
    }

    if args.len() > 2 {
        return Err("only one args");
    }

    let arg = args[1].clone();

    return Ok(arg);
}
