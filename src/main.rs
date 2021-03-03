extern crate clap;
use clap::{Arg, App};

fn calculate_number(input: isize) -> isize {
    match (input % 2) == 0 {
        true => input / 2,
        false => input * 3 + 1
    }
}

fn main() {
    let matches = App::new("collatz")
                          .version("v0.1")
                          .author("Jeroen Knoops <jeroen.knoops@gmail.com>")
                          .about("calculates collatz number given an input")
                          .arg(Arg::with_name("NUMBER")
                               .help("Sets number for which the collatz number is calculated... guess the answer ;)")
                               .required(true)
                               .index(1))
                          .get_matches();

    let number = matches.value_of("NUMBER").unwrap().parse::<isize>().unwrap();
    println!("Using number: {}", number);
    let mut result = number;
    let mut age = 0;
    while result > 1 {
        age += 1;
        result = calculate_number(result);
        println!("Result: {}: {}", age, result);
    }

}
