use num_bigint::{BigInt, Sign, RandBigInt};
use rand::{Rng, thread_rng};
use num_traits::{Zero, One, ToPrimitive, FromPrimitive};

pub struct LatticeParams {
  dimension: usize,
  modulus: BigInt,
  error_range: BigInt
}

impl LatticeParams {
    pub fn new(dimension: usize, modulus: BigInt, error_range: BigInt) -> Self {
        Self {dimension, modulus, error_range}
    }
}

pub struct PublicKey {
  a: Vec<Vec<BigInt>>,
  b: Vec<BigInt>
}

impl PublicKey {
  fn new(a: Vec<Vec<BigInt>>, b: Vec<BigInt>) -> Self {
    Self { a, b }
  }
}

pub struct SecretKey {
  s: Vec<BigInt>
}

impl SecretKey {
  fn new(s: Vec<BigInt>) -> Self {
    Self { s }
  }
}

pub struct Cipher {
  c1: Vec<BigInt>,
  c2: BigInt
}

impl Cipher {
  fn new(c1: Vec<BigInt>, c2: BigInt) -> Self {
    Self {c1, c2}
  }
}

fn generate_random_vector(size: usize, modulus: &BigInt) -> Vec<BigInt> {
  let mut rng = rand::thread_rng();
  (0..size).map(|_| rng.gen_bigint_range(&BigInt::ZERO, modulus)).collect()
}

fn matrix_mul(a: &Vec<Vec<BigInt>>, b: &Vec<BigInt>, modulus: &BigInt) -> Vec<BigInt> {
  let mut result = Vec::new();
  for row in a {
    let mut result_index_value = BigInt::ZERO;
    for (index, value) in row.iter().enumerate() {
      result_index_value += value * b[index].clone()
    }
    result.push(result_index_value % modulus)
  }
  result
}

fn matrix_add(a: &Vec<BigInt>, b: &Vec<BigInt>, modulus: &BigInt) -> Vec<BigInt> {
  a.iter().zip(b.iter()).map(|(a, b)| (a + b) % modulus).collect()
}

/// generate public and private keys
pub fn key_gen(params: &LatticeParams) -> (PublicKey, SecretKey) {
  // secret key
  let s = generate_random_vector(params.dimension, &params.modulus);

  let a: Vec<Vec<BigInt>> = (0..params.dimension).map(|_| generate_random_vector(params.dimension, &params.modulus)).collect();
  let e = generate_random_vector(params.dimension, &params.error_range);
  let b = matrix_add(&matrix_mul(&a, &s, &params.modulus), &e, &params.modulus);

  let public_key = PublicKey::new(a, b);
  let secret_key = SecretKey::new(s);

  (public_key, secret_key)
}

/// Encrypt the message
pub fn encrypt(params: &LatticeParams, public_key: &PublicKey, message: u8) -> Cipher {
  
  let e = generate_random_vector(params.dimension, &params.error_range);
  let m = BigInt::from(message);

  let c1 = matrix_mul(&public_key.a, &e, &params.modulus);
  let c2 = (matrix_mul(&vec![e.clone()], &public_key.b, &params.modulus)[0].clone() + &params.modulus / 2 * m ) % &params.modulus;

  return Cipher::new(c1, c2);
}


/// Decrypt the message
pub fn decrypt(params: &LatticeParams, cipher: &Cipher, secret_key: &SecretKey) -> u8 {
    dbg!(&cipher.c2 - matrix_mul(&vec![cipher.c1.clone()], &secret_key.s, &params.modulus)[0].clone());
//   let m = (&cipher.c2 - matrix_mul(&vec![cipher.c1.clone()], &secret_key.s, &params.modulus)[0].clone()) % &params.modulus;
  let m = (&cipher.c2 - matrix_mul(&vec![cipher.c1.clone()], &secret_key.s, &params.modulus)[0].clone() + &params.modulus) % &params.modulus;
  let threshold = &params.modulus / 4;
  if m <= threshold || m > &params.modulus - threshold {
      0
  } else {
      1
  }
}