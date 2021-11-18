use rocket::{
    http::{hyper::header::CONTENT_DISPOSITION, ContentType, Header},
    response::{Responder, Response},
    tokio::fs::File,
};

use std::ffi::OsString;

///This responder properties will be joined with inner
pub struct FileResponce {
    file: File,
    file_name: Option<OsString>,
    file_ext: Option<OsString>,
}

impl FileResponce {
    pub fn new(file: File) -> Self {
        Self {
            file,
            file_name: None,
            file_ext: None,
        }
    }

    pub fn with_file_name(&mut self, file_name: OsString) -> &mut Self {
        self.file_name = Some(file_name);
        self
    }

    pub fn with_extenstion(&mut self, extension: OsString) -> &mut Self {
        self.file_ext = Some(extension);
        self
    }

    pub fn extension_from_file_name(&mut self) -> &mut Self {
        if let Some(name) = &self.file_name {
            let path = std::path::Path::new(&name);

            self.file_ext = path.extension().map(OsString::from);
            self.file_name = path.file_stem().map(OsString::from);
        } else {
            self.file_ext = None;
        }

        self
    }
}

impl<'r> Responder<'r, 'static> for FileResponce {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let mut builder = Response::build_from(self.file.respond_to(request)?);

        let mut file_name = if let Some(name) = self.file_name {
            name
        } else {
            OsString::from("unknown")
        };

        if let Some(ext) = self.file_ext {
            file_name.push(".");
            file_name.push(&ext);

            if let Some(content_type) = ContentType::from_extension(ext.to_str().unwrap()) {
                builder.header(content_type);
            } else {
                builder.header(ContentType::Binary);
            }
        }

        let content_disposition_value =
            format!("attachment; filename = \"{}\"", file_name.to_str().unwrap());

        builder.header(Header::new(CONTENT_DISPOSITION.as_str(), content_disposition_value));

        Ok(builder.finalize())
    }
}
