mod src;

use self::src::pixel;
use self::pixel::BmpFileInfo;
use self::pixel::BmpFileInfoFuncs;

fn main() {
    match BmpFileInfo::new("img.bmp".to_string(), vec!['B' as u8, 'M' as u8]) {
        Ok(mut t) => {
            t.read_data();
            t.mult();
        },
        Err(e) => panic!("{:?}", e)
    }
}