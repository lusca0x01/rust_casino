use hex::decode_to_slice;

fn is_divisible(hash: &str) -> bool {
    let mut val: u128 = 0;
    let mut decoded_chunk = [0u8; 2];

    for i in (0..hash.len()).step_by(4) {
        if i + 4 <= hash.len() {
            decode_to_slice(&hash[i..i + 4], &mut decoded_chunk).unwrap();
            let chunk_val = u16::from_be_bytes(decoded_chunk);
            val = (val << 16) + (chunk_val as u128 % 15);
        }
    }

    val == 0
}

pub fn get_point(hash: &str) -> f64 {
    if is_divisible(hash) {
        return 0.0;
    }

    let mut total_chunk: [u8; 8] = [0u8; 8];
    let mut decoded_chunk: [u8; 6] = [0u8; 6];

    decode_to_slice(&hash[0..12], &mut decoded_chunk).unwrap();
    total_chunk[2..8].copy_from_slice(&decoded_chunk);

    let mut h = u64::from_be_bytes(total_chunk);
    let e = 2u64.pow(52);

    while h < 1_000_000_000_000_000 {
        h *= 10;
    }

    let point = (100 * e - h) / (e - h);

    point as f64 / 100.0
}
