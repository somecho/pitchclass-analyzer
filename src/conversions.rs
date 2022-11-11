
pub fn name_to_pitchclass(name: &str) -> i32 {
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
pub fn pitchclass_to_name(pc: &i32)->&str{
    match pc {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        11 => "B",
        _ => {
            println!("{} is not a valid pitch class.",pc);
            std::process::exit(1);
        }
    }

}