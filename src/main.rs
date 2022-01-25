use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = read_params(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    let result = factorial(num);
    println!("Factorial of {} is {}", num, result);
}

fn read_params(args: &[String]) -> Result<u64, &str> {
    if args.len() < 2 {
        return Err("Missing number parameter");
    }

    match args[1].trim().parse::<u64>() {
        Ok(num) => {
            if num > 20 {
                return Err("Number too big for factorial calculation");
            } 
            return Ok(num);
        },
        Err(_) => return Err("Parsing int error")
    }
}

fn factorial(num: u64) -> u64 {
    if num == 1 {
        return 1;
    } 

    return num * factorial(num - 1);
}