extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, StoreOption};

extern crate wfst;
use wfst::semiring::float::Float;
use wfst::semiring::floatweight::{FloatWeight, TropicalWeight, LogWeight, MinmaxWeight};
use wfst::wfst_vec::{VecFst};
use wfst::{MutableFst};

use std::fmt::Debug;
use std::str::FromStr;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write, BufRead};
use std::process::exit;
use std::collections::HashMap;

extern crate rustc_serialize;
use rustc_serialize::{Encodable, json};
extern crate bincode;
use bincode::SizeLimit;
use bincode::rustc_serialize::encode;

const EXCODE_BADINPUT: i32 = 2;

#[derive(Debug)]
pub struct FileReadError {
    pub message: String,
}

impl From<std::num::ParseIntError> for FileReadError {
    fn from(e: std::num::ParseIntError) -> FileReadError {
        FileReadError{message: format!("Format error: {}", e.description())}
    }
}

impl From<std::io::Error> for FileReadError {
    fn from(e: std::io::Error) -> FileReadError {
        FileReadError{message: format!("IO error: {}", e.description())}
    }
}

fn load_symtab(symfn: Option<String>) -> Result<Option<Vec<String>>, FileReadError> {
    if let Some(tempfn) = symfn {
        //Slurp lines to parsed fields (DEMITMEM)
        let mut fh = File::open(tempfn)?;
        let mut s = String::new();
        fh.read_to_string(&mut s)?;
        let mut entries = Vec::new();
        let mut n: usize = 0;
        for line in s.lines() {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() != 2 {
                return Err(FileReadError{message: format!("Format error: wrong number of fields")})
            }
            let entry = (fields[1].parse::<usize>()?, String::from(fields[0]));
            if entry.0 > n {
                n = entry.0;
            }
            entries.push(entry)
        }
        let mut syms = Vec::with_capacity(n+1);
        for _ in 0..n+1 {
            syms.push(String::new());
        }
        for entry in entries {
            syms[entry.0] = entry.1;
        }
        Ok(Some(syms))
    } else {
        Ok(None)
    }
}

fn input<T: Float<T> + FromStr, W: FloatWeight<T>, F: MutableFst<W>>(mut fst: F, isymfn: Option<String>, osymfn: Option<String>) -> F
    where T::Err: Debug {
    match load_symtab(isymfn) {
        Ok(d) => if let Some(syms) = d {
            //println!("{:?}", syms);
            fst.set_isyms(syms);
        },
        Err(e) => {println!("Error loading file: {:?}", e.message);
                   exit(EXCODE_BADINPUT);
        },
    };
    match load_symtab(osymfn) {
        Ok(d) => if let Some(syms) = d {
            //println!("{:?}", syms);
            fst.set_osyms(syms);
        },
        Err(e) => {println!("Error loading file: {:?}", e.message);
                   exit(EXCODE_BADINPUT);
        },
    };

    ////Parse input from STDIN
    let stdin = io::stdin();
    let handle = stdin.lock();    //for (fast) buffered input
    let mut is_start = true;
    let mut arcs = Vec::new();
    let mut nstates: usize = 0;
    let mut startstate: usize = 0;
    let mut finalstates = HashMap::<usize, _>::new();
    for l in handle.lines() {     //strips newlines
        let line: String = l.unwrap();
        //println!("{}", line);
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let (is_final, weight) = match fields.len() {
            1 => (true, W::zero()),
            2 => (true, W::new(Some(fields[1].parse().expect("Format error: could not parse float weight")))),
            4 => (false, W::zero()),
            5 => (false, W::new(Some(fields[4].parse().expect("Format error: could not parse float weight")))),
            _ => panic!("Format error: wrong number of fields"),
        };
        //println!("{:?} {:?} {:?}", is_start, is_final, weight);
        if !is_final {
            let src = fields[0].parse().expect("Format error: could not parse src state <usize>");
            let tgt = fields[1].parse().expect("Format error: could not parse tgt state <usize>");
            if src > nstates { nstates = src };
            if tgt > nstates { nstates = tgt };
            let ilabel = fields[2].parse().expect("Format error: could not parse ilabel <usize>");
            let olabel = fields[3].parse().expect("Format error: could not parse olabel <usize>");
            arcs.push((src, tgt, ilabel, olabel, weight));
        } else {
            let tgt = fields[0].parse().expect("Format error: could not parse final tgt state <usize>");
            finalstates.insert(tgt, weight);
            if tgt > nstates { nstates = tgt };
        }            
        if is_start {
            startstate = fields[0].parse().expect("Format error: could not parse src state <usize>");
            is_start = false;
        }
    }
    nstates += 1;
    //println!("nstates: {}", nstates);
    //println!("startstate: {}", startstate);
    //println!("finalstates: {:?}", finalstates);
    //println!("{:?}", arcs);
    //println!("{:?}", fst);

    ////Construct FST
    for i in 0..nstates {
        if finalstates.contains_key(&i) {
            fst.add_state(finalstates.remove(&i).unwrap());
        } else {
            fst.add_state(W::zero());
        }
    }
    fst.set_start(startstate);
    for arc in arcs {
        fst.add_arc(arc.0, arc.1, arc.2, arc.3, arc.4);
    }
    fst
}

