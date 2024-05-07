use base58::ToBase58;
use rand::{thread_rng, Rng};
use super::helpers::hash256;

pub fn gen_otp() -> String {
    let mut rng = thread_rng();
    let num = rng.gen_range(0..=999999);
    let otp = format!("{:0>6}", num);
    otp
}

pub fn hash_otp(otp: &String) -> String {
    hash256(&otp.as_bytes().to_vec()).to_base58()
}
