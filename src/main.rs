extern crate semiring;

use semiring::TropicalWeight;
use std::f64;

fn main() {
    {
        let t = TropicalWeight::new(Some(12.0f64));
        let tt = TropicalWeight::new(Some(f64::NAN));
        println!("{}", tt.is_member());
        let ttt = &t + &tt;
        println!("{:?}\t{:?}\t{:?}", t, tt, ttt);
    }
    let t = TropicalWeight::new(Some(12.0f32));
    let tt = TropicalWeight::new(Some(13.0f32));
    let ttt = &t + &tt;
    println!("{:?}", ttt);
}
