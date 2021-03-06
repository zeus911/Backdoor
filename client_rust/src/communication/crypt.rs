use aes_gcm::Aes256Gcm;
use std::error::Error;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use rand::Rng;

pub fn decrypt(data: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let key = GenericArray::clone_from_slice(b"\xbch`9\xd6k\xcbT\xed\xa5\xef_\x9d*\xda\xd2sER\xedA\xc0a\x1b)\xcc9\xb2\xe7\x91\xc2A");
    let cipher: Vec<u8> = (&data[12..]).to_vec();
    let nonce = GenericArray::from_slice(&data[..12]);
    let aead = Aes256Gcm::new(key);
    let plain = aead.decrypt(nonce, cipher.as_ref()).expect("decryption failure!");
    Ok(plain)
}

pub fn encrypt(plain: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let key = GenericArray::clone_from_slice(b"\xbch`9\xd6k\xcbT\xed\xa5\xef_\x9d*\xda\xd2sER\xedA\xc0a\x1b)\xcc9\xb2\xe7\x91\xc2A");
    let rand_bytes = rand::thread_rng().gen::<[u8; 32]>();
    let nonce = GenericArray::clone_from_slice(&rand_bytes);
    let aead = Aes256Gcm::new(key);
    let cipher = aead.encrypt(&nonce, plain.as_ref()).expect("encryption failure!");
    Ok(cipher)
}
