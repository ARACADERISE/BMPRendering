use std::fs::File;
use std::io::Write;
use std::io;

use super::render;
use self::render::BmpImg;
use self::render::BmpImgFuncs;
use self::render::FileErrors;
use self::render::FileErrorsFuncs;

use self::render::PixelErrors;
use self::render::PixelErrorsFuncs;

impl PixelErrorsFuncs for PixelErrors
{
    fn invalid_pixel(pixel: char) -> PixelErrors
    {
        PixelErrors::InvalidPixel(pixel)
    }
    fn unknown_pixel(pixel: char) -> PixelErrors
    {
        PixelErrors::UnknownPixel(pixel)
    }
}

impl FileErrorsFuncs for FileErrors {
    fn err(err: io::Error) -> FileErrors
    {
        FileErrors::FileErr(err)
    }
}

impl BmpImgFuncs for BmpImg
{
    fn new_bmp() -> Self {
        Self {
            info_header: vec!['B', 'M'],
            header: vec![
                0, 0x00,
                0x36, 0x28,
                0, 0, // width, height
                0x1800a1,
                0, 0,
                0x002e23, 0x002e23, 0
            ],
            bmp_image: Vec::new()
        }
    }

    fn check_pixel(&mut self, pixel: u8) -> Result<char, PixelErrors>
    {
        if pixel < 10 || pixel > 250 {
            if !(pixel == 0x00)
            {
                return Err(PixelErrors::invalid_pixel(pixel as char));
            }
        }

        return Ok(pixel as char);
    }

    fn assign(&mut self, pixels: Vec<char>) -> Result<BmpImg, PixelErrors>
    {
        self.bmp_image = pixels;

        for i in 0..self.bmp_image.len()
        {
            match self.check_pixel(self.bmp_image[i] as u8) {
                Ok(_) => {}
                Err(e) => return Err(e)
            }
        }
        Ok(self.clone())
    }

    fn write_image(&mut self) -> Result<BmpImg, FileErrors>
    {
        match File::create("img.bmp".to_string()) {
            Ok(mut f) => {
                let mut image: Vec<u8> = Vec::new();

                for i in 0..self.bmp_image.len() {
                    image.push(i as u8);
                }
                f.write_all(&image);
            },
            Err(e) => return Err(FileErrors::err(e))
        }

        Ok(self.clone())
    }
}