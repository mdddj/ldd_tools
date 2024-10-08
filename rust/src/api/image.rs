use flutter_rust_bridge::frb;
use image::{GrayImage, ImageBuffer, ImageFormat, Luma};
use imageproc::contrast::{threshold, ThresholdType};
use std::io::Cursor;

use super::printer::download_bmp_image_tspl_command_data;

///图片数据
pub struct BitmapImage {
    pub bitmap: Vec<u8>,
    pub width: u32,
    pub height: u32,
}



impl BitmapImage {

    ///保存到文件
    #[frb(sync)]
    pub fn save_file(self,path: String){
        let image = image::load_from_memory(&self.bitmap).unwrap();
        let _ = image.save(path);
    }
}

//Options for how to treat the threshold value in [`threshold`] and [`threshold_mut`].
pub enum LddThresholdType {
    /// `dst(x,y) = if src(x,y) > threshold { 255 } else { 0 }`
    Binary,
    /// `dst(x,y) = if src(x,y) > threshold { 0 } else { 255 }`
    BinaryInverted,
    /// `dst(x,y) = if src(x,y) > threshold { threshold } else { src(x,y) }`
    Truncate,
    /// `dst(x,y) = if src(x,y) > threshold { src(x,y) } else { 0 }`
    ToZero,
    /// `dst(x,y) = if src(x,y) > threshold { 0 } else { src(x,y) }`
    ToZeroInverted,
}

impl From<LddThresholdType> for ThresholdType {
    fn from(ldd_threshold_type: LddThresholdType) -> Self {
        match ldd_threshold_type {
            LddThresholdType::Binary => ThresholdType::Binary,
            LddThresholdType::BinaryInverted => ThresholdType::BinaryInverted,
            LddThresholdType::Truncate => ThresholdType::Truncate,
            LddThresholdType::ToZero => ThresholdType::ToZero,
            LddThresholdType::ToZeroInverted => ThresholdType::ToZeroInverted,
        }
    }
}

///image type
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[non_exhaustive]
pub enum LddImageFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
    Pnm,
    Tiff,
    Tga,
    Dds,
    Bmp,
    Ico,
    Hdr,
    OpenExr,
    Farbfeld,
    Avif,
    Qoi,
}

// 
impl LddImageFormat {

    ///获取对应的扩展名
    #[frb(sync)]
    pub fn file_ext(&self) -> String {
        match self {
            LddImageFormat::Png => "png".to_string(),
            LddImageFormat::Jpeg => "jpg".to_string(),
            LddImageFormat::Gif => "gif".to_string(),
            LddImageFormat::WebP => "webp".to_string(),
            LddImageFormat::Pnm => "pgm".to_string(),
            LddImageFormat::Tiff => "tif".to_string(),
            LddImageFormat::Tga => "tga".to_string(),
            LddImageFormat::Dds => "dds".to_string(),
            LddImageFormat::Bmp => "bmp".to_string(),
            LddImageFormat::Ico => "ico".to_string(),
            LddImageFormat::Hdr => "hdr".to_string(),
            LddImageFormat::OpenExr => "exr".to_string(),
            LddImageFormat::Farbfeld => "farbfeld".to_string(),
            LddImageFormat::Avif => "avif".to_string(),
            LddImageFormat::Qoi => "qoi".to_string(),
        }
    }
}

impl From<LddImageFormat> for ImageFormat {
    fn from(ldd_image_format: LddImageFormat) -> Self {
        match ldd_image_format {
            LddImageFormat::Png => ImageFormat::Png,
            LddImageFormat::Jpeg => ImageFormat::Jpeg,
            LddImageFormat::Gif => ImageFormat::Gif,
            LddImageFormat::WebP => ImageFormat::WebP,
            LddImageFormat::Pnm => ImageFormat::Pnm,
            LddImageFormat::Tiff => ImageFormat::Tiff,
            LddImageFormat::Tga => ImageFormat::Tga,
            LddImageFormat::Dds => ImageFormat::Dds,
            LddImageFormat::Bmp => ImageFormat::Bmp,
            LddImageFormat::Ico => ImageFormat::Ico,
            LddImageFormat::Hdr => ImageFormat::Hdr,
            LddImageFormat::OpenExr => ImageFormat::OpenExr,
            LddImageFormat::Farbfeld => ImageFormat::Farbfeld,
            LddImageFormat::Avif => ImageFormat::Avif,
            LddImageFormat::Qoi => ImageFormat::Qoi,
        }
    }
}

