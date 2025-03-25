use std::fs::File;
use std::io::{BufReader, Read};
use flate2::read::GzDecoder;
use needletail::parse_fastx_reader;
use std::collections::hash_map::RandomState;
use hyperloglogplus::{HyperLogLog, HyperLogLogPlus};
use clap::Parser;
use std::io::{self, BufRead};
use std::path::Path;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// Function to hash a k-mer
fn hash_kmer(kmer: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    kmer.hash(&mut hasher);
    hasher.finish()
}

// Process a single file
fn process_file(filename: &str, 
    global_hll: &mut HyperLogLogPlus<u64, RandomState>, 
    local_hll: &mut HyperLogLogPlus<u64, RandomState>,
    k: usize) -> std::io::Result<()> {

    let reader: Box<dyn Read + Send> = if filename.ends_with(".gz") {
        Box::new(GzDecoder::new(File::open(filename)?))
    } else {
        Box::new(File::open(filename)?)
    };
    
    let mut reader = BufReader::new(reader);
    let mut fastx_reader = parse_fastx_reader(&mut reader).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    while let Some(record) = fastx_reader.next() {
        let seqrec = record.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let seq = seqrec.seq();
        for kmer in seq.windows(k) {
            global_hll.insert(&hash_kmer(kmer));
            local_hll.insert(&hash_kmer(kmer));
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file of files (one fasta or fastq [.gz] per line)
    #[arg(short, long)]
    input: String,
    /// K-mer size (up to 32)
    #[arg(short)]
    k: usize,
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args = Args::parse();
    let k = args.k;
    assert!(k <= 32);
    let path = args.input.clone();
    let mut global_hll = HyperLogLogPlus::new(16, RandomState::new()).unwrap();
    if let Ok(lines) = read_lines(&path) {
        // Consumes the iterator, returns an (Optional) String
        for local_path in lines.map_while(Result::ok) {
            let mut local_hll = HyperLogLogPlus::new(16, RandomState::new()).unwrap();
            if let Err(e) = process_file(&local_path, &mut local_hll, &mut global_hll, k) {
                eprintln!("Error processing {}: {}", local_path, e);
            }
            println!("Estimated unique k-mers in {}: {}", local_path, local_hll.count().floor() as u64);
        }
    }
    println!("Estimated unique k-mers in all files from {}: {}", path, global_hll.count().floor() as u64); 
}
