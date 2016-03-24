extern crate semiring;

use semiring::Weight;
use semiring::TropicalWeight;

fn main() {
    {
        let t = TropicalWeight::new(Some(12.0f64));
        let tt = TropicalWeight::new(Some(12.0001f64));
        println!("{}", tt.approx_eq(t));
        println!("{:?} ⊕ {:?} = {:?}", t, tt, t.plus(tt)); 
        println!("{:?} ⊗ {:?} = {:?}", t, tt, t.times(tt)); 
    }
    let t: TropicalWeight<f32> = TropicalWeight::one();
    let tt = TropicalWeight::zero();
    println!("{}", tt.is_member());
    println!("{:?} ⊕ {:?} = {:?}", t, tt, t.plus(tt)); 
    println!("{:?} ⊗ {:?} = {:?}", t, tt, t.times(tt)); 
}
