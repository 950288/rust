use pdbtbx::*;
use std::env;

fn main() {

    let mut pdb_file = None;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: pdb2seq.exe <pdb_file>");
        return;
    } else {
        println!("Reading PDB file: {}", args[1]);
        pdb_file = Some(&args[1]);
    }

    let (pdb, _errors) = pdbtbx::open(
        pdb_file.unwrap(),
        StrictnessLevel::Medium
    ).unwrap();

    for i in 0..pdb.chain_count() {
        let chain = pdb.chain(i).unwrap();
        println!("Chain {} :", chain.id());
        println!("{}", chain2seq(&chain));
    }
}

fn chain2seq(chain: &Chain) -> String {
    
    let residues = chain.residues();

    let mut seq: Vec<&str> = Vec::new();

    let aa_map = [
        ("ALA", "A"), ("CYS", "C"), ("ASP", "D"), ("GLU", "E"), ("PHE", "F"),
        ("GLY", "G"), ("HIS", "H"), ("ILE", "I"), ("LYS", "K"), ("LEU", "L"),
        ("MET", "M"), ("ASN", "N"), ("PRO", "P"), ("GLN", "Q"), ("ARG", "R"),
        ("SER", "S"), ("THR", "T"), ("VAL", "V"), ("TRP", "W"), ("TYR", "Y")
    ].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    for residue in residues {
        if let Some(three_letter_code) = residue.name() {
            if let Some(one_letter_code) = aa_map.get(three_letter_code) {
                seq.push(one_letter_code);
            }
        }
    }

    let mut seq_str = String::new();

    for aa in seq {
        seq_str.push_str(aa);
    }

    return seq_str;
}