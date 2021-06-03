use crate::type_prelude::*;

#[derive(Serialize, Deserialize)]
pub struct MediaInfo {
  pub id: u64,
  pub title: String,
  pub kind: MediaKind,
  pub author: AuthorInfo,
}

#[derive(Serialize, Deserialize)]
pub enum MediaKind {
  Audio,
  Video,
  Image
}

#[derive(Serialize, Deserialize)]
pub struct Media {
  pub info: MediaInfo,
  pub materials: Vec<MaterialInfo>
}

impl Media {

}

#[derive(Serialize, Deserialize)]
pub struct MaterialInfo {
  pub id: u64,
  pub format: String,
  pub quality: MaterialQuality,
  #[serde(rename = "licenseName")]
  pub license_name: String,
  #[serde(rename = "downloadLink")]
  pub download_link: String
}

//TODO: Material quality enumeration
#[derive(Serialize, Deserialize)]
pub enum MaterialQuality {

}

#[derive(Serialize, Deserialize)]
pub struct RegisterMedia {

}

#[derive(Serialize, Deserialize)]
pub struct Usage {

}