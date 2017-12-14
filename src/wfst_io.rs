// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

//! This module implements a wrapper and utilities for simplifying IO
//! of different supported types.
use std::any::TypeId;
use std::error::Error;

extern crate serde;
use self::serde::{Serialize, Deserialize};
extern crate bincode;


#[derive(Debug)]
pub struct IOError {
    pub message: String,
}

impl<T: Error> From<T> for IOError {
    fn from(e: T) -> IOError {
        IOError{message: format!("Format error: {}", e.description())}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IOWrapper {
    pub tid: String,
    pub data: Vec<u8>,
}

pub fn serialise<T: Serialize + 'static>(d: &T) -> Result<Vec<u8>, IOError> {
    let wrapped = IOWrapper{tid: format!("{:?}", TypeId::of::<T>()),
                            data: bincode::serialize(d, bincode::Infinite)?};
    Ok(bincode::serialize(&wrapped, bincode::Infinite)?)
}


pub fn deserialise_wrapper(d: &Vec<u8>) -> Result<IOWrapper, IOError> {
    Ok(bincode::deserialize(d)?)
}

pub fn deserialise<T: Deserialize + 'static>(d: &Vec<u8>) -> Result<T, IOError> {
    let wrapped: IOWrapper = bincode::deserialize(d)?;

    if wrapped.tid == format!("{:?}", TypeId::of::<T>()) {
        Ok(bincode::deserialize(&wrapped.data)?)
    } else {
        Err(IOError{message: format!("IO error: wrong type")})
    }
}

#[macro_export]
macro_rules! wfstio_autodeserialise_apply {
    ($buf:ident, $fst:ident, $e:expr) => { //expression -> Result<(), IOError>
        match wfst::wfst_io::deserialise_wrapper(&$buf) {
            Ok(w) => {
                if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::TropicalWeight<f64>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::TropicalWeight<f64>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::LogWeight<f64>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::LogWeight<f64>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::MinmaxWeight<f64>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::MinmaxWeight<f64>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::TropicalWeight<f32>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::TropicalWeight<f32>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::LogWeight<f32>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::LogWeight<f32>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::MinmaxWeight<f32>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::MinmaxWeight<f32>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else {
                    Err(IOError{message: format!("IO error: Fst type not recognised")})
                }
            },
            Err(e) => Err(IOError{message: e.message}),
        }
    }
}

#[macro_export]
macro_rules! wfstio_autodeserialise_apply_naturalless {
    ($buf:ident, $fst:ident, $e:expr) => { //expression -> Result<(), IOError>
        match wfst::wfst_io::deserialise_wrapper(&$buf) {
            Ok(w) => {
                if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::TropicalWeight<f64>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::TropicalWeight<f64>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::MinmaxWeight<f64>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::MinmaxWeight<f64>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::TropicalWeight<f32>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::TropicalWeight<f32>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else if w.tid == format!("{:?}", TypeId::of::<wfst::wfst_vec::VecFst<wfst::semiring::floatweight::MinmaxWeight<f32>>>()) {
                    match bincode::deserialize(&w.data) {
                        Ok(f) => {
                            let $fst: wfst::wfst_vec::VecFst<wfst::semiring::floatweight::MinmaxWeight<f32>> = f;
                            $e
                        },
                        Err(e) => Err(IOError{message: format!("{:?}", e)}),
                    }
                } else {
                    Err(IOError{message: format!("IO error: Fst type not recognised")})
                }
            },
            Err(e) => Err(IOError{message: e.message}),
        }
    }
}
