// Copyright 2021 Fabian Bergström
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//!
//! I could probably use https://crates.io/crates/tinytga
//! but wanted to try to have no deps outside of std.
//!

use std::io::Read;

use crate::image::Image;

impl Image {
    /// Parse the given TGA data into a bitmap image.
    pub fn from_tga<R: Read>(data: &mut R) -> Image {
        let header = TgaHeader::from_data(data);
        // only 24 bit RLE is implemented, origins from the left
        assert_eq!(header.bit_depth, 24);
        assert_eq!(header.image_type, 10);
        assert!(header.origin == 0b00 || header.origin == 0b10);

        let pixel_count = header.image_width * header.image_height;
        let flipped = header.origin == 0b00;
        let pixels = read_image_data_rle(data, pixel_count, header.image_width, flipped);
        Image {
            width: header.image_width,
            height: header.image_height,
            flipped,
            pixels,
        }
    }
}

struct TgaHeader {
    image_type: u8,
    image_width: usize,
    image_height: usize,
    bit_depth: u8,
    origin: u8, // but really 4
}

impl TgaHeader {
    /// Parse the TGA header in the given data.
    pub fn from_data<R: Read>(data: &mut R) -> TgaHeader {
        // we'll just read the header fields in order
        let mut id_length = [0];
        let mut colormap_type = [0];
        let mut image_type = [0];
        let mut colormap_spec = [0, 0, 0, 0, 0];
        let mut image_spec = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        data.read_exact(&mut id_length)
            .expect("cannot read id length");
        data.read_exact(&mut colormap_type)
            .expect("cannot read colormap type");
        data.read_exact(&mut image_type)
            .expect("cannot read image type");
        data.read_exact(&mut colormap_spec)
            .expect("cannot read colormap spec");
        data.read_exact(&mut image_spec)
            .expect("cannot read image spec");

        // not all fields are used in this code,
        // but they are named for clarity
        let _id_length = id_length[0];
        let _colormap_type = colormap_type[0];
        let image_type = image_type[0];

        let _colormap_start = to_u16(&colormap_spec[0..2]);
        let colormap_length = to_u16(&colormap_spec[2..4]);
        let _colormap_bits = colormap_spec[4];

        let _origin_x = to_u16(&image_spec[0..2]);
        let _origin_y = to_u16(&image_spec[2..4]);
        let image_width = to_u16(&image_spec[4..6]) as usize;
        let image_height = to_u16(&image_spec[6..8]) as usize;
        let bit_depth = image_spec[8];
        let image_desc = image_spec[9];
        let _alpha_bits = image_desc & 0b0000_1111;
        let origin = (image_desc & 0b0011_0000) >> 4;

        assert_eq!(colormap_length, 0, "color maps are not supported");

        TgaHeader {
            image_type,
            image_width,
            image_height,
            bit_depth,
            origin,
        }
    }
}

/// TGA uses Intel byte ordering, we have to convert to Motorola.
fn to_u16(bytes: &[u8]) -> u16 {
    ((bytes[1] as u16) << 8) | bytes[0] as u16
}

fn read_image_data_rle<R: Read>(
    data: &mut R,
    count: usize,
    width: usize,
    upsidedown: bool,
) -> Vec<[u8; 3]> {
    // TODO texture loading is _really_ slow, profile this
    let mut pixels = Vec::new();
    while pixels.len() < count {
        let mut row = read_pixel_row_rle(data, width);
        if upsidedown {
            row.extend(pixels);
            pixels = row;
        } else {
            pixels.extend(row);
        }
    }
    pixels
}

fn read_pixel_row_rle<R: Read>(source: &mut R, width: usize) -> Vec<[u8; 3]> {
    let mut pixel_row = Vec::new();
    while pixel_row.len() < width {
        // RLE packets should not pass the image width
        read_pixel_packet(source, &mut pixel_row);
    }
    pixel_row
}

fn read_pixel_packet<R: Read>(source: &mut R, sink: &mut Vec<[u8; 3]>) {
    let mut packet_header = [0];
    let mut pixel_value = [0, 0, 0];
    source
        .read_exact(&mut packet_header)
        .expect("unable to read RLE packet header");
    let packet_type = (packet_header[0] & 0b1000_0000) >> 7;
    let packet_size = packet_header[0] & 0b0111_1111;
    if packet_type == 1 {
        // 1 means RLE packet
        source
            .read_exact(&mut pixel_value)
            .expect("unable to read RLE packet");
        pixel_value.reverse(); // "fix" BGR
        for _ in 0..=packet_size {
            sink.push(pixel_value);
        }
    } else {
        // 0 means raw packet
        for _ in 0..=packet_size {
            source
                .read_exact(&mut pixel_value)
                .expect("unable to read raw packet");
            pixel_value.reverse(); // "fix" BGR
            sink.push(pixel_value);
        }
    }
}
