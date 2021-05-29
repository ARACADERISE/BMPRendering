use std::io;

#[derive(Debug, Clone)]
pub enum PixelErrors
{
    InvalidPixel(char),
    UnknownPixel(char),
}

pub enum FileErrors
{
    FileErr(io::Error)
}

#[derive(Debug, Clone)]
pub struct BmpImg
{
    pub info_header: Vec<char>,
    pub header: Vec<i32>,
    pub bmp_image: Vec<char>, // pixel array
}

pub trait BmpImgFuncs
{
    fn new_bmp() -> Self;
    fn check_pixel(&mut self, pixel: u8) -> Result<char, PixelErrors>;
    fn assign(&mut self, pixels: Vec<char>) -> Result<BmpImg, PixelErrors>; // assign pixels
    fn write_image(&mut self) -> Result<BmpImg, FileErrors>;
}

pub trait PixelErrorsFuncs
{
    fn invalid_pixel(pixel: char) -> PixelErrors;
    fn unknown_pixel(pixel: char) -> PixelErrors;
}

pub trait FileErrorsFuncs
{
    fn err(err: io::Error) -> FileErrors;
}