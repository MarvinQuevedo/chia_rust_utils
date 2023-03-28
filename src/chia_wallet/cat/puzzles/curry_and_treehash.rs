use crate::program_utils::{serialized_program::SerializedProgram, program::Program};

 
pub static CURRY_AND_TREEHASH_MOD_STR : &str = "ff02ffff01ff02ff1effff04ff02ffff04ff05ffff04ff17ffff04ffff0bff1cff0b80ffff04ffff0bff1cff0580ff80808080808080ffff04ffff01ffff02ff0401ffff0102ffff02ffff03ff05ffff01ff02ff16ffff04ff02ffff04ff0dffff04ffff0bff1affff0bff1cff1480ffff0bff1affff0bff1affff0bff1cff1280ff0980ffff0bff1aff0bffff0bff1cff8080808080ff8080808080ffff010b80ff0180ff0bff1affff0bff1cff0880ffff0bff1affff0bff1affff0bff1cff1280ff0580ffff0bff1affff02ff16ffff04ff02ffff04ff07ffff04ffff0bff1cff1c80ff8080808080ffff0bff1cff8080808080ff018080";


  lazy_static::lazy_static! {

    pub static ref CURRY_AND_TREEHASH_MOD: Program = {
            let program = SerializedProgram::from_hex(CURRY_AND_TREEHASH_MOD_STR.to_string())
                .to_program()
                .unwrap(); 
            program
    };
 
}
