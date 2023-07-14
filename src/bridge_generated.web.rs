use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_secret_key_from_seed(port_: MessagePort, seed: Box<[u8]>) {
    wire_secret_key_from_seed_impl(port_, seed)
}

#[wasm_bindgen]
pub fn wire_secret_key_public_key(port_: MessagePort, sk: Box<[u8]>) {
    wire_secret_key_public_key_impl(port_, sk)
}

#[wasm_bindgen]
pub fn wire_secret_key_derive_path_hardened(port_: MessagePort, sk: Box<[u8]>, path: Box<[u32]>) {
    wire_secret_key_derive_path_hardened_impl(port_, sk, path)
}

#[wasm_bindgen]
pub fn wire_secret_key_derive_path_unhardened(port_: MessagePort, sk: Box<[u8]>, path: Box<[u32]>) {
    wire_secret_key_derive_path_unhardened_impl(port_, sk, path)
}

#[wasm_bindgen]
pub fn wire_public_key_derive_path_unhardened(port_: MessagePort, sk: Box<[u8]>, path: Box<[u32]>) {
    wire_public_key_derive_path_unhardened_impl(port_, sk, path)
}

#[wasm_bindgen]
pub fn wire_signature_sign(port_: MessagePort, sk: Box<[u8]>, msg: Box<[u8]>) {
    wire_signature_sign_impl(port_, sk, msg)
}

#[wasm_bindgen]
pub fn wire_signature_is_valid(port_: MessagePort, sig: Box<[u8]>) {
    wire_signature_is_valid_impl(port_, sig)
}

#[wasm_bindgen]
pub fn wire_signature_aggregate(port_: MessagePort, sigs_stream: Box<[u8]>, length: usize) {
    wire_signature_aggregate_impl(port_, sigs_stream, length)
}

#[wasm_bindgen]
pub fn wire_signature_verify(port_: MessagePort, pk: Box<[u8]>, msg: Box<[u8]>, sig: Box<[u8]>) {
    wire_signature_verify_impl(port_, pk, msg, sig)
}

#[wasm_bindgen]
pub fn wire_pub_mnemonic_to_entropy(port_: MessagePort, mnemonic_words: String) {
    wire_pub_mnemonic_to_entropy_impl(port_, mnemonic_words)
}

#[wasm_bindgen]
pub fn wire_pub_entropy_to_mnemonic(port_: MessagePort, entropy: Box<[u8]>) {
    wire_pub_entropy_to_mnemonic_impl(port_, entropy)
}

#[wasm_bindgen]
pub fn wire_pub_entropy_to_seed(port_: MessagePort, entropy: Box<[u8]>) {
    wire_pub_entropy_to_seed_impl(port_, entropy)
}

#[wasm_bindgen]
pub fn wire_bytes_to_hex(port_: MessagePort, bytes: Box<[u8]>) {
    wire_bytes_to_hex_impl(port_, bytes)
}

#[wasm_bindgen]
pub fn wire_hex_to_bytes(port_: MessagePort, hex: String) {
    wire_hex_to_bytes_impl(port_, hex)
}

#[wasm_bindgen]
pub fn wire_bytes_to_sha256(port_: MessagePort, bytes: Box<[u8]>) {
    wire_bytes_to_sha256_impl(port_, bytes)
}

#[wasm_bindgen]
pub fn wire_pub_master_to_wallet_unhardened_intermediate(port_: MessagePort, master: Box<[u8]>) {
    wire_pub_master_to_wallet_unhardened_intermediate_impl(port_, master)
}

#[wasm_bindgen]
pub fn wire_pub_master_to_wallet_unhardened(port_: MessagePort, master: Box<[u8]>, idx: u32) {
    wire_pub_master_to_wallet_unhardened_impl(port_, master, idx)
}

#[wasm_bindgen]
pub fn wire_pub_master_to_wallet_hardened_intermediate(port_: MessagePort, master: Box<[u8]>) {
    wire_pub_master_to_wallet_hardened_intermediate_impl(port_, master)
}

#[wasm_bindgen]
pub fn wire_pub_master_to_wallet_hardened(port_: MessagePort, master: Box<[u8]>, idx: u32) {
    wire_pub_master_to_wallet_hardened_impl(port_, master, idx)
}

