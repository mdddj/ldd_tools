use flutter_rust_bridge::frb;
use magick_rust::{magick_wand_genesis, MagickWand};




unsafe  impl Sync for LddMagickTool{}
pub struct LddMagickTool {
    wand: MagickWand
}


impl LddMagickTool {

    ///初始化
    #[frb(sync)]
    pub fn new() -> LddMagickTool {
        magick_wand_genesis();
        let wand: MagickWand = MagickWand::new();
        LddMagickTool { wand }
    }

    ///转灰度图
    pub fn covert_to_gray_color_image(&self,data: &[u8],format: String) -> Vec<u8> {
        let mut wand = self.wand.clone();
        wand.read_image_blob(data).unwrap();
        wand.set_colorspace(magick_rust::ColorspaceType::GRAY).unwrap();
        wand.ordered_dither_image("h4x4a").unwrap();
        wand.write_image_blob(&format).unwrap()
    }

    ///转黑白图
    pub fn cover_to_monochrome(&self,data: &[u8],format: String) -> Vec<u8>{
        let wand = MagickWand::new();
        wand.read_image_blob(data).unwrap();
        wand.set_monochrome();
        wand.write_image_blob(&format).unwrap()
    }
}