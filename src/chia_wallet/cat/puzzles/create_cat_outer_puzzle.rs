use crate::{
    api::{bytes_to_hex, hex_to_bytes},
    program_utils::{program::Program, serialized_program::SerializedProgram},
};

use super::{
    cat_puzzle_program::{CAT_MOD, CAT_MOD_HASH_PROGRAM},
    curry_and_treehash::CURRY_AND_TREEHASH_MOD,
};

// bytes tailhash,  bytes innerpuzhash
pub fn create_cat_puzzle(tail_hash: Vec<u8>, inner_puzzle_hash: Vec<u8>) -> Program {
    let cat_program = CAT_MOD.clone();
    let cat_hash_program = CAT_MOD_HASH_PROGRAM.clone() ;
    let limitations_program_hash = Program::from(&tail_hash.clone());
    let inner_puzzle_hash_program = Program::from(&inner_puzzle_hash.clone());
  
    let args = [
        cat_hash_program,
        limitations_program_hash.clone(),
        inner_puzzle_hash_program.clone(),
    ]
    .to_vec();
 
  

    return cat_program.curry(args.clone());
}
// bytes tailhash,  bytes innerpuzhash
pub fn create_cat_outer_puzzlehash(tail_hash: Vec<u8>, inner_puzzle_hash: Vec<u8>) -> Vec<u8> {
   // let cat_program = CAT_MOD.clone();
    let cat_hash_program = CAT_MOD_HASH_PROGRAM.clone();
    let tail_hash_program = Program::from(&tail_hash.clone());
    let inner_puzzle_hash_program = Program::from(&inner_puzzle_hash.clone());

    let solution = [
        cat_hash_program,
        tail_hash_program,
        inner_puzzle_hash_program,
    ]
    .to_vec();

    let result = CURRY_AND_TREEHASH_MOD.clone().run(Program::from(solution));
    let atom = result.program.as_atom().unwrap();
    let puzzlehash  = atom.serialized.clone();
    return puzzlehash;
}
