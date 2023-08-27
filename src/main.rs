mod api;
mod blockchain;
mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod chia_wallet;
mod chiapos;
mod program_utils;

use std::collections::HashMap;

use crate::chia_wallet::{
    cat::puzzles::create_cat_outer_puzzle::{create_cat_outer_puzzlehash, create_cat_puzzle},
    core::serialization::{serialize_item, serialize_list, serialize_map, ToBytesMixin},
};
use api::hex_to_bytes;

use crate::{
    api::{
        bytes_to_hex, cmd_program_opc, cmd_program_opd, cmds_program_run, program_curry,
        program_uncurry,
    },
    chia_wallet::standart::puzzles::p2_delegated_puzzle_or_hidden_puzzle::get_puzzle_from_pk,
    program_utils::{program::Program, serialized_program::SerializedProgram},
};

fn main() {
    let acs = Program::from(1);
    // let acs_ph = acs.clone().tree_hash().to_sized_bytes().to_vec();
    let empyth_program_lst: Vec<Program> = vec![];
    let tail = Program::from(empyth_program_lst);
    let tail_hash = tail.tree_hash().to_sized_bytes().to_vec();
    let cat_puzzle = create_cat_puzzle(tail_hash.clone(), acs.clone().serialized.clone());
    let cat_puzzlehash = cat_puzzle.tree_hash().to_sized_bytes().to_vec();
    let uncurried = program_uncurry(cat_puzzle.serialized.clone());
    //println!("uncurried: {}", bytes_to_hex(uncurried.program.clone()));

    //iter args

    let binding = uncurried.args.clone();
    let mut args = binding.iter();
    let mut i = 0;
    loop {
        match args.next() {
            Some(arg) => {
                i = i + 1;
                println!("arg: {} {}", i, (arg.clone()));
            }
            None => break,
        }
    }

    // println!("cat_puzzlehash1: {}", bytes_to_hex(cat_puzzle.serialized.clone()));
    println!("cat_puzzlehash1: {}", bytes_to_hex(cat_puzzlehash.clone()));
    println!(
        "tail_hash as atom: {}",
        bytes_to_hex(Program::from(cat_puzzlehash.clone()).serialized.clone())
    );
    println!(
        "tail_hash from serialized: {}",
        bytes_to_hex(Program::new(cat_puzzlehash.clone()).serialized.clone())
    );

    let number: i64 = 42;
    let serialized_number = serialize_item(&number);
    println!("Número serializado: {:?}", serialized_number);

    // Ejemplo de uso para bool
    let boolean: bool = true;
    let serialized_boolean = serialize_item(&boolean);
    println!("Booleano serializado: {:?}", serialized_boolean);

    // Ejemplo de uso para String
    let text: String = "Hello, Rust!".to_string();
    let serialized_text = serialize_item(&text);
    println!("Texto serializado: {:?}", serialized_text);

    // Ejemplo de uso para HashMap
    let mut map: HashMap<Box<dyn ToBytesMixin>, Box<dyn ToBytesMixin>> = HashMap::new();
    map.insert(Box::new(42_i64), Box::new(true));
    map.insert(Box::new("Key".to_string()), Box::new("Value".to_string()));
    let serialized_map = serialize_map(&map);
    println!("Mapa serializado: {:?}", serialized_map);

    // Ejemplo de uso para Vec
    let list: Vec<Box<dyn ToBytesMixin>> = vec![
        Box::new("Apple".to_string()),
        Box::new(false),
        Box::new(123_i64),
    ];
    let serialized_list = serialize_list(&list);
    println!("Lista serializada: {:?}", serialized_list);
}
