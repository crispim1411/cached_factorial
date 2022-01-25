use std::process;
use std::io;

use cached_factorial::{ read_params, Cacher };

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

fn factorial(num: u64) -> u64 {
    if num == 1 {
        return 1;
    } 

    return num * factorial(num - 1);
}