#[wasm_bindgen]
pub fn wire_pub_master_to_pool_singleton(
    port_: MessagePort,
    master: Box<[u8]>,
    pool_wallet_idx: u32,
) {
    wire_pub_master_to_pool_singleton_impl(port_, master, pool_wallet_idx)
}

#[wasm_bindgen]
pub fn wire_pub_master_to_pool_authentication(
    port_: MessagePort,
    sk: Box<[u8]>,
    pool_wallet_idx: u32,
    idx: u32,
) {
    wire_pub_master_to_pool_authentication_impl(port_, sk, pool_wallet_idx, idx)
}

#[wasm_bindgen]
pub fn wire_cmds_program_run(port_: MessagePort, args: JsValue) {
    wire_cmds_program_run_impl(port_, args)
}

#[wasm_bindgen]
pub fn wire_cmds_program_brun(port_: MessagePort, args: JsValue) {
    wire_cmds_program_brun_impl(port_, args)
}

#[wasm_bindgen]
pub fn wire_cmd_program_opc(port_: MessagePort, args: JsValue) {
    wire_cmd_program_opc_impl(port_, args)
}

#[wasm_bindgen]
pub fn wire_cmd_program_opd(port_: MessagePort, args: JsValue) {
    wire_cmd_program_opd_impl(port_, args)
}

#[wasm_bindgen]
pub fn wire_cmd_program_cldb(port_: MessagePort, args: JsValue) {
    wire_cmd_program_cldb_impl(port_, args)
}

#[wasm_bindgen]
pub fn wire_program_tree_hash(port_: MessagePort, ser_program_bytes: Box<[u8]>) {
    wire_program_tree_hash_impl(port_, ser_program_bytes)
}

#[wasm_bindgen]
pub fn wire_program_curry(port_: MessagePort, ser_program_bytes: Box<[u8]>, args_str: JsValue) {
    wire_program_curry_impl(port_, ser_program_bytes, args_str)
}

#[wasm_bindgen]
pub fn wire_program_uncurry(port_: MessagePort, ser_program_bytes: Box<[u8]>) {
    wire_program_uncurry_impl(port_, ser_program_bytes)
}

#[wasm_bindgen]
pub fn wire_program_from_list(port_: MessagePort, program_list: JsValue) {
    wire_program_from_list_impl(port_, program_list)
}

#[wasm_bindgen]
pub fn wire_program_disassemble(port_: MessagePort, ser_program_bytes: Box<[u8]>) {
    wire_program_disassemble_impl(port_, ser_program_bytes)
}

#[wasm_bindgen]
pub fn wire_program_run(port_: MessagePort, ser_program_bytes: Box<[u8]>, args_str: JsValue) {
    wire_program_run_impl(port_, ser_program_bytes, args_str)
}

#[wasm_bindgen]
pub fn wire_program_from_atom_bytes(port_: MessagePort, ser_program_bytes: Box<[u8]>) {
    wire_program_from_atom_bytes_impl(port_, ser_program_bytes)
}

#[wasm_bindgen]
pub fn wire_program_to_atom_bytes(port_: MessagePort, ser_program_bytes: Box<[u8]>) {
    wire_program_to_atom_bytes_impl(port_, ser_program_bytes)
}

#[wasm_bindgen]
pub fn wire_get_puzzle_from_public_key(port_: MessagePort, pk: Box<[u8]>) {
    wire_get_puzzle_from_public_key_impl(port_, pk)
}

#[wasm_bindgen]
pub fn wire_cats_create_cat_puzzle(
    port_: MessagePort,
    tail_hash: Box<[u8]>,
    inner_puzzle_hash: Box<[u8]>,
) {
    wire_cats_create_cat_puzzle_impl(port_, tail_hash, inner_puzzle_hash)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<Vec<String>> for JsValue {
    fn wire2api(self) -> Vec<String> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}

impl Wire2Api<Vec<u32>> for Box<[u32]> {
    fn wire2api(self) -> Vec<u32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u32>> for JsValue {
    fn wire2api(self) -> Vec<u32> {
        self.unchecked_into::<js_sys::Uint32Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
impl Wire2Api<usize> for JsValue {
    fn wire2api(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
