#![feature(zero_one)]
use std::option::Option;
use std::cmp::PartialOrd;
//use std::num::{Zero, One};
use std::ops::{Add, Mul};

//Define tropical weight for type T, where:
//  -- The output type of adding two T's is also T
//  -- T has PartialOrd, Clone and Zero
//We give the struct "Copy semantics", i.e. it will be copied instead
// of "moved". We may want to revisit this decision, but in the
// meantime this is more in line with the behaviour of numeric types.
#[derive(Copy, Clone, Debug)]
pub struct TropicalWeight<T: Add<Output=T> + PartialOrd + Clone> {
    x: Option<T>
}

impl<T: Add<Output=T> + PartialOrd + Clone> TropicalWeight<T> {
    pub fn new(val: Option<T>) -> TropicalWeight<T> {
        TropicalWeight {x: val}
    }

    pub fn is_member(&self) -> bool {
        //DEMIT: Revisit? -- rejects NaN but not negative infinity (as in openFST)
        self.x == self.x 
    }
}

//ADD
impl<T: Add<Output=T> + PartialOrd + Clone> Add<TropicalWeight<T>> for TropicalWeight<T> {
    type Output = TropicalWeight<T>;
    
    fn add(self, rhs: TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else if self.x < rhs.x {
            TropicalWeight::new(self.x.clone())
        } else {
            TropicalWeight::new(rhs.x.clone())
        }
    }
}

//MUL
impl<T: Add<Output=T> + PartialOrd + Clone> Mul<TropicalWeight<T>> for TropicalWeight<T> {
    type Output = TropicalWeight<T>;
    
    fn mul(self, rhs: TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else {
            TropicalWeight::new(Some(self.x.clone().unwrap() + rhs.x.clone().unwrap()))
        }
    }
}

// //ZERO
// impl<T: Zero + Add<Output=T> + PartialOrd + Clone> Zero for TropicalWeight<T> {
//     fn zero() -> TropicalWeight<T> {
//         TropicalWeight::new(Some(T::infinity()))
//     }
// }

// //ONE
// impl<T: Zero + Add<Output=T> + PartialOrd + Clone> One for TropicalWeight<T> {
//     fn one() -> TropicalWeight<T> {
//         TropicalWeight::new(Some(T::zero()))
//     }
// }
