use hashbrown::HashMap;
use rustc_hash::FxBuildHasher;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("fileseq.txt").expect("open");
    let reader = BufReader::new(f);
    let mut m: HashMap<u64, (), FxBuildHasher> = HashMap::default();
    let (mut ins_count, mut rm_count) = (0u64, 0u64);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 || parts[0] != "DSTDIAG_FILESEQ" {
            continue;
        }
        let id: u64 = parts[2].parse().unwrap();
        match parts[1] {
            "INS" => {
                m.insert(id, ());
                ins_count += 1;
            }
            "RM" => {
                m.remove(&id);
                rm_count += 1;
            }
            _ => {}
        }
    }
    let order: Vec<u64> = m.keys().copied().collect();
    println!(
        "ins={} rm={} final_len={} cap={}",
        ins_count,
        rm_count,
        m.len(),
        m.capacity()
    );
    println!("iter_order: {:?}", order);
}
