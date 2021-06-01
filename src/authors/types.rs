use crate::type_prelude::*;

#[derive(Serialize)]
pub struct Author {
  id: u64,  
} 

impl Author {
  pub fn new(id: u64) -> Self {
    Self { id: id}
  }
}