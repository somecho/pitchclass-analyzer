use crate::conversions::*;
use crate::operations::Operations;
use std::collections::HashSet;

pub struct PitchClassSet {
    pitch_class_set: Vec<i32>,
}

pub fn wrap_modulo(a: &i32, b: &i32) -> i32 {
    (a % b + b) % b
}
impl PitchClassSet {
    pub fn set(&self) -> &Vec<i32> {
        &self.pitch_class_set
    }
    pub fn ordered(&self) -> Vec<i32> {
        self.pitch_class_set.ascending()
    }
    pub fn from_vec(input: &Vec<i32>) -> PitchClassSet {
        PitchClassSet {
            pitch_class_set: input.clone(),
        }
    }
    pub fn from_string(input: &String) -> PitchClassSet {
        let pitch_class_set = input
            .split(" ")
            .map(|pitch| name_to_pitchclass(pitch))
            .collect::<Vec<i32>>();
        PitchClassSet { pitch_class_set }
    }
    pub fn normal_order(&self) -> Vec<i32> {
        let permuts = self.pitch_class_set.remove_dup().ascending().shift_add_12();
        let scores = permuts
            .iter()
            .map(|s| s.distance_score())
            .collect::<Vec<i32>>();
        let min = scores.iter().min().unwrap();
        let pos = scores.iter().position(|s| s == min).unwrap();
        permuts[pos].iter().map(|pc| pc % 12).collect::<Vec<i32>>()
    }
    pub fn prime_form(&self) -> Vec<i32> {
        let normal = self.normal_order();
        normal
            .iter()
            .map(|pc| wrap_modulo(&(pc - normal[0]), &12))
            .collect::<Vec<i32>>()
    }
    pub fn inversion_eq(&self, other: &PitchClassSet) -> bool {
        if self.set().len() != other.set().len() {
            panic!("The pitchsets are not the same size.");
        }
        let a = self.set().ascending();
        let b = other.set().descending();
        let c = a
            .clone()
            .into_iter()
            .zip(b)
            .map(|(x, y)| x + y)
            .collect::<HashSet<i32>>();
        c.len() == 1
    }
    pub fn interval_vector(&self) -> Vec<i32> {
        let mut intervals: Vec<i32> = Vec::new();
        let size = self.pitch_class_set.len();
        for i in 0..size {
            for j in 0..i {
                let diff = match (self.pitch_class_set[i] - self.pitch_class_set[j]).abs() {
                    0 => 0,
                    1 | 11 => 1,
                    2 | 10 => 2,
                    3 | 9 => 3,
                    4 | 8 => 4,
                    5 | 7 => 5,
                    6 => 6,
                    _ => {
                        println!("error");
                        std::process::exit(1);
                    }
                };
                intervals.push(diff);
            }
        }
        let interval_vec = (0..6)
            .map(|i| {
                intervals
                    .clone()
                    .into_iter()
                    .filter(|ic| *ic == i + 1)
                    .collect::<Vec<i32>>()
                    .len() as i32
            })
            .collect::<Vec<i32>>();
        interval_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interval_vector() {
        let a = PitchClassSet::from_vec(&vec![0, 1, 3, 5, 6]);
        assert_eq!(a.interval_vector(), vec![2, 2, 2, 1, 2, 1]);
        let b = PitchClassSet::from_vec(&vec![0, 1, 2]);
        assert_eq!(b.interval_vector(), vec![2, 1, 0, 0, 0, 0]);
        let c = PitchClassSet::from_string(&String::from("C C# D#"));
        assert_eq!(c.interval_vector(), vec![1, 1, 1, 0, 0, 0]);
    }
    #[test]
    fn inversion_eq() {
        let a = PitchClassSet::from_vec(&vec![3, 4, 7, 10]);
        let b = PitchClassSet::from_vec(&vec![9, 8, 5, 2]);
        assert_eq!(a.inversion_eq(&b), true);
    }
}
