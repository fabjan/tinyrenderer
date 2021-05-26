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
//! Netpbm support,
//! because also implementing _creating_ TGA files seemed like a bother.
//!

use std::io::BufWriter;
use std::io::Write;

use crate::image::Image;

impl Image {
    /// Save the bitmap data as a Netpbm image (P6, a binary PixMap).
    pub fn write<W: Write>(&self, writer: &mut BufWriter<W>) {
        let header = format!("P6 {} {} {}\n", self.width, self.height, 255);
        writer
            .write_all(header.as_bytes())
            .expect("unable to write image header");

        // TODO I don't think this handles flipped images properly.
        for p in &self.pixels {
            writer.write_all(p).expect("unable to write pixel data");
        }
    }
}
