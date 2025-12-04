use hex::decode_to_slice;
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn generate_random_seed() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub fn generate_chain(server_seed: &String, client_seed: &String, iterations: u32) -> Vec<String> {
    let mut variable_seed = [0u8; 32];
    let mut decoded_client_seed = [0u8; 32];
    let mut chain_vec: Vec<String> = Vec::new();

    decode_to_slice(&server_seed, &mut variable_seed).unwrap();
    decode_to_slice(&client_seed, &mut decoded_client_seed).unwrap();
    let mut mac =
        HmacSha256::new_from_slice(&variable_seed).expect("HMAC can take key of any size");
    mac.update(&decoded_client_seed);
    let result = mac.finalize();
    chain_vec.push(hex::encode(result.into_bytes()));

    for _ in 0..iterations {
        decode_to_slice(chain_vec.last().unwrap(), &mut decoded_client_seed).unwrap();
        mac = HmacSha256::new_from_slice(&variable_seed).expect("HMAC can take key of any size");
        mac.update(&decoded_client_seed);
        let result = mac.finalize();
        chain_vec.push(hex::encode(result.into_bytes()));
    }

    chain_vec
}
