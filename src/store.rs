use keter_media_model::media::MaterialKey;
use rocket::tokio::io::AsyncWriteExt;
use rocket::{fs::TempFile, tokio};
use std::path::PathBuf;
use std::{ffi::OsString, path::Path};

pub struct MaterialStore {
    root: OsString,
}

impl MaterialStore {
    pub async fn init(root: impl AsRef<Path>) -> Result<Self, Error> {
        let root = if root.as_ref().is_absolute() {
            PathBuf::from(root.as_ref())
        } else {
            let mut absolute = std::env::current_exe().map_err(Error::Path)?;
            absolute.pop();
            absolute.push(root.as_ref());

            absolute
        }; 

        if !root.exists() {
            tokio::fs::create_dir_all(&root)
                .await
                .map_err(Error::CreateRoot)?;
        }

        if root.is_dir() {
            Ok(Self { root: root.into() })
        } else {
            Err(Error::InvalidRoot)
        }
    }

    fn create_material_path(&self, material_id: MaterialKey) -> Result<OsString, std::io::Error> {
        let mut buff = PathBuf::from(&self.root);
        buff.push(material_id.to_string());

        Ok(buff.into_os_string())
    }

    pub async fn save_material(
        &self,
        material_id: MaterialKey,
        mut file: TempFile<'_>,
    ) -> Result<SaveMaterialData, Error> {
        let path = self
            .create_material_path(material_id)
            .map_err(Error::Path)?;

        //TODO: Exchange copy with persist
        file.persist_to(&path).await.map_err(Error::TempFile)?;
        let meta = tokio::fs::metadata(&path).await.map_err(Error::Metadata)?;

        Ok(SaveMaterialData { size: meta.len() })
    }

    pub async fn get_material(&self, material_id: MaterialKey) -> Result<tokio::fs::File, Error> {
        let path = self
            .create_material_path(material_id)
            .map_err(Error::Path)?;

        let file = tokio::fs::File::open(path).await.map_err(Error::OpenFile)?;

        Ok(file)
    }

    pub async fn remove_material(&self, material_id: MaterialKey) -> Result<(), Error> {
        let path = self
            .create_material_path(material_id)
            .map_err(Error::Path)?;

        let path = Path::new(&path);

        if path.is_file() {
            tokio::fs::remove_file(path)
                .await
                .map_err(Error::RemoveFile)
        } else {
            Err(Error::Unexpected)
        }
    }
}

pub struct SaveMaterialData {
    size: u64,
}

#[derive(Debug)]
pub enum Error {
    InvalidRoot,
    Path(std::io::Error),
    TempFile(std::io::Error),
    OpenFile(std::io::Error),
    RemoveFile(std::io::Error),
    CreateRoot(std::io::Error),
    Metadata(std::io::Error),
    Unexpected,
}
