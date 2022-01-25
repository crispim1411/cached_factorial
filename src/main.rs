use std::process;
use std::io;

use cached_factorial::Cacher;

fn main() {
    let mut cache_result = Cacher::new(factorial);
    loop {
        println!("Type a number for factorial calculation");
        let mut arg = String::new();
        io::stdin()
            .read_line(&mut arg)
            .expect("Failed to read line");

        let num = read_params(&arg).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });

        let result = cache_result.value(num);
        println!("Factorial of {} is {}\n", num, result);
    }
}

fn read_params(arg: &String) -> Result<u64, &str> {
    match arg.trim().parse::<u64>() {
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