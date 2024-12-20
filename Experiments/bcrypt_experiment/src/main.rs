// use aes_gcm::{
//     aead::{Aead, AeadCore, KeyInit, OsRng},
//     Aes256Gcm, Key, Nonce,
// };
//
// use rand::distributions::Alphanumeric;
// use rand::{thread_rng, Rng};
//
// fn main() {
//     let plaintext = "backendengineer.io".to_string();
//
//     let key_str = "thiskeystrmustbe32charlongtowork".to_string();
//
//     let encrypted_data = encrypt(key_str.clone(), plaintext);
//
//     println!("encrypted_data: {:?}", encrypted_data.clone());
//
//     let original = decrypt(key_str, encrypted_data);
//
//     println!("original: {:?}", original);
//
//     let random_string = generate_random_string();
//     println!("Random String: {}", random_string);
// }
//
// fn generate_random_string() -> String {
//     let mut rng = thread_rng();
//     let random_string: String = (0..32)
//         .map(|_| rng.sample(Alphanumeric))
//         .map(char::from)
//         .collect();
//     random_string
// }
//
// fn encrypt(key_str: String, plaintext: String) -> String {
//     let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
//     let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
//
//     let cipher = Aes256Gcm::new(key);
//
//     let ciphered_data = cipher
//         .encrypt(&nonce, plaintext.as_bytes())
//         .expect("failed to encrypt");
//
//     // combining nonce and encrypted data together
//     // for storage purpose
//     let mut encrypted_data: Vec<u8> = nonce.to_vec();
//     encrypted_data.extend_from_slice(&ciphered_data);
//
//     hex::encode(encrypted_data)
// }
//
// fn decrypt(key_str: String, encrypted_string: String) -> String {
//     let encrypted_data =
//         hex::decode(encrypted_string).expect("failed to decode hex string into vec");
//
//     let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
//
//     let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
//     let nonce = Nonce::from_slice(nonce_arr);
//
//     let cipher = Aes256Gcm::new(key);
//
//     let plaintext = cipher
//         .decrypt(nonce, ciphered_data)
//         .expect("failed to decrypt data");
//
//     String::from_utf8(plaintext).expect("failed to convert vector of bytes to string")
// }
