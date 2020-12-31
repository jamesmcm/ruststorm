use super::errors::ShapesFileError;
use std::io::prelude::*;
use std::io::SeekFrom;

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
        let mut offset_from_unit: [u8; 4] = [0; 4];
        file.read_exact(&mut offset_from_unit)?;
        let offset_from_unit = u32::from_le_bytes(offset_from_unit);
        *offset += 4;

        let mut color_table: [u8; 4] = [0; 4];
        file.read_exact(&mut color_table)?;
        *offset += 4;
        let color_table = u32::from_le_bytes(color_table);

        if color_table != 0 {
            return Err(ShapesFileError::BadColorTable(color_table));
        }

        // Seek to FrameData and read
        file.seek(SeekFrom::Start(
            (start_unit_offset + offset_from_unit) as u64,
        ))?;
        let framedata = FrameData::from_file_offset(file: &mut std::fs::File, offset: &mut u32)?;

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
    start_frame_offset: u32,
    end_frame_offset: u32,
    data: Vec<Vec<u8>>,
}

impl FrameData {
    pub fn from_file_offset(file: &mut std::fs::File, offset: &mut u32) -> Self {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
