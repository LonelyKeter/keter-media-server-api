use crate::type_prelude::*;

#[derive(Serialize)]
pub struct Media {
  id: u64,  
} 

impl Media {
  pub fn new(id: u64) -> Self {
    Self { id: id}
  }
}