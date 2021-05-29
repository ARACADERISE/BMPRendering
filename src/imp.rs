use super::pixel;

use self::pixel::BmpFileInfo;
use self::pixel::BmpFileInfoFuncs;
use self::pixel::_Err;
use self::pixel::_ErrFuncs;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::env;
use std::path::PathBuf;

impl From<io::Error> for _Err
{
    fn from(err: io::Error) -> _Err
    {
        _Err::NoSuchFile(err)
    }
}

impl _ErrFuncs for _Err
{
    fn invalid_start(start: Vec<u8>) -> _Err
    {
        _Err::InvalidStart(start)
    }
    fn invalid_rgb(rgb: char) -> _Err
    {
        _Err::InvalidRGB(rgb)
    }
    fn invalid_size(size: i32) -> _Err
    {
        _Err::InvalidSize(size)
    }
}

/*
 * This function will generate a new rgb value for the current pixel
 * depending on the current rgb value.

 Example:
    [53, 65, 239]
          =
    [104, 128, 220]
 */
priv fn generate(mut from_rgb: u8) -> u8
{
    from_rgb <<= 1;
    from_rgb ^= 2;

    from_rgb
}

impl BmpFileInfoFuncs for BmpFileInfo
{
    /*
     * Initialize the BmpFileInfo struct to have information
     * about the BMP file.
     *
     * Error if the file doesn't exist.
     */
    fn new(filename: String, info_header: Vec<u8>) -> Result<BmpFileInfo, _Err>
    {

        let path: PathBuf = env::current_dir()?.to_path_buf().join(filename);

        let mut info = BmpFileInfo{
            path: path,
            got: Vec::new(),
            rgb_vals: Vec::new()
        };

        if info.path.clone().exists()
        {
            let mut f = File::open(info.path.clone()).expect("No such file");

            let meta = fs::metadata(info.path.clone()).expect("cannot read metadata");

            let mut buffer = vec![0; meta.len() as usize];
            f.read(&mut buffer).expect("Couldn't read");

            info.got = buffer;

            let mut arr: Vec<u8> = Vec::new();

            for i in 0..info_header.len() {
                if !(info.got[i] == info_header[i])
                {
                    arr.push(info.got[i]);
                    if i == info_header.len() - 1
                    {
                        return Err(_Err::InvalidStart(arr))
                    }
                }
            }
        }

        Ok(info)
    }

    fn read_data(&mut self)
    {
        let start_of_pixel_array = self.got[2] as usize;
        let mut index = start_of_pixel_array;

        let mut pixel_array: Vec<u8> = Vec::new();

        while self.got[index] != 0x00
        {
            pixel_array.push(self.got[index]);
            index += 1;
        }

        self.rgb_vals = pixel_array;
    }

    /*
     * This will take the rgb values and generate new rgb pixel
     * values, dependable.
     */
    fn mult(&mut self)
    {
        for i in 0..self.rgb_vals.len() {
            self.rgb_vals[i] = generate(self.rgb_vals[i]);
        }
    }
}