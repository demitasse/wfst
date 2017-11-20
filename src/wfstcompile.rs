extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, StoreOption};

extern crate wfst;
use wfst::semiring::floatweight::TropicalWeight;
use wfst::wfst_vec::{VecFst};
use wfst::{MutableFst};

use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader};
use std::process::exit;

extern crate rustc_serialize;
use rustc_serialize::json;
extern crate bincode;
use bincode::rustc_serialize::decode;

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
        for l in s.lines() {
            let fields = l.split_whitespace().collect::<Vec<_>>();
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


fn main() {
    //Setup defaults and parse args
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
        ap.refer(&mut binfile)
            .add_option(&["-b", "--bin"], StoreTrue, "Output file in binary format (default is json)");
        ap.parse_args_or_exit();
    }

    //Create FST
    let mut fst = VecFst::<TropicalWeight<f64>>::new();

    match load_symtab(isymfn) {
        Ok(d) => if let Some(syms) = d {
            println!("{:?}", syms);
            fst.set_isyms(syms);
        },
        Err(e) => {println!("Error loading file: {:?}", e.message);
                   exit(EXCODE_BADINPUT);
        },
    };
    match load_symtab(osymfn) {
        Ok(d) => if let Some(syms) = d {
            println!("{:?}", syms);
            fst.set_osyms(syms);
        },
        Err(e) => {println!("Error loading file: {:?}", e.message);
                   exit(EXCODE_BADINPUT);
        },
    };
    println!("{:?}", fst);

    ////Parse and build fst from stdin

    //IO loop using
    let stdin = io::stdin();
    let handle = stdin.lock();    //for (fast) buffered input
    for l in handle.lines() {     //strips newlines
        let line = l.unwrap();
        println!("{}", line);
    }
}
