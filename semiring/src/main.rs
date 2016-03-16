extern crate semiring;

use semiring::TropicalWeight;

fn main() {
    {
        let t = TropicalWeight::new(Some(12f64));
        let tt = TropicalWeight::new(Some(13f64));
        println!("{}", tt.is_member());
        println!("{:?} ⊕ {:?} = {:?}", t, tt, t + tt); 
        println!("{:?} ⊗ {:?} = {:?}", t, tt, t * tt); 
    }
    let t = TropicalWeight::new(Some(10.0f32));
    let tt = TropicalWeight::new(Some(13.0f32));
    println!("{}", tt.is_member());
    println!("{:?} ⊕ {:?} = {:?}", t, tt, t + tt); 
    println!("{:?} ⊗ {:?} = {:?}", t, tt, t * tt); 
}
