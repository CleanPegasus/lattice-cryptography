use num_bigint::BigInt;


pub struct LatticePoint {
    coordinates: Vec<BigInt>
}

impl LatticePoint {
  pub fn new(coordinates: Vec<BigInt>) -> Self {
      Self { coordinates }
  }

  pub fn coordinates(&self) -> Vec<BigInt> {
      self.coordinates.clone()
  }

  pub fn dimension(&self) -> usize {
    self.coordinates.len()
  }

  pub fn index_at(&self, index: u64) -> &BigInt {
      &self.coordinates[index as usize]
  }

  pub fn multiply_scalar(&self, scalar: &BigInt) -> Self {
      let new_coordinates = self.coordinates.iter().map(|x| scalar * x).collect();
      Self { coordinates: new_coordinates }
  }

  pub fn add_points(&self, other: &LatticePoint) -> Result<Self, LatticePointError> {
    match self.verify_dimension(other) {
        Ok(_) => {
          let mut result_coordinates = Vec::new();

          for i in 0..self.dimension() {
            result_coordinates.push(self.index_at(i as u64) + other.index_at(i as u64))
          }

          return Ok(Self { coordinates: result_coordinates })
        },

        Err(e) => return Err(e)
    }
  }

  pub fn distance_from_point(&self, other: &LatticePoint) -> Result<BigInt, LatticePointError> {
    match self.verify_dimension(other) {
      Ok(_) => {
        let mut difference_vector = Vec::new();

        for i in 0..self.dimension() {
          difference_vector.push(self.index_at(i as u64))
        }

        let mut distance = BigInt::ZERO;

        for point in difference_vector {
          distance += point.pow(2)
        }

        return Ok(distance);
      },

      Err(e) => return Err(e)
  }
  }

  fn verify_dimension(&self, lattice_point: &LatticePoint) -> Result<(), LatticePointError> {
    if self.coordinates.len() == lattice_point.coordinates().len() {
        Ok(())
    } else {
        Err(LatticePointError::DimensionMismatch { expected: self.dimension(), found: lattice_point.dimension() })
    }
}

}

pub enum LatticePointError {
  DimensionMismatch {expected: usize, found: usize}
}