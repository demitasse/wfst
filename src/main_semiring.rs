extern crate wfst;

use wfst::semiring::Weight;
use wfst::semiring::TropicalWeight;

fn main() {
    {
        let t = TropicalWeight::new(Some(12.0f64));
        let tt = TropicalWeight::new(Some(12.5f64)).quantize(Some(1.0));
        println!("{}", tt.approx_eq(t, Some(1.0)));
        println!("{:?} ⊕ {:?} = {:?}", t, tt, t.plus(tt)); 
        println!("{:?} ⊗ {:?} = {:?}", t, tt, t.times(tt)); 
        println!("t.times(tt).divide(tt, None) ==> {:?}", t.times(tt).divide(tt, None)); 
    }
    let t: TropicalWeight<f32> = TropicalWeight::one().quantize(None);
    let tt = TropicalWeight::zero().quantize(None);
    println!("{}", tt.is_member());
    println!("{:?} ⊕ {:?} = {:?}", t, tt, t.plus(tt)); 
    println!("{:?} ⊗ {:?} = {:?}", t, tt, t.times(tt));
    println!("{:?}", t.times(tt).divide(tt, None)); 
}
