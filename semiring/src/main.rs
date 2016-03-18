extern crate semiring;

use semiring::Weight;
use semiring::TropicalWeight;

fn main() {
    {
        let t = TropicalWeight::new(Some(12f64));
        let tt = TropicalWeight::one();
        println!("{}", tt.is_member());
        println!("{:?} ⊕ {:?} = {:?}", t, tt, t.plus(tt)); 
        println!("{:?} ⊗ {:?} = {:?}", t, tt, t.times(tt)); 
    }
    let t = TropicalWeight::new(Some(10.0f32));
    let tt = TropicalWeight::zero();
    println!("{}", tt.is_member());
    println!("{:?} ⊕ {:?} = {:?}", t, tt, t.plus(tt)); 
    println!("{:?} ⊗ {:?} = {:?}", t, tt, t.times(tt)); 
}
