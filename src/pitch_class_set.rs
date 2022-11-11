use crate::operations::Operations;
use crate::conversions::*;

pub struct PitchClassSet {
    pitch_class_set: Vec<i32>
}

pub fn wrap_modulo(a:&i32,b:&i32)->i32{
    (a%b+b)%b
}
impl PitchClassSet {
    pub fn set(&self)->&Vec<i32>{
        &self.pitch_class_set
    }
    pub fn from(input: &String)->PitchClassSet {
        let pitch_class_set = input.split(" ")
        .map(|pitch| name_to_pitchclass(pitch))
        .collect::<Vec<i32>>();
        PitchClassSet {
            pitch_class_set
        }
    }
    pub fn normal_order(&self)->Vec<i32>{
        let permuts = self.pitch_class_set.remove_dup().ascending().shift_add_12();
        let scores = permuts.iter().map(|s|s.distance_score()).collect::<Vec<i32>>();
        let min = scores.iter().min().unwrap();
        let  pos = scores.iter().position(|s|s==min).unwrap();
        permuts[pos].iter().map(|pc|pc%12).collect::<Vec<i32>>()
    }
    pub fn prime_form(&self)->Vec<i32>{
        let normal = self.normal_order();
        normal.iter().map(|pc|wrap_modulo(&(pc-normal[0]),&12)).collect::<Vec<i32>>()
    }
}