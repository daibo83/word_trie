use word_trie::WordTrie;
// use ahash::AHashMap;
use std::{
    fs::File,
    io::{prelude::*, BufReader,stdin,stdout,Write
	},
	path::Path,
};

use std::time::Instant;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let now = Instant::now();
    let mut trie = WordTrie::new();
	let lines = lines_from_file("/home/linh/invertedIndex/database-updater/files/index_files/inv_index/shingle_inv_index.tsv");
	
	let mut key = "".to_string();
	let mut value: u32 = 0;
    for line in lines{
		let tag = line.clone();
		let key_temp: String = tag.split('\t').next().unwrap().to_string().chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect();
		key = key_temp.to_lowercase();
		// println!("{}", key);
		
		let temp_value: f64 = tag.split('\t').last().unwrap().parse::<f64>().unwrap_or(0.0);
		// if temp_value >=1.0 {
			
			value = temp_value.log10().abs().ceil() as u32;
			trie.insert(key, Some(value));
			// dict.insert(key, Some(value));
		// }
	}
    println!("trie build in: {:?}", now.elapsed());
    loop{
		let _=stdout().flush();
		let mut input: String = "".to_string();
		stdin().read_line(&mut input).expect("Did not enter a correct string");
		if let Some('\n')=input.chars().next_back() {
		input.pop();
		}
		if let Some('\r')=input.chars().next_back() {
		input.pop();
		}
		input = input.trim().to_owned();
		let now = Instant::now();
		let word  = trie.get(&input);
		println!("found {:?} in {:?}", word, now.elapsed());
	}
}