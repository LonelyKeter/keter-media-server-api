use crate::req_prelude::*;
use super::types::*;



#[get("/?<title>&<kind>&<author>&<rating>&<ord>", format = "json")]
pub async fn get_base(
  title: Option<String>,
  kind: Option<MediaType>,
  author: Option<String>,
  rating: Option<u32>,
  ord: Option<Ordering>) 
    -> Json<Vec<MediaInfo>> {
  Json(vec![Media::new(12345)])
}

#[get("/<id>", format = "json")]
pub async fn get_concrete(id: u128) -> Json<Media> {
  unimplemented!();
}

#[post("/media", format = "json")]
pub async fn post_product(reg_media: Json<RegisterMedia>) {

}