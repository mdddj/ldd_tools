use flutter_rust_bridge::frb;

use super::{image::BitmapImage, printer::download_bmp_image_tspl_command_data};
#[frb(opaque)]
pub struct TsplCommandBuild {
    data: Vec<u8>,
}

impl TsplCommandBuild {
    ///定义尺寸, 毫米类型
    #[frb(sync)]
    pub fn size(&mut self, size: (u8, u8)) {
        self.data
            .extend(format!("SIZE {} mm, {} mm\n", size.0, size.1).as_bytes());
    }

    ///设置浓度
    #[frb(sync)]
    pub fn density(&mut self, n: u8) {
        self.data.extend(format!("DENSITY {n}\n").as_bytes());
    }

    ///设置打印速度
    #[frb(sync)]
    pub fn speed(&mut self, n: u8) {
        self.data.extend(format!("SPEED {n}\n").as_bytes());
    }

    ///移动标签的横向和纵向位置。正数使标签往打印方向的相反方向移动，负数使标签往打印方向移动。
    #[frb(sync)]
    pub fn shift(&mut self, y: u8) {
        self.data.extend(format!("SHIFT {y}\n").as_bytes());
    }

    ///定义每次标签定位时可选择的、额外的标签进纸高度。主要在撕纸或切纸模式下，用于调整标签停止位置，并在下次打印前回退响应的距离。
    /// 单位: 毫米
    #[frb(sync)]
    pub fn offset(&mut self, m: u8) {
        self.data.extend(format!("OFFSET {m} mm\n").as_bytes());
    }

    ///定义两张标签之间的缝宽。单位毫米mm
    ///[`m`] 两个标签之间的距离
    ///[`n`] 缝的偏移
    ///0,0 连续纸
    #[frb(sync)]
    pub fn gap(&mut self, m: u8, n: u8) {
        self.data.extend(format!("GAP {m} mm,{n} mm\n").as_bytes());
    }

    /// 设置代码页及国际字符集,推荐UTF-8
    #[frb(sync)]
    pub fn code_page(&mut self, n: String) {
        self.data.extend(format!("CODEPAGE {n}\n").as_bytes());
    }

    ///清除画布缓存
    /// 注意：该命令必须在SIZE命令之后
    #[frb(sync)]
    pub fn cls(&mut self) {
        self.data.extend(b"CLS\n");
    }

    ///自定义命令
    #[frb(sync)]
    pub fn command(&mut self, command: String) {
        self.data.extend(format!("{command}\n").as_bytes());
    }

    ///添加数据
    #[frb(sync)]
    pub fn append_data(&mut self, data: Vec<u8>) {
        self.data.extend(data);
    }

    ///添加bmp图片
    /// [`image`] bmp图片资源
    /// [`pos`] x,y 位置
    pub fn append_bmp_image(&mut self, image: BitmapImage, pos: (u8, u8)) {
        let data = download_bmp_image_tspl_command_data(image.bitmap, pos.0, pos.1);
        self.data.extend(data);
    }

    ///打印已缓存的标签。一般是(1,1)
    /// [`count`] 打印标签的数量,1: 打印标签的组数 2: 每组标签打印的数量
    #[frb(sync)]
    pub fn printer(&mut self, count: (u8, u8)) {
        self.data
            .extend(format!("PRINT {},{}\n", count.0, count.1).as_bytes());
    }

    /// 初始化
    #[frb(sync)]
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    ///获取字节数据
    #[frb(sync)]
    pub fn build(self) -> Vec<u8> {
        self.data
    }
}
