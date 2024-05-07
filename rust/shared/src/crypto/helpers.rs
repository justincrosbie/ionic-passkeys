use hmac::Hmac;
use rand::{thread_rng, RngCore};
use sha2::{Sha256, Digest as shaDigest};
use ripemd::Ripemd160;
use sha3::Keccak256;


const PBKDF2_ROUNDS: u32 = 2048;
const PBKDF2_BYTES: usize = 64;

pub(crate) fn get_seed(secret: &Vec<u8>, password: &str) -> Vec<u8> {
    let bytes = pbkdf2(&secret, password);

    bytes
}

pub(crate) fn gen_random_bytes() -> Vec<u8> {
    let byte_length = 256 /8;
    let mut rng = thread_rng();
    let mut bytes = vec![0u8; byte_length];

    rng.fill_bytes(&mut bytes);

    bytes
}

pub(crate) fn pbkdf2(input: &[u8], salt: &str) -> Vec<u8> {
    let mut seed = vec![0u8; PBKDF2_BYTES];

    let _ = pbkdf2::pbkdf2::<Hmac<sha2::Sha512>>(input, salt.as_bytes(), PBKDF2_ROUNDS, &mut seed);

    seed
}

pub(crate) fn checksum(hex: &Vec<u8>) -> Vec<u8> {
    let mut hash = hash256(hex);
    _ = hash.split_off(4);

    hash
}

pub(crate) fn hash256(hex: &Vec<u8>) -> Vec<u8> {
    let hash1 = Sha256::digest(hex);
    let hash2 = Sha256::digest(hash1);

    hash2.to_vec()
}

pub(crate) fn hash160(hex: &Vec<u8>) -> Vec<u8> {
    let hash1 = Sha256::digest(hex);
    let hash2 = Ripemd160::digest(hash1);

    hash2.to_vec()
}

pub(crate) fn keccak(hex: &Vec<u8>) -> Vec<u8> {
    let hash = Keccak256::digest(hex);

    hash.to_vec()
}

#[cfg(test)]
pub(crate) fn parse_hex(hex_asm: &str) -> Vec<u8> {
    let mut hex_bytes = hex_asm.as_bytes().iter().filter_map(|b| {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }).fuse();

    let mut bytes = Vec::new();
    while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
        bytes.push(h << 4 | l)
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::{gen_random_bytes, get_seed, checksum, hash160, parse_hex};

    #[test]
    fn random_bytes_size() {
        let result = gen_random_bytes();
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn gen_seed() {
        let secret = gen_random_bytes();
        let password = hex::encode(gen_random_bytes());
        let result = get_seed(&secret, &password);
        println!("{:X?}", result);
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn check_checksum() {
        let hex: Vec<u8> = parse_hex("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        let result: Vec<u8> = checksum(&hex);
        assert_eq!(result, parse_hex("05c4de7c"));
    }

    #[test]
    fn get_hash160() {
        let result = hash160(&parse_hex("02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737"));
        assert_eq!(result, parse_hex("93ce48570b55c42c2af816aeaba06cfee1224fae"));
    }

}