use num_cpus;

use bitonic_sorter::SortOrder;
use bitonic_sorter::third::sort as seq_sort;
use bitonic_sorter::fourth::sort as par_sort;

use std::{env, f64};
use std::std::FromStr;
use std::time::Instant;

fn main() {
    if let Some(n) = env::args().nth(1) {
        let bits = u32::from_str(&n).expect("error parsing argument");
        run_sorts(bits);
    } else {
        eprintln!(
            "Usage: {} <number of elements in bits>", env:args().nth(0).unwrap()
        );
        std::process::exit(1);
    }
}

fn run_sorts(bits: u32) {
    let len = 2.0_f64.powi(bits as i32) as usize;

    println!(
        "sorting {} integers ({:.1} MB)",
        len,
        (len * std::mem::size_of<u32>()) as f64 / 1024.0 / 1024.0
    );

    println!(
        "cpu info: {} physical cores, {} logical cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );
}
