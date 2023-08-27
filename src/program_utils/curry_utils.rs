use crate::program_utils::program::Program;
use crate::program_utils::serialize::node_to_bytes as serialize2;
use crate::program_utils::serialized_program::SerializedProgram;
use clvm_tools_rs::classic::clvm::__type_compatibility__::{Bytes, BytesFromType};
use clvm_tools_rs::classic::clvm::sexp::equal_to;
use clvm_tools_rs::classic::clvm_tools::binutils::assemble as chia_assemble;
use clvmr::allocator::{Allocator, NodePtr};

use crate::program_utils::serialize::node_to_bytes;
use clvmr::cost::Cost;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;

use super::node::Node;

pub fn assemble(input_text: &str) -> SerializedProgram {
    let mut allocator = Allocator::new();
    let ptr = chia_assemble(&mut allocator, &input_text.to_string()).unwrap();
    let result = serialize2(&Node::new(&mut allocator, ptr))
        .map_err(|e| e.to_string().as_bytes().to_vec())
        .unwrap();
    SerializedProgram::from_bytes(&result)
}

lazy_static! {
    pub static ref CURRY_OBJ_CODE: SerializedProgram = assemble("(a (q #a 4 (c 2 (c 5 (c 7 0)))) (c (q (c (q . 2) (c (c (q . 1) 5) (c (a 6 (c 2 (c 11 (q 1)))) 0))) #a (i 5 (q 4 (q . 4) (c (c (q . 1) 9) (c (a 6 (c 2 (c 13 (c 11 0)))) 0))) (q . 11)) 1) 1))");
    pub static ref UNCURRY_PATTERN_FUNCTION: SerializedProgram = assemble("(a (q . (: . function)) (: . core))");
    pub static ref UNCURRY_PATTERN_CORE: SerializedProgram = assemble("(c (q . (: . parm)) (: . core))");
}

const BYTE_MATCH: [u8; 1] = [81 as u8];
const ATOM_MATCH: [u8; 1] = ['$' as u8];
const SEXP_MATCH: [u8; 1] = [':' as u8];

pub fn curry<'a>(program: &Program, args: Vec<Program>) -> Result<(Cost, Program), Box<dyn Error>> {
    let mut alloc = Allocator::new();
    let args = make_args(args);
    let pair: Program = program.cons(&args);
    let cur_prog = CURRY_OBJ_CODE.clone();
    let (cost, result) = cur_prog.run_with_cost(&mut alloc, Cost::MAX, &pair)?;
    let prog = Node::new(&mut alloc, result);
    let bytes = node_to_bytes(&prog)?;
    Ok((cost, Program::new(bytes)))
}

fn make_args(args: Vec<Program>) -> Program {
    if args.len() == 0 {
        return Program::null();
    }
    let mut rtn = args.last().unwrap().cons(&Program::null());
    let mut rest = args.clone();
    rest.reverse();
    for arg in &rest[1..=rest.len() - 1] {
        rtn = arg.cons(&rtn);
    }
    rtn
}

pub fn unify_bindings<'a>(
    allocator: &'a mut Allocator,
    bindings: HashMap<String, NodePtr>,
    new_key: &Vec<u8>,
    new_value: NodePtr,
) -> Option<HashMap<String, NodePtr>> {
    /*
     * Try to add a new binding to the list, rejecting it if it conflicts
     * with an existing binding.
     */
    let new_key_str = Bytes::new(Some(BytesFromType::Raw(new_key.to_vec()))).decode();
    match bindings.get(&new_key_str) {
        Some(binding) => {
            if !equal_to(allocator, *binding, new_value) {
                return None;
            }
            return Some(bindings);
        }
        _ => {
            let mut new_bindings = bindings.clone();
            new_bindings.insert(new_key_str, new_value);
            return Some(new_bindings);
        }
    }
}
