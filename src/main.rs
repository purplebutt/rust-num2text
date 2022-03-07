use serde_json;
use std::{time::Instant, process::exit, collections::HashMap};
use num_format::{Locale, ToFormattedString};
use rand::Rng;
pub mod num2text;

use num2text::caller::totxt as n2txt;
use std::{env::args};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len()!=2 {
        println!("You provide no argument, running test instead!");
        //randomtest(Some(100));
        match update_cache() {
            Err(e) => println!("error-> {}",e),
            Ok(_) => ()
        }
        exit(1)
    }
    match args.last() {
        Some(v) => {
            if v=="test" {randomtest(Some(50)); return;}
            if v=="benchmark" {benchmark(); return;}
            match v.parse::<u32>() {
                Ok(_v) => {
                    //TODO -> no error, running application
                    println!("{}", n2txt(args.last().unwrap()));
                    exit(0)
                }
                Err(e) => {
                    println!("{}",e);
                    exit(2)
                }
            }
        },
        _=>{ println!("need numbers");
                exit(3)
            }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn benchmark() {
    let start = Instant::now();
    const MAX_ITER: i32 = 1_000_000;
    // let mut x = String::new();
    for i in 0..MAX_ITER {
        n2txt(&i.to_string());
    }
    let elapsed = Instant::now()-start;
    println!("execution time: {:?} for {} loops",elapsed, MAX_ITER.to_formatted_string(&Locale::en));
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn randomtest(max_loop: Option<u16>) {
    let mut rng = rand::thread_rng();
    let start = Instant::now();
    
    match max_loop {
        Some(v) => {
            for i in 0..v {
                let rnd_number = rng.gen::<u32>();
                let numtext = n2txt(&rnd_number.to_string());
                println!("{}| {}-> {}",i,rnd_number.to_formatted_string(&Locale::en) ,numtext)
            }
            let elapsed = Instant::now()-start;
            println!("execution time: {:?} for {} loops",elapsed, v);
        }
        _=>()
    }
}

use std::fs::File;
#[allow(dead_code)]
#[allow(unused_variables)]
fn update_cache() -> std::io::Result<()> {
    let start = Instant::now();
    let mut cache: HashMap<u16, String> = HashMap::new();
    for i in 0..1000 {
        cache.insert(i, n2txt(&i.to_string()));
    };

    let f = File::create("./cache.n2t")?;

    serde_json::to_writer(f, &cache)?;
    println!("{:?}",cache);
    println!("\"{}\"",cache.get(&4).expect("error"));
    let elapsed = Instant::now()-start;
    println!("execution time: {:?}",elapsed);
    Ok(())
}
