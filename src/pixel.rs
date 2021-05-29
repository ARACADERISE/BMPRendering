use std::path::PathBuf;
use std::io;

#[derive(Debug)]
pub enum _Err
{
    NoSuchFile(io::Error),
    InvalidStart(Vec<u8>),
    InvalidRGB(char),
    InvalidSize(i32)
}

#[derive(Debug, Clone)]
pub struct BmpFileInfo
{
    pub path: PathBuf,
    pub got: Vec<u8>,
    pub rgb_vals: Vec<u8>
}

pub trait BmpFileInfoFuncs
{
    fn new(filename: String, info_header: Vec<u8>) -> Result<BmpFileInfo, _Err>;
    fn read_data(&mut self);
    fn mult(&mut self);
}

pub trait _ErrFuncs
{
    fn invalid_start(start: Vec<u8>) -> _Err;
    fn invalid_rgb(rgb: char) -> _Err;
    fn invalid_size(size: i32) -> _Err;
}