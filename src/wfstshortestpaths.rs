extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, StoreOption};

#[macro_use]
extern crate wfst;
use wfst::semiring::{Weight, NaturalLess};
use wfst::{MutableFst, ExpandedFst};
use wfst::wfst_vec::VecFst;

use std::any::TypeId;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read, Write, BufWriter};
use std::process::exit;

use wfst::wfst_io::{serialise, IOError};
use wfst::algorithms::shortestpath::shortest_paths;

//needed to use `wfstio_autodeserialise_apply` macro...
extern crate bincode;

extern crate serde;
use serde::{Serialize};

const EXCODE_BADINPUT: i32 = 2;

//DEMIT TODO: Remove duplication
fn load_symtab(symfn: &str) -> Result<Vec<String>, IOError> {
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


fn save_symtab(symtab: Vec<String>, symfn: &str) -> Result<(), IOError> {
    let fh = File::create(symfn)?;
    let mut fh = BufWriter::new(fh);
    for (i, s) in symtab.into_iter().enumerate() {
        fh.write(format!("{}\t{}\n", s, i).as_bytes())?;
    }
    Ok(())
}


fn wfstprint<W: Weight + NaturalLess + Serialize + 'static, F: MutableFst<W> + ExpandedFst<W> + Display>(mut fst: F, isyms: Option<Vec<String>>, osyms: Option<Vec<String>>, isymfn: Option<String>, osymfn: Option<String>, mapsyms: bool) -> Result<(), IOError> {

    if let Some(syms) = isyms {
        fst.set_isyms(syms);
    }
    if let Some(syms) = osyms {
        fst.set_osyms(syms);
    }

    if let Some(symfn) = isymfn {
        if let Some(syms) = fst.get_isyms() {
            let symtab: Vec<String> = syms.into_iter().collect();
            save_symtab(symtab, &symfn)?;
        }
    }

    if let Some(symfn) = osymfn {
        if let Some(syms) = fst.get_osyms() {
            let symtab: Vec<String> = syms.into_iter().collect();
            save_symtab(symtab, &symfn)?;
        }
    }
    
    if !mapsyms {
        fst.del_isyms();
        fst.del_osyms();
    }

    //Messy that shortest_paths() requires Serialize...
    let fst2: VecFst<_> = shortest_paths(fst, 6, false); //DEMIT: hardcoded 'npaths' (fixme) 
    let encoded = serialise(&fst2)?;
    io::stdout().write(&*encoded).ok();
    Ok(())
    //println!("{:?}", fst2); //DEMIT: should actually output "compiled fst" (fix later)
    //Ok(())
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

    //Slurp STDIN (DEMITMEM)
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = Vec::new();
    match handle.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(e) => { eprintln!("{}", e);
                        exit(EXCODE_BADINPUT);
        },
    };

    //Try to load symtabs?
    let isyms = match isymfn {
        Some(symfn) => match load_symtab(&symfn) {
            Ok(syms) => Some(syms),
            Err(e) => { eprintln!("{}", e.message);
                        exit(EXCODE_BADINPUT);
            },
        },
        None => None,
    };
    let osyms = match osymfn {
        Some(symfn) => match load_symtab(&symfn) {
            Ok(syms) => Some(syms),
            Err(e) => { eprintln!("{}", e.message);
                        exit(EXCODE_BADINPUT);
            },
        },
        None => None,
    };

    match wfstio_autodeserialise_apply_naturalless!(buffer, fst, wfstprint(fst, isyms, osyms, saveisymfn, saveosymfn, mapsyms)) {
        Ok(_) => (),
        Err(e) => { eprintln!("{}", e.message);
                    exit(EXCODE_BADINPUT);
        },
    };
}
