use crate::gfx::Size;
use fast_image_resize::{images::Image, FilterType, PixelType, ResizeAlg, ResizeOptions, Resizer};

#[derive(Clone, Debug)]
pub struct Frame {
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum Error {
    Encode(sixel_bytes::SixelError),
    InvalidSize(Size<u32>),
}

impl From<sixel_bytes::SixelError> for Error {
    fn from(value: sixel_bytes::SixelError) -> Self {
        Self::Encode(value)
    }
}

impl Frame {
    fn encode_rgba(pixels: &[u8], size: Size<u32>) -> Result<Self, Error> {
        if size.width == 0 || size.height == 0 {
            return Err(Error::InvalidSize(size));
        }

        let bytes = sixel_bytes::sixel_string(
            pixels,
            size.width as i32,
            size.height as i32,
            sixel_bytes::PixelFormat::BGRA8888,
            sixel_bytes::DiffusionMethod::Ordered,
        )
        .map_err(Error::from)?
        .into_bytes();

        Ok(Self { bytes })
    }

    pub fn from_viewport_scaled(
        pixels: &[u8],
        src_size: Size<u32>,
        target: Size<u32>,
    ) -> Result<Self, Error> {
        if target.width == 0 || target.height == 0 || src_size.width == 0 || src_size.height == 0 {
            return Err(Error::InvalidSize(target));
        }

        if src_size == target {
            return Self::encode_rgba(pixels, src_size);
        }

        let src = Image::from_vec_u8(
            src_size.width,
            src_size.height,
            pixels.to_vec(),
            PixelType::U8x4,
        )
        .map_err(|_| Error::InvalidSize(src_size))?;
        let mut dst = Image::new(target.width, target.height, PixelType::U8x4);
        let mut resizer = Resizer::new();
        let options =
            ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::CatmullRom));
        resizer
            .resize(&src, &mut dst, &options)
            .map_err(|_| Error::InvalidSize(target))?;
        let buffer = dst.into_vec();

        Self::encode_rgba(&buffer, target)
    }
}
