use std::io::Write;
use futures_util::StreamExt;

pub async fn upload(mut payload: actix_multipart::Multipart) -> Result<String, crate::error::Error> {
    while let Some(field) = payload.next().await {
        let mut buffer = Vec::new();
        let mut field = match field {
            Ok(v) => v,
            Err(err) => {
                log::error!("Error.");
                log::debug!("{:?}", err);

                return Err(
                    crate::error::Error {
                        code: 48,
                        message: String::from("Error.")
                    }
                );
            }
        };

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(v) => v,
                Err(err) => {
                    log::error!("Error.");
                    log::debug!("{:?}", err);

                    return Err(
                        crate::error::Error {
                            code: 49,
                            message: String::from("Error.")
                        }
                    );
                }
            };

            // Prevent uploads larger than MAX_UPLOAD_SIZE
            // Bad request on error
            if buffer.len() + data.len() > 10 * 1024 * 1024 {
                log::warn!("The user trying to upload file larger than 10MB.");

                return Err(
                    crate::error::Error {
                        code: 50,
                        message: String::from("Upload too large. Max 10MB allowed.")
                    }
                );
            }

            buffer.extend_from_slice(&data);
        }

        // Detect MIME type
        // Bad request on error
        let kind = infer::get(&buffer);
        let mime_type = kind.map(|k| k.mime_type()).unwrap_or("application/octet-stream");
        let allowed_types = ["image/jpeg", "image/png", "image/webp", "image/gif"];

        if !allowed_types.contains(&mime_type) {
            log::warn!("The user trying to upload non image file.");

            return Err(
                crate::error::Error {
                    code: 51,
                    message: String::from("Only image files are allowed.")
                }
            );
        }

        let extension = kind.map(|k| k.extension()).unwrap_or("bin");
        let filename = format!("{}.{}", uuid::Uuid::now_v7(), extension);
        let filepath = format!("./uploads/{}", filename);

        let mut file = match std::fs::File::create(&filepath) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to create an uploaded file.");
                log::debug!("{:?}", e);

                return Err(
                    crate::error::Error {
                        code: 52,
                        message: String::from("Failed to create an uploaded file.")
                    }
                );
            }
        };
        
        match file.write_all(&buffer) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Failed to write to uploaded file.");
                log::debug!("{:?}", e);

                return Err(
                    crate::error::Error {
                        code: 53,
                        message: String::from("Failed to write to uploaded file.")
                    }
                );
            }
        };

        let url = format!("/uploads/{}", filename);

        return Ok(url);
    }

    log::warn!("No file uploaded.");
    // Bad request
    return Err(
        crate::error::Error {
            code: 54,
            message: String::from("No file uploaded.")
        }
    );
}