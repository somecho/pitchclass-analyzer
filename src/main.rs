mod operations;
mod conversions;
mod pitch_class_set;
use operations::*;
use pitch_class_set::*;
use std::env;

fn main() {
    let pc = PitchClassSet::from_vec(&vec![3,4,7,10]);
    let pc2 = PitchClassSet::from_vec(&pc.set().inverse());
    pc.set().print();
    pc2.set().transpose(11).print();

    

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
