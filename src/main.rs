use num_bigint::{BigInt, Sign, RandBigInt};

use lattice_cryptography::{LatticeParams, key_gen, encrypt, decrypt, Cipher, PublicKey, SecretKey};


fn main() {
    let params = LatticeParams::new(
        256,
        BigInt::from(65537), // A prime number
        BigInt::from(16),
    );

    let (public_key, secret_key) = key_gen(&params);

    for message in 0..2 {
        println!("Original message: {}", message);
        let ciphertext = encrypt(&params, &public_key, message);
        let decrypted = decrypt(&params, &ciphertext, &secret_key);
        println!("Decrypted message: {}", decrypted);
        assert_eq!(message, decrypted);
        println!("-------------------------");
    }
}