//use crate::sortable::*;

pub struct PitchClassSet {
    pub set: Vec<i32>
}

impl PitchClassSet {
    pub fn from(input: &String)->PitchClassSet {
        let set = input.split(" ")
        .map(|pitch| name_to_pitchclass(pitch))
        .collect::<Vec<i32>>();
        PitchClassSet {
            set
        }
    }
}

fn name_to_pitchclass(name: &str) -> i32 {
    match name {
        "B#" | "C" | "Dbb" => 0,
        "B##" | "C#" | "Db" => 1,
        "C##" | "D" | "Ebb" => 2,
        "D#" | "Eb" | "Fbb" => 3,
        "D##" | "E" | "Fb" => 4,
        "E#" | "F" | "Gbb" => 5,
        "E##" | "F#" | "Gb" => 6,
        "F##" | "G" | "Abb" => 7,
        "G#" | "Ab" => 8,
        "G##" | "A" | "Bbb" => 9,
        "A#" | "Bb" | "Cbb" => 10,
        "A##" | "B" | "Cb" => 11,
        _ => {
            println!("{} is not a valid pitch.", name);
            std::process::exit(1);
        }
    }
}
