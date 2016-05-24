extern crate semiring;
extern crate fst;

use semiring::TropicalWeight;
use fst::{Std32Arc, Std64Arc};


fn main() {

    let a = Std32Arc::new(0, 0, TropicalWeight::new(Some(12.0000001)), 1);
    let b = Std64Arc::new(0, 0, TropicalWeight::new(Some(12.0000001)), 1);
    let c = b.clone();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
