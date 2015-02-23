#![feature(hash)]

use std::collections::HashMap;
use std::old_io as io;

// https://stackoverflow.com/questions/28392008/more-concise-hashmap-initialization
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() {
    let rna_codon_table: HashMap<&'static str,&'static str> = hashmap![
        "UUU" => "F","CUU" => "L","AUU" => "I","GUU" => "V","UUC" => "F","CUC" => "L","AUC" => "I","GUC" => "V","UUA" => "L","CUA" => "L","AUA" => "I","GUA" => "V","UUG" => "L","CUG" => "L","AUG" => "M","GUG" => "V","UCU" => "S","CCU" => "P","ACU" => "T","GCU" => "A","UCC" => "S","CCC" => "P","ACC" => "T","GCC" => "A","UCA" => "S","CCA" => "P","ACA" => "T","GCA" => "A","UCG" => "S","CCG" => "P","ACG" => "T","GCG" => "A","UAU" => "Y","CAU" => "H","AAU" => "N","GAU" => "D","UAC" => "Y","CAC" => "H","AAC" => "N","GAC" => "D","UAA" => "Stop","CAA" => "Q","AAA" => "K","GAA" => "E","UAG" => "Stop","CAG" => "Q","AAG" => "K","GAG" => "E","UGU" => "C","CGU" => "R","AGU" => "S","GGU" => "G","UGC" => "C","CGC" => "R","AGC" => "S","GGC" => "G","UGA" => "Stop","CGA" => "R","AGA" => "R","GGA" => "G","UGG" => "W","CGG" => "R","AGG" => "R","GGG" => "G"];


   
    let rna = io::stdin().read_line().ok().expect("Reading stdin failed!");

    let mut result: String = "".to_string();

    for i in 0 ..(rna.len()/3) {
        let i = i * 3;
        let tmp: &str = &rna[i..i+3];
        let result = match rna_codon_table.get(tmp) {
            Some(&"Stop") => break,
            Some(s) => *s,
            None => panic!("Unexpected string!"),
        };
        print!("{}", result)
    }


}