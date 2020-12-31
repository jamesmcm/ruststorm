use super::errors::ShapesFileError;
use super::frame::Frame;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub struct UnitType {
    version: u32, //0x30312E31
    frame_count: u32,
    frames: Vec<Frame>,
}

impl UnitType {
    pub fn new(version: u32, frame_count: u32, frames: Vec<Frame>) -> Self {
        UnitType {
            version,
            frame_count,
            frames,
        }
    }

    /// Parse UnitType from shapes file
    /// Assumes &mut File is passed with cursor at the start of the unit type header
    pub fn from_file_offset(
        file: &mut std::fs::File,
        offset: &mut u32,
    ) -> Result<Self, ShapesFileError> {
        let start_unit_offset = offset.clone();
        let mut vers: [u8; 4] = [0; 4];
        file.read_exact(&mut vers)?;
        *offset += 4;

        // Data is stored little-endian
        let vers = u32::from_le_bytes(vers);

        // Enforce version 1.10
        if vers != 0x30312E31 {
            return Err(ShapesFileError::BadVersion(vers));
        }

        let mut frame_count: [u8; 4] = [0; 4];
        file.read_exact(&mut frame_count)?;
        *offset += 4;
        let frame_count = u32::from_le_bytes(frame_count);

        let mut frames: Vec<Frame> = vec![];
        for i in 0..frame_count {
            let frame = Frame::from_file_offset(file, offset)?;
            frames.push(frame);
        }
        Ok(Self {
            version: vers,
            frame_count,
            frames,
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
