use std::collections::HashSet;
pub trait Sortable {
    fn ascending(&self) -> Vec<i32>;
    fn remove_dup(self)->Vec<i32>;
    fn circular_permutations(&self)->Vec<Vec<i32>>;
}
impl Sortable for Vec<i32> {
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
    fn circular_permutations(&self)->Vec<Vec<i32>>{
        todo!();
    }
}
