mod src;

use src::render;
use self::render::BmpImg;
use self::render::BmpImgFuncs;

fn main() {
  let mut bmp_info = BmpImg::new_bmp();
  match bmp_info.assign(vec![
      0x35 as char,
      0x41 as char,
      0xef as char,
      0x00 as char
  ]) {
      Ok(t) => {
          bmp_info.write_image();   
      },
      Err(e) => panic!("{:?}", e)
  }
}