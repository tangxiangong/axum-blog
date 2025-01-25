use crate::{AppError, AppResult};
use axum::{
    body::Bytes,
    extract::{FromRequest, Multipart, Request},
};
pub struct Image(pub Bytes, pub String);

impl Image {
    pub fn extension(&self) -> &str {
        &self.1
    }

    pub fn data(&self) -> &[u8] {
        &self.0[..]
    }
}

impl<S> FromRequest<S> for Image
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = fetch_one(req, state).await?;
        if !infer::is_image(&bytes[..]) {
            return Err(AppError::bad_request("Invalid file format"));
        }
        let ext = infer::get(&bytes[..]).unwrap().extension().to_owned();
        Ok(Image(bytes, ext))
    }
}

async fn fetch_one<S>(req: Request, state: &S) -> AppResult<Bytes>
where
    S: Send + Sync,
{
    let mut mp = Multipart::from_request(req, state).await?;
    if let Some(field) = mp.next_field().await? {
        Ok(field.bytes().await?)
    } else {
        Err(AppError::bad_request("No file found"))
    }
}
