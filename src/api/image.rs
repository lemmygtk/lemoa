use crate::settings;
use rand::distributions::{Alphanumeric, DistString};
use reqwest::blocking::multipart::Part;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use super::CLIENT;

#[derive(Deserialize)]
pub struct UploadImageResponse {
    #[allow(dead_code)]
    msg: String,
    files: Vec<UploadImageFile>,
}

#[derive(Deserialize)]
struct UploadImageFile {
    pub file: String,
    #[allow(dead_code)]
    pub delete_token: String,
}

pub fn upload_image(image: std::path::PathBuf) -> Result<String, reqwest::Error> {
    let mime_type = mime_guess::from_path(image.clone()).first();
    let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let mut file = File::open(image).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let part = Part::bytes(data)
        .file_name(file_name)
        .mime_str(mime_type.unwrap().essence_str())?;
    let form = reqwest::blocking::multipart::Form::new().part("images[]", part);
    let account = settings::get_current_account();
    let base_url = account.instance_url;
    let path = format!("{}/pictrs/image", base_url);
    let res: UploadImageResponse = CLIENT
        .post(path)
        .header(
            "cookie",
            format!("jwt={}", account.jwt.unwrap().into_inner()),
        )
        .multipart(form)
        .send()?
        .json()?;

    Ok(format!("{}/pictrs/image/{}", base_url, res.files[0].file))
}