impl BitmapImage {
    ///转成打印机可识别的tspl命令数据
    /// [`pos`] 打印的x,y位置
    pub fn download_bmp_image_tspl_command_data(self, pos: (u8, u8)) -> Vec<u8> {
        download_bmp_image_tspl_command_data(self.bitmap, pos.0, pos.1)
    }
}

///图片转单位色图
/// [`image_buffer`] 图片的字节数据
/// [`width`] 转换后目标宽度
/// [`height`] 转换后的目标高度
/// [`threshold_value`] 将图像阈值化，转换为单色图像程度,一般是 128
/// [`threshold_type`] 阈值化转换逻辑类型, 详见枚举[`LddThresholdType`],默认是Binary
/// [`image_format`] 转换后的目标图片类型, 详见枚举[`LddImageFormat`],默认是bmp
/// [`is_monochrome`] 是否要转黑白图像
pub fn ldd_cover_image_to_luma8(
    image_buffer: &[u8],
    width: Option<u32>,
    height: Option<u32>,
    threshold_value: Option<u8>,
    threshold_type: Option<LddThresholdType>,
    image_format: Option<LddImageFormat>,
    is_monochrome: Option<bool>
) -> BitmapImage {
    let image = image::load_from_memory(image_buffer).unwrap();
    let mut gray_image = image.to_luma8();
    
    gray_image = ordered_dither(&gray_image);
    gray_image = image::imageops::resize(
        &gray_image,
        width.unwrap_or(gray_image.width()),
        height.unwrap_or(gray_image.height()),
        image::imageops::FilterType::Lanczos3,
    );
    if let Some(value) = threshold_value {
        let t_type = threshold_type.unwrap_or(LddThresholdType::Binary);
        gray_image = threshold(&gray_image, value, ThresholdType::from(t_type));
    }
    let width = gray_image.width();
    let height = gray_image.height();
    let mut bts = get_image_byte_data(&gray_image, image_format);
    if is_monochrome.map_or(false, |v|v) {
        let da  = to_monochrome(&gray_image);
        bts = da.into_vec();
    }
    return BitmapImage {
        bitmap: bts,
        width: width,
        height: height,
    };
}

// 获取图片的字节数据
fn get_image_byte_data(
    img: &ImageBuffer<Luma<u8>, Vec<u8>>,
    image_format: Option<LddImageFormat>,
) -> Vec<u8> {
    let format = image_format.unwrap_or(LddImageFormat::Bmp);
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, image::ImageFormat::from(format))
        .expect("Failed to write image to buffer");
    buffer.into_inner()
}

// 简单的4x4有序抖动算法
fn ordered_dither(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let threshold_map = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

    let mut dithered_img = img.clone();
    for (x, y, pixel) in img.enumerate_pixels() {
        let threshold = threshold_map[(y as usize % 4)][(x as usize % 4)];
        let new_value = if pixel[0] > threshold * 16 { 255 } else { 0 };
        dithered_img.put_pixel(x, y, Luma([new_value]));
    }

    dithered_img
}






///------v2
///把一个图片转换成打印机可以识别的位图
pub fn ldd_tools_image_to_printer_image(image_buffer: &[u8]) ->BitmapImage{
    //to_luma8转换成灰度图
    let image: ImageBuffer<Luma<u8>, Vec<u8>> = image::load_from_memory(image_buffer).unwrap().to_luma8();
    let gray_image = dither(&image);
    let monochrome_image = to_monochrome(&gray_image);
    let w = monochrome_image.width();
    let h = monochrome_image.height();
    let bts = monochrome_image.into_vec();
    BitmapImage { bitmap: bts, width: w, height:h }
}

// 简单的抖动实现（实际应用中可以使用更复杂的有序抖动算法）
fn dither(img: &GrayImage) -> GrayImage {
    let threshold = 128; // 简单的阈值
    let mut dithered_img = ImageBuffer::new(img.width(), img.height());

    for (x, y, pixel) in img.enumerate_pixels() {
        let Luma(data) = *pixel;
        let new_pixel = if data[0] < threshold { 0 } else { 255 };
        dithered_img.put_pixel(x, y, Luma([new_pixel]));
    }

    dithered_img
}


// 将图像转换为单色（黑白图像）
fn to_monochrome(img: &GrayImage) -> GrayImage {
    let mut monochrome_img = ImageBuffer::new(img.width(), img.height());

    for (x, y, pixel) in img.enumerate_pixels() {
        let Luma(data) = *pixel;
        let new_pixel = if data[0] < 128 { 0 } else { 255 };
        monochrome_img.put_pixel(x, y, Luma([new_pixel]));
    }


    monochrome_img
}