fn output<T: Encodable>(t: &T, bin: bool) {
    ////Output on STDOUT
    if !bin {    
        let encoded = json::encode(t).unwrap();
        println!("{}", encoded);
    } else {
        let encoded = encode(t, SizeLimit::Infinite).unwrap();
        io::stdout().write(&*encoded).ok();
    }
}


fn main() {
    //Setup defaults and parse args
    let mut p64 = false;
    let mut wtype: Option<usize> = None;
    let mut binfile = false;
    let mut isymfn: Option<String> = None;
    let mut osymfn: Option<String> = None;
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Creates native FSTs from simple text format.");
        ap.refer(&mut isymfn)
            .add_option(&["-i", "--isymfn"], StoreOption, "Input label symbol table filename");
        ap.refer(&mut osymfn)
            .add_option(&["-o", "--osymfn"], StoreOption, "Output label symbol table filename");
        ap.refer(&mut wtype)
            .add_option(&["-w", "--wtype"], StoreOption, "Select the weight type (semiring) from (0: Tropical, 1: Log, 2: Minmax -- default is 0)");
        ap.refer(&mut p64)
            .add_option(&["-p", "--precision"], StoreTrue, "Use 64-bit precision for weights (default is 32-bit)");
        ap.refer(&mut binfile)
            .add_option(&["-b", "--bin"], StoreTrue, "Output file in binary format (default is json)");
        ap.parse_args_or_exit();
    }

    let semiring = wtype.unwrap_or(0);
    //DEMIT: Probably only the precision makes a difference to the output at the moment...
    if p64 {
        match semiring {
            0 => output(&input(VecFst::<TropicalWeight<f64>>::new(), isymfn, osymfn), binfile),
            1 => output(&input(VecFst::<LogWeight<f64>>::new(), isymfn, osymfn), binfile),
            2 => output(&input(VecFst::<MinmaxWeight<f64>>::new(), isymfn, osymfn), binfile),
            _ => { println!("Invalid weight type: {:?}", semiring);
                   exit(EXCODE_BADINPUT);
            },
        };
    } else {
        match semiring {
            0 => output(&input(VecFst::<TropicalWeight<f32>>::new(), isymfn, osymfn), binfile),
            1 => output(&input(VecFst::<LogWeight<f32>>::new(), isymfn, osymfn), binfile),
            2 => output(&input(VecFst::<MinmaxWeight<f32>>::new(), isymfn, osymfn), binfile),
            _ => { println!("Invalid weight type: {:?}", semiring);
                   exit(EXCODE_BADINPUT);
            },
        };
    }
}
