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

extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
extern crate bincode;
use bincode::{serialize, deserialize, Infinite};

const EXCODE_BADINPUT: i32 = 2;

#[derive(Debug)]
pub struct IOError {
    pub message: String,
}

impl<T: Error> From<T> for IOError {
    fn from(e: T) -> IOError {
        IOError{message: format!("Format error: {}", e.description())}
    }
}

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

fn input<T, W, F>(mut fst: F, isymfn: Option<String>, osymfn: Option<String>, mapisyms: bool, maposyms: bool) -> Result<F, IOError>
    where T: Float<T> + FromStr,
          T::Err: Error + Debug,
          W: FloatWeight<T>,
          F: MutableFst<W>,
{
    ////Possibly load/apply symbol tables
    let isymtab = load_set_syms(isymfn, &mut fst, mapisyms, true)?;
    let osymtab = load_set_syms(osymfn, &mut fst, maposyms, false)?;
    //eprintln!("{:?}", isymtab);
    //eprintln!("{:?}", osymtab);
    
    ////Parse input from STDIN
    let stdin = io::stdin();
    let handle = stdin.lock();    //for (fast) buffered input
    let mut is_start = true;
    let mut arcs = Vec::new();
    let mut nstates: usize = 0;
    let mut startstate: usize = 0;
    let mut finalstates = HashMap::new();
    for l in handle.lines() {     //strips newlines
        let line: String = l.unwrap();
        //eprintln!("{}", line);
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let (is_final, weight) = match fields.len() {
            1 => (true, W::one()),
            2 => (true, W::new(Some(fields[1].parse()?))),
            4 => (false, W::one()),
            5 => (false, W::new(Some(fields[4].parse()?))),
            _ => return Err(IOError{message: format!("Format error: wrong number of fields")})
        };
        //eprintln!("{:?} {:?} {:?}", is_start, is_final, weight);
        if !is_final {
            let src = fields[0].parse()?;
            let tgt = fields[1].parse()?;
            if src > nstates { nstates = src };
            if tgt > nstates { nstates = tgt };
            let ilabel = if let Some(ref symtab) = isymtab {
                *symtab.get(fields[2]).ok_or(IOError{message: format!("Input error: symbol table does not contain 'string' symbol")})?
            } else {
                fields[2].parse()?
            };
            let olabel = if let Some(ref symtab) = osymtab {
                *symtab.get(fields[3]).ok_or(IOError{message: format!("Input error: symbol table does not contain 'string' symbol")})?
            } else {
                fields[3].parse()?
            };
            arcs.push((src, tgt, ilabel, olabel, weight));
        } else {
            let tgt = fields[0].parse()?;
            finalstates.insert(tgt, weight);
            if tgt > nstates { nstates = tgt };
        }            
        if is_start {
            startstate = fields[0].parse()?;
            is_start = false;
        }
    }
    nstates += 1;
    // eprintln!("nstates: {}", nstates);
    // eprintln!("startstate: {}", startstate);
    // eprintln!("finalstates: {:?}", finalstates);
    // eprintln!("{:?}", arcs);
    // eprintln!("{:?}", fst);

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
    Ok(fst)
}

fn output<T: Serialize + Deserialize + Debug>(t: Result<T, IOError>, jsonout: bool) -> Result<(), IOError> {
    ////Output on STDOUT
    match t {
        Ok(tt) => {
            println!("{:?}\n", tt);
            if jsonout {    
                let encoded = serde_json::to_string(&tt)?;
                //////////
                let ww: T = serde_json::from_str(&encoded).unwrap();
                println!("{:?}\n", ww);
                //////////
                println!("{}", encoded);
            } else {
                let encoded = serialize(&tt, Infinite)?;
                //////////
                let ww: T = deserialize(&encoded).unwrap();
                println!("{:?}\n", ww);
                //////////
                io::stdout().write(&*encoded).ok();
            }
            Ok(())
        },
        Err(e) => Err(e),
    }
}


fn main() {
    //Setup defaults and parse args
    let mut p64 = false;
    let mut wtype: Option<usize> = None;
    let mut jsonout = false;
    let mut mapisyms = false;
    let mut maposyms = false;
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
        ap.refer(&mut mapisyms)
            .add_option(&["-I", "--strsin"], StoreTrue, "Map input symbols using symbol table (default is to read integer symbols)");
        ap.refer(&mut maposyms)
            .add_option(&["-O", "--strsout"], StoreTrue, "Map output symbols using symbol table (default is to read integer symbols)");
        ap.refer(&mut p64)
            .add_option(&["-p", "--precision"], StoreTrue, "Use 64-bit precision for weights (default is 32-bit)");
        ap.refer(&mut jsonout)
            .add_option(&["-j", "--json"], StoreTrue, "Output file in JSON format (not recommended JSON does not support inf/NaN -- use only for debugging)");
        ap.parse_args_or_exit();
    }

    let semiring = wtype.unwrap_or(0);
    match if p64 {
        match semiring {
            0 => output(input(VecFst::<TropicalWeight<f64>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
            1 => output(input(VecFst::<LogWeight<f64>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
            2 => output(input(VecFst::<MinmaxWeight<f64>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
            _ => { eprintln!("Invalid weight type: {:?}", semiring);
                   exit(EXCODE_BADINPUT);
            },
        }
    } else {
        match semiring {
            0 => output(input(VecFst::<TropicalWeight<f32>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
            1 => output(input(VecFst::<LogWeight<f32>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
            2 => output(input(VecFst::<MinmaxWeight<f32>>::new(), isymfn, osymfn, mapisyms, maposyms), jsonout),
            _ => { eprintln!("Invalid weight type: {:?}", semiring);
                   exit(EXCODE_BADINPUT);
            },
        }
    } {
        Ok(_) => (),
        Err(e) => { eprintln!("{}", e.message);
                    exit(EXCODE_BADINPUT);
        },
    }
}
