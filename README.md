# Hyperloglog distinct kmer counter
A simple tool for estimating the number of distinct kmers in a set of fasta fastq files gzipped or not. 

## License 
AGPL-3.0

## Install
```bash
git clone https://github.com/pierrepeterlongo/hyperloglog_kmer_counter
cd hyperloglog_kmer_counter 
cargo install --path .  
```

The executable name is `hyperloglog_kmer_counter`.

## Warning
- This tool does not filter input data, it does not count kmers.
- This tool considers **all** kmers, also those including non-ACGTacgt characters
- Size of kmers is at most 32.
- No parallelization

## Usage
```
Usage: hyperloglog_kmer_counter --input <INPUT> -k <K>

Options:
  -i, --input <INPUT>  Input file of files (one fasta or fastq [.gz] per line)
  -k <K>               K-mer size (up to 32)
  -h, --help           Print help
  -V, --version        Print version
```

### Example:
```
hyperloglog_kmer_counter -i tests/fof.txt -k 31
Estimated unique k-mers in tests/a.fa: 970
Estimated unique k-mers in tests/b.fq: 970
Estimated unique k-mers in tests/b.fq.gz: 970
Estimated unique k-mers in all files from tests/fof.txt: 1940
```

