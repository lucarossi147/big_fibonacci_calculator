use std::fs::File;
use std::io::prelude::*;
use std::env;
use memoized_fibonacci::fibo_iterative;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let n_str = args.get(1).expect("No number to process");
    let n = u32::from_str_radix(n_str, 10).unwrap();
    let now = Instant::now();
    let ciccia = fibo_iterative(n);
    println!("Completed iterative version in {}", now.elapsed().as_millis());
    let mut file = File::create(n.to_string()+".txt")?;
    file.write(ciccia.to_string().as_bytes())?;
    Ok(())
}
