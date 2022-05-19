use keter_media_db::auth::roles::Auth;
use keter_media_model::{media::MaterialKey, usage::*};
use rocket::fairing::AdHoc;

use crate::{auth::*, utility::*};
