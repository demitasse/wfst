extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, StoreOption};

#[macro_use]
extern crate wfst;
use wfst::semiring::float::Float;
use wfst::semiring::floatweight::{FloatWeight, TropicalWeight, LogWeight, MinmaxWeight};
use wfst::wfst_vec::{VecFst};
use wfst::{MutableFst};

use wfst::wfst_io::{deserialise, deserialise_wrapper, IOError};

use std::any::TypeId;
use std::fmt::Debug;
use std::str::FromStr;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write, BufRead};
use std::process::exit;
use std::collections::HashMap;

extern crate serde;
use serde::{Serialize, Deserialize};
extern crate bincode;
use self::bincode::{deserialize};

const EXCODE_BADINPUT: i32 = 2;

fn load_symtab(symfn: String) -> Result<Vec<String>, IOError> {
    //Slurp lines to parsed fields (DEMITMEM)
    let mut fh = File::open(symfn)?;
    let mut s = String::new();
    fh.read_to_string(&mut s)?;
    let mut entries = Vec::new();
    let mut n: usize = 0;
    for line in s.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 2 {
            return Err(IOError{message: format!("Format error: wrong number of fields")})
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
    Ok(syms)
}

fn load_set_syms<T, W, F>(symfn: Option<String>, fst: &mut F, mapsyms: bool, insym: bool) -> Result<Option<HashMap<String, usize>>, IOError>
    where T: Float<T>,
          W: FloatWeight<T>,
          F: MutableFst<W>,
{
    if let Some(tempfn) = symfn {
        match load_symtab(tempfn) {
            Ok(syms) =>
            {
                if insym {
                    fst.set_isyms(syms.clone());
                } else {
                    fst.set_osyms(syms.clone());
                }
                let symtab: HashMap<String, usize> = syms.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
                if mapsyms {
                    Ok(Some(symtab))
                } else {
                    Ok(None)
                }
            },
            Err(e) => Err(IOError{message: e.message}),
        }
    } else {
        if mapsyms {
            Err(IOError{message: format!("CLI Option error: cannot map input symbols without specifying a table file")})
        } else {
            Ok(None)
        }
    }
}


fn stdin_wfstprint() -> Result<(), IOError> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    handle.read_to_end(&mut buffer)?;
    wfstio_autodeserialise_apply!(buffer, fst, println!("{}", fst))
}



fn main() {
    //Setup defaults and parse args
    let mut mapsyms = false;
    let mut isymfn: Option<String> = None;
    let mut osymfn: Option<String> = None;
    let mut saveisymfn: Option<String> = None;
    let mut saveosymfn: Option<String> = None;
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Creates native FSTs from simple text format.");
        ap.refer(&mut isymfn)
            .add_option(&["-i", "--loadisfn"], StoreOption, "Load input symbol table from filename");
        ap.refer(&mut osymfn)
            .add_option(&["-o", "--loadosfn"], StoreOption, "Load output symbol table from filename");
        ap.refer(&mut saveisymfn)
            .add_option(&["-I", "--saveisfn"], StoreOption, "Save input symbol table to filename");
        ap.refer(&mut saveosymfn)
            .add_option(&["-O", "--saveosfn"], StoreOption, "Save output symbol table to filename");
        ap.refer(&mut mapsyms)
            .add_option(&["-m", "--mapsyms"], StoreTrue, "Map symbols using symbol tables (default is to output integer symbols)");
        
        ap.parse_args_or_exit();
    }

    match stdin_wfstprint() {
        Ok(_) => (),
        Err(e) => { eprintln!("{}", e.message);
                    exit(EXCODE_BADINPUT);
        },
    };

    //     match semiring {
    //         0 => output(input(VecFst::<TropicalWeight<f64>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
    //         1 => output(input(VecFst::<LogWeight<f64>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
    //         2 => output(input(VecFst::<MinmaxWeight<f64>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
    //         _ => { eprintln!("Invalid weight type: {:?}", semiring);
    //                exit(EXCODE_BADINPUT);
    //         },
    //     }
    // } else {
    //     match semiring {
    //         0 => output(input(VecFst::<TropicalWeight<f32>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
    //         1 => output(input(VecFst::<LogWeight<f32>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
    //         2 => output(input(VecFst::<MinmaxWeight<f32>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
    //         _ => { eprintln!("Invalid weight type: {:?}", semiring);
    //                exit(EXCODE_BADINPUT);
    //         },
    //     }
    // } {
    //     Ok(_) => (),
    //     Err(e) => { eprintln!("{}", e.message);
    //                 exit(EXCODE_BADINPUT);
    //     },
    // }
}
