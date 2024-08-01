///在打印数据中添加单色位图
pub fn download_bmp_image_tspl_command_data(image_buffer: Vec<u8>, x: u8, y: u8) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];

    // Add the DOWNLOAD command
    data.extend(
        b"
        DOWNLOAD \"temp.bmp\"\n
    ",
    );

    // Add the image buffer
    data.extend(image_buffer);

    // Add the EOP and PUTBMP commands with x and y parameters
    data.extend(format!("\nEOP\nPUTBMP {},{},\"temp.bmp\"\n", x, y).as_bytes());

    data
}

//tspl打印命令
pub fn tspl_print_command_data() -> Vec<u8> {
    let mut data: Vec<u8> = vec![];
    data.extend(b"PRINT 1,1\n");
    return data;
}
