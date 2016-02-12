use std::option::Option;
use std::ops::Add;
use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct TropicalWeight<T: PartialOrd + Clone> {
    x: Option<T>
}

impl<T: PartialOrd + Clone> TropicalWeight<T> {
    pub fn new(val: Option<T>) -> TropicalWeight<T> {
        TropicalWeight {x: val}
    }

    pub fn is_member(&self) -> bool {
        //DEMIT: Revisit: rejects NaN but not negative infinity (as in openFST)
        self.x == self.x 
    }
}


impl<'a, T: PartialOrd + Clone> Add<&'a TropicalWeight<T>> for &'a TropicalWeight<T> {
    type Output = TropicalWeight<T>;
    
    fn add(self, rhs: &'a TropicalWeight<T>) -> TropicalWeight<T> {
        if (!self.is_member()) || (!rhs.is_member()) {
            TropicalWeight::new(None)
        } else if self.x < rhs.x {
            TropicalWeight::new(self.x.clone())
        } else {
            TropicalWeight::new(rhs.x.clone())
        }
    }
}
