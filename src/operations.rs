use std::collections::HashSet;
use std::iter::zip;
use crate::pitch_class_set::wrap_modulo;

use crate::conversions::pitchclass_to_name;
pub trait Operations {
    fn circular_permutations(&self) -> Vec<Vec<i32>>;
    fn ascending(&self) -> Vec<i32>;
    fn remove_dup(&self) -> Vec<i32>;
    fn shift_add_12(&self) -> Vec<Vec<i32>>;
    fn distance_score(&self) -> i32;
    fn print(&self);
    fn print_names(&self);
    fn inverse(&self)->Vec<i32>;
    fn transpose(&self,t:i32)->Vec<i32>;
    fn descending(&self)->Vec<i32>;
    fn as_reversed(&self)->Vec<i32>;
}

impl Operations for Vec<i32> {
    fn circular_permutations(&self) -> Vec<Vec<i32>> {
        let len = self.len() as i32;
        (0..len)
            .map(|i| (0..len).map(move |j| (i + j) % len))
            .map(|indices| indices.map(|i| self[i as usize]).collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()
    }
    fn shift_add_12(&self) -> Vec<Vec<i32>> {
        let permutations = self.circular_permutations();
        let order = (0..self.len()).collect::<Vec<usize>>();
        zip(permutations, order)
            .map(|(p, i)| {
                let mut new_permut = p.clone();
                for j in 0..i {
                    new_permut[p.len() - j - 1] += 12;
                }
                new_permut
            })
            .collect::<Vec<Vec<i32>>>()
    }
    fn distance_score(&self) -> i32 {
        self.iter().map(|p| (p - self[0]).abs()).sum()
    }
    fn transpose(&self,t:i32)->Vec<i32>{
        self.iter().map(|pc|wrap_modulo(&(pc+t),&12)).collect::<Vec<i32>>()

    }
    fn inverse(&self)->Vec<i32> {
        self.iter().map(|pc|(12-pc)%12).collect::<Vec<i32>>()
    }
    fn ascending(&self) -> Vec<i32> {
        let mut out = self.clone();
        out.sort();
        out
    }
    fn descending(&self)->Vec<i32> {
        self.ascending().as_reversed()
    }
    fn remove_dup(&self) -> Vec<i32> {
        self.clone().into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>()
    }
    fn as_reversed(&self)->Vec<i32>{
        self.clone().into_iter().rev().collect()
    }

    fn print(&self) {
        for i in self {
            print!("{} ", i);
        }
        print!("\n")
    }
    fn print_names(&self){
        for pc in self {
            print!("{} ",pitchclass_to_name(pc));
        }
        print!("\n ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ascending(){
        let a = vec![8,5,4,7,9,2,3,6,1];
        let b = vec![1,2,3,4,5,6,7,8,9];
        assert_eq!(a.ascending(),b);
    }
    #[test]
    fn remove_dup(){
        let a = vec![1,1,3,3,1,2,2,4,5];
        let b = vec![1,2,3,4,5];
        assert_eq!(a.remove_dup().ascending(),b);
    }
    #[test]
    fn inverse(){
        let a = vec![0,1,2,3,4,5,6,7,8,9,10,11];
        let b = vec![0,11,10,9,8,7,6,5,4,3,2,1];
        assert_eq!(a.inverse(),b);
    }
}
