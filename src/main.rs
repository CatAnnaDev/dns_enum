use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

fn main() {
    let arg = args().nth(1).expect("need word_list as arg\n ./word_list_cleaner my_word_list.txt");
    let mut file = File::open(&arg).expect("Can't open wordlist");
    let mut tmp = String::new();
    let _ = file.read_to_string(&mut tmp);
    let mut hashmap = HashMap::new();
    let mut backup_vec = vec![];
    for x in tmp.lines() {
        if let Some(_) = hashmap.insert(x, ()) { backup_vec.push(x) }
    }
    let len = hashmap.len();
    let back_len = backup_vec.len();
    println!("Keywords Unique {}", len);
    println!("Keywords Duplicate {}", back_len);
    println!("Keywords Total {}", len + back_len);

    if back_len == 0{ println!("Exiting no duplicate found"); exit(0); }

    let mut write_file = File::create(arg.replace(".txt", ".cleaned.txt")).unwrap();
    for (x, (s, _)) in hashmap.iter().enumerate() {
        print!("\r{}/{}", x, len);
       let _ = writeln!(write_file, "{s}");
    }

}
