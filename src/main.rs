mod print;
mod sortable;
mod pitchclass;
use pitchclass::*;
mod ListFunctions;
use ListFunctions::*;
use std::env;
use print::Print;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
     match command.as_ref() {
         "pitchclass" => PitchClassSet::from(&args[2]).set.print(),
         _ => {
             println!("{} is not a valid command", command);
             std::process::exit(1);
         }
     }
    let a: Vec<i32> = vec![6,9,1,2,3];
    let b: Vec<Vec<i32>> = a.circular_permutations();
    println!("{:?}",a);
    println!("{:?}",b);
}
