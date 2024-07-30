use pdbtbx::*;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: pdb2seq.exe <pdb_file>");
        return;
    } else {
        println!("Reading PDB file: {}", args[1]);
    }

    let (pdb, _errors) = pdbtbx::open(
        &args[1],
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

    // let mut seq: Vec<&str> = Vec::new();
    let mut seq = String::new();

    let aa_map = [
        ("ALA", "A"), ("CYS", "C"), ("ASP", "D"), ("GLU", "E"), ("PHE", "F"),
        ("GLY", "G"), ("HIS", "H"), ("ILE", "I"), ("LYS", "K"), ("LEU", "L"),
        ("MET", "M"), ("ASN", "N"), ("PRO", "P"), ("GLN", "Q"), ("ARG", "R"),
        ("SER", "S"), ("THR", "T"), ("VAL", "V"), ("TRP", "W"), ("TYR", "Y")
    ].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    for residue in residues {
        if let Some(three_letter_code) = residue.name() {
            if let Some(one_letter_code) = aa_map.get(three_letter_code) {
                seq.push((one_letter_code).chars().next().unwrap());
            }
        }
    }

    return seq;
}