use axum::body::{BoxBody, Full};
use axum::headers::HeaderValue;
use axum::http::header::InvalidHeaderValue;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{body, http};
use image::io::Reader;
use image::ImageFormat;
use std::io::{Bytes, Cursor, Read};

pub struct Image(pub Vec<u8>);

impl IntoResponse for Image {
    fn into_response(self) -> Response {
        let bytes = self.0;

        let reader = image::io::Reader::new(Cursor::new(bytes.clone()));

        let guessed_format = match reader.with_guessed_format() {
            Ok(reader) => reader,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let format = match guessed_format.format() {
            Some(format) => format,
            None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let mime = match format {
            ImageFormat::Png => mime::IMAGE_PNG,
            ImageFormat::Jpeg => mime::IMAGE_JPEG,
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let content_type = match HeaderValue::from_str(mime.as_ref()) {
            Ok(val) => val,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let mut res = Response::new(body::boxed(Full::from(bytes)));
        res.headers_mut().insert(header::CONTENT_TYPE, content_type);
        res
    }
}
