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
                Ok(_) => println!("db connection established"),
            }
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
