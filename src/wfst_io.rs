// Author: Daniel van Niekerk <dvn.demitasse@gmail.com>
//
// Copyright 2016 The Department of Arts and Culture of the Government
// of South Africa
//
// See the "LICENCE" file for information on usage and redistribution
// of this file.

//! This module implements a wrapper for the possibility of seamless
//! IO of different supported types.
#![macro_escape]

use std::any::TypeId;
use std::error::Error;

extern crate serde;
use self::serde::{Serialize, Deserialize};
extern crate bincode;
use self::bincode::{serialize, deserialize, Infinite};

use super::*;
use super::semiring::floatweight::*;
use super::wfst_vec::*;

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
                            data: serialize(d, Infinite)?};
    Ok(serialize(&wrapped, Infinite)?)
}


pub fn deserialise_wrapper(d: &Vec<u8>) -> Result<IOWrapper, IOError> {
    Ok(deserialize(d)?)
}

pub fn deserialise<T: Deserialize + 'static>(d: &Vec<u8>) -> Result<T, IOError> {
    let wrapped: IOWrapper = deserialize(d)?;

    if wrapped.tid == format!("{:?}", TypeId::of::<T>()) {
        Ok(deserialize(&wrapped.data)?)
    } else {
        Err(IOError{message: format!("IO error: wrong type")})
    }
}

#[macro_export]
macro_rules! wfstio_autodeserialise_apply {
    ($buf:ident, $fst:ident, $e:expr) => {
        {
            let w = deserialise_wrapper(&$buf)?;
            if w.tid == format!("{:?}", TypeId::of::<VecFst<TropicalWeight<f64>>>()) {
                let $fst: VecFst<TropicalWeight<f64>> = deserialize(&w.data)?;
                Ok($e)
            } else if w.tid == format!("{:?}", TypeId::of::<VecFst<LogWeight<f64>>>()) {
                let $fst: VecFst<LogWeight<f64>> = deserialize(&w.data)?;
                Ok($e)
            } else if w.tid == format!("{:?}", TypeId::of::<VecFst<MinmaxWeight<f64>>>()) {
                let $fst: VecFst<MinmaxWeight<f64>> = deserialize(&w.data)?;
                Ok($e)
            } else if w.tid == format!("{:?}", TypeId::of::<VecFst<TropicalWeight<f32>>>()) {
                let $fst: VecFst<TropicalWeight<f32>> = deserialize(&w.data)?;
                Ok($e)
            } else if w.tid == format!("{:?}", TypeId::of::<VecFst<LogWeight<f32>>>()) {
                let $fst: VecFst<LogWeight<f32>> = deserialize(&w.data)?;
                Ok($e)
            } else if w.tid == format!("{:?}", TypeId::of::<VecFst<MinmaxWeight<f32>>>()) {
                let $fst: VecFst<MinmaxWeight<f32>> = deserialize(&w.data)?;
                Ok($e)
            } else {
                Err(IOError{message: format!("IO error: Fst type not recognised")})
            }
        }
    }
}


