extern crate num;
use num::Float;
use std::option::Option;

pub trait Weight {
    fn is_member(&self) -> bool;
    fn plus(self, rhs: Self) -> Self;
    fn times(self, rhs: Self) -> Self;
    // fn zero() -> Self;
    // fn one() -> Self;
}

//We give the struct "Copy semantics", i.e. it will be copied instead
// of "moved". We may want to revisit this decision, but in the
// meantime this is more in line with the behaviour of numeric types.
#[derive(Copy, Clone, Debug)]
pub struct TropicalWeight<T: Float> {
    val: Option<T>
}

impl<T: Float> TropicalWeight<T> {
    pub fn new(val: Option<T>) -> TropicalWeight<T> {
        TropicalWeight {val: val}
    }
}

impl<T: Float> Weight for TropicalWeight<T> {
    fn plus(self, rhs: TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else if self.val < rhs.val {
            TropicalWeight::new(self.val)
        } else {
            TropicalWeight::new(rhs.val)
        }        
    }

    fn times(self, rhs: TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else {
            TropicalWeight::new(Some(self.val.unwrap() + rhs.val.unwrap()))
        }
    }

    fn is_member(&self) -> bool {
        //DEMIT: Revisit? -- rejects NaN but not negative infinity (as in openFST)
        self.val == self.val 
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
