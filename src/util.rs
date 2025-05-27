use base64::{engine::general_purpose, Engine};

#[derive(Debug, thiserror::Error)]
pub enum GenerateQrCodeError {
    #[error("error when creating qrcode instance")]
    CreatingInstance(#[from] qrcode::types::QrError),
    #[error("error when writing qrcode as image")]
    WriteAsImage(#[from] image::ImageError),
}

pub fn generate_qr_code(text: &str, size: (u32, u32)) -> Result<String, GenerateQrCodeError> {
    let qr = qrcode::QrCode::new(text)?
        .render::<image::Luma<u8>>()
        .min_dimensions(size.0, size.1)
        .build();

    let mut image_bytes = Vec::<u8>::new();
    qr.write_to(
        &mut std::io::Cursor::new(&mut image_bytes),
        image::ImageFormat::Png,
    )?;

    Ok(general_purpose::STANDARD.encode(image_bytes))
}

pub fn base64qr(text: &str, size: (u32, u32)) -> Result<String, GenerateQrCodeError> {
    Ok(format!(
        "data:image/png;base64,{}",
        generate_qr_code(text, size)?
    ))
}
