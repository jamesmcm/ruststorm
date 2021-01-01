use super::errors::ShapesFileError;
use std::io::prelude::*;
use std::io::SeekFrom;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, PartialEq)]
pub struct Frame {
    offset_from_unit: u32,
    color_table: u32, // Always zero
    data: FrameData,
}

impl Frame {
    pub fn from_file_offset(
        file: &mut std::fs::File,
        offset: &mut u32,
        start_unit_offset: u32,
    ) -> Result<Self, ShapesFileError> {
        let offset_from_unit = file.read_u32::<LittleEndian>()?;
        *offset += 4;

        let color_table = file.read_u32::<LittleEndian>()?;
        *offset += 4;
        if color_table != 0 {
            return Err(ShapesFileError::BadColorTable(color_table));
        }

        // Seek to FrameData and read
        file.seek(SeekFrom::Start(
            (start_unit_offset + offset_from_unit) as u64,
        ))?;
        let framedata = FrameData::from_file_offset(file)?;

        file.seek(SeekFrom::Start(*offset as u64))?;
        Ok(Self {
            offset_from_unit,
            color_table,
            data: framedata,
        })
    }
}

#[derive(Debug, PartialEq)]
struct FrameData {
    height: u16,
    width: u16,
    origin_x: u16,
    origin_y: u16,
    minimum_x: i32,
    minimum_y: i32,
    maximum_x: i32,
    maximum_y: i32,
    // start_frame_offset: u32,
    // end_frame_offset: u32,
    data: Vec<Vec<u8>>,
}

impl FrameData {
    pub fn from_file_offset(file: &mut std::fs::File) -> Result<Self, ShapesFileError> {
        let height = file.read_u16::<LittleEndian>()?;
        let width = file.read_u16::<LittleEndian>()?;

        let origin_x = file.read_u16::<LittleEndian>()?;
        let origin_y = file.read_u16::<LittleEndian>()?;

        let minimum_x = file.read_i32::<LittleEndian>()?;
        let minimum_y = file.read_i32::<LittleEndian>()?;
        let maximum_x = file.read_i32::<LittleEndian>()?;
        let maximum_y = file.read_i32::<LittleEndian>()?;

        let mut row: u16 = 0;
        let mut pixel_data: Vec<Vec<u8>> = vec![vec![0; width as usize]; height as usize];
        while row < height {
            let packet_type = file.read_u8()?;

            match packet_type {
                0 => { // Fill remaining part of row with 0xFF
                }
                1 => { // Read u8 and write that many 0XFF to current row, do not advance row
                }
                x if x & 1 != 0 => { // Write next x >> 1 bytes as-is - do not advance row
                }
                x => { // Runlength encoding, read next u8 and write it x times
                }
            }

            break;
        }
        Ok(Self {
            height,
            width,
            origin_x,
            origin_y,
            minimum_x,
            minimum_y,
            maximum_x,
            maximum_y,
            data: pixel_data,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
