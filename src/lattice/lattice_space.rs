use num_bigint::BigInt;
use super::LatticePoint;

pub struct LatticeSpace {
  dimension: u64
}

impl LatticeSpace {

  fn new(dimension: u64) -> Self {
      Self { dimension }
  }

  fn create_random_lattice_point() -> LatticePoint {
    unimplemented!()
  }

}
