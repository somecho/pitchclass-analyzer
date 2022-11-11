use std::collections::HashSet;
pub trait ListFunctions {
    fn circular_permutations(&self)->Vec<Vec<i32>>;
    fn ascending(&self) -> Vec<i32>;
    fn remove_dup(self)->Vec<i32>;
}

impl ListFunctions for Vec<i32> {
    fn circular_permutations(&self)->Vec<Vec<i32>>{
        let len = self.len() as i32;
        (0..len).map(|i|{
            (0..len).map(move |j|{
                (i+j) % len
            })
        }).map(|indices|{
            indices.map(|i|{
                self[i as usize]
            }).collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>()
    }
    fn ascending(&self) -> Vec<i32> {
        let mut out = self.clone();
        out.sort();
        out
    }
    fn remove_dup(self)->Vec<i32>{
        self.into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>()
    }
}
