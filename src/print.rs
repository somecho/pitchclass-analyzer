pub trait Print {
    fn print(&self);
}

impl Print for Vec<i32> {
    fn print(&self) {
        for i in self {
            print!("{} ", i);
        }
        print!("\n")
    }
}