use std::io::Read;

use crate::image::Image;

impl Image {
    pub fn from_tga<R: Read>(data: &mut R) -> Image {
        let header = TgaHeader::from_data(data);
        // only 24 bit RLE is implemented
        assert_eq!(header.bit_depth, 24);
        assert_eq!(header.image_type, 10);
        let pixel_count = header.image_width * header.image_height;
        let pixels = read_image_data_rle(data, pixel_count);
        Image {
            width: header.image_width,
            height: header.image_height,
            flipped: false,
            pixels: pixels,
        }
    }
}

struct TgaHeader {
    pub image_type: u8,
    pub image_width: usize,
    pub image_height: usize,
    pub bit_depth: u8,
    pub origin: u8, // but really 4
}

impl TgaHeader {
    pub fn from_data<R: Read>(data: &mut R) -> TgaHeader {
        let mut id_length     = [0];
        let mut colormap_type = [0];
        let mut image_type    = [0];
        let mut colormap_spec = [0, 0, 0, 0, 0];
        let mut image_spec    = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        data.read_exact(&mut id_length).expect("cannot read id length");
        data.read_exact(&mut colormap_type).expect("cannot read colormap type");
        data.read_exact(&mut image_type).expect("cannot read image type");
        data.read_exact(&mut colormap_spec).expect("cannot read colormap spec");
        data.read_exact(&mut image_spec).expect("cannot read image spec");

        let id_length      = id_length[0];
        let _colormap_type = colormap_type[0];
        let image_type     = image_type[0];

        let _colormap_start = to_u16(&colormap_spec[0..2]);
        let colormap_length = to_u16(&colormap_spec[2..4]);
        let _colormap_bits  = colormap_spec[4];

        let _origin_x    = to_u16(&image_spec[0..2]);
        let _origin_y    = to_u16(&image_spec[2..4]);
        let image_width  = to_u16(&image_spec[4..6]);
        let image_height = to_u16(&image_spec[6..8]);
        let image_bits   = image_spec[8];
        let image_desc   = image_spec[9];
        let _alpha_bits  = image_desc & 0b00001111u8;
        let image_origin = (image_desc & 0b00110000u8) >> 4;

        // these optional fields are not supported
        assert_eq!(id_length, 0);
        assert_eq!(colormap_length, 0);

        TgaHeader {
            image_type: image_type,
            image_width: image_width as usize,
            image_height: image_height as usize,
            bit_depth: image_bits,
            origin: image_origin,
        }
    }
}

fn to_u16(bytes: &[u8]) -> u16 {
    // TGA uses Intel byte ordering, convert to Motorola.
    ((bytes[1] as u16) << 8) | bytes[0] as u16
}

fn read_image_data_rle<R: Read>(data: &mut R, count: usize) -> Vec<[u8;3]> {
    let mut pixels = Vec::new();
    while pixels.len() < count {
        read_pixel_packet(data, &mut pixels);
    }
    pixels
}

fn read_pixel_packet<R: Read>(source: &mut R, sink: &mut Vec<[u8;3]>) {
    let mut packet_header = [0];
    let mut pixel_value   = [0, 0, 0];
    source.read_exact(&mut packet_header).expect("unable to read RLE packet header");
    let packet_type = (packet_header[0] & 0b10000000) >> 7;
    let packet_size = packet_header[0] & 0b01111111;
    if packet_type == 1 { // 1 means RLE packet
        source.read_exact(&mut pixel_value).expect("unable to read RLE packet");
        pixel_value.reverse(); // "fix" BGR
        for _i in 0..(packet_size+1) {
            sink.push(pixel_value);
        }
    } else { // 0 means raw packet
        for _i in 0..(packet_size+1) {
            source.read_exact(&mut pixel_value).expect("unable to read raw packet");
            pixel_value.reverse(); // "fix" BGR
            sink.push(pixel_value);
        }
    }
}
