use std::fmt;
trait Dumpable {
    fn dump(&self);
}
impl<T: fmt::Display> Dumpable for Vec<T> {
    fn dump(&self) {
        for (i, x) in self.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", x);
        }
        println!();
    }
}

// --------------------

#[allow(unused_imports)]
use proconio::{fastout, input};

#[allow(non_snake_case)]
// #[fastout]
fn main() {
    input! {
        
    }
    let mut ans = 0;
    println!("{}", ans);
}
