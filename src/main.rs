mod print;
mod sortable;
mod pitchclass;
use pitchclass::*;
use std::env;
use print::Print;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    // match command.as_ref() {
    //     "pitchclass" => PitchClassSet::from(&args[2]).set.print(),
    //     _ => {
    //         println!("{} is not a valid command", command);
    //         std::process::exit(1);
    //     }
    // }
    let a = vec![1,2,3,4,5];
    let b = a.clone().into_iter().map(|i|{
        (0..a.len() as i32).map(|j|{
            (j+i)%(a.len() as i32)
        }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    println!("{:?}",b);
}
