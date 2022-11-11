mod operations;
mod conversions;
mod pitch_class_set;
use operations::*;
use pitch_class_set::*;
use std::env;

fn main() {
    let pitchnames = String::from("C D F F# A");
    let pc = PitchClassSet::from(&pitchnames);
    pc.normal_order().print_names();
    pc.prime_form().print_names();


    // println!("{}",-3%10);
    // println!("{:?}",permuts);
    // println!("{:?}",scores);


    //let args: Vec<String> = env::args().collect();
    //let command = &args[1];
    // match command.as_ref() {
    //     "pitchclass" => PitchClassSet::from(&args[2]).set.print(),
    //     _ => {
    //         println!("{} is not a valid command", command);
    //         std::process::exit(1);
    //     }
    // }
}
