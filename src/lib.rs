use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::Aes128;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn general_request_id(time_stamp: u32) -> String {
    let key_bytes = b"SGi8#jsoigai10s2";
    let key = GenericArray::from_slice(&key_bytes[..16]);
    let cipher = Aes128::new(&key);
    let raw_data = time_stamp.to_be_bytes();
    let mut convert_block = [0u8; 16];
    convert_block[..4].copy_from_slice(&raw_data);
    let content_block = GenericArray::from_mut_slice(&mut convert_block);
    cipher.encrypt_block(content_block);
    hex::encode(content_block)
}



