mod errors;
mod frame;
mod unit;

use errors::ShapesFileError;
use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::path::Path;
use unit::UnitType;

#[derive(Debug, PartialEq)]
struct Shapes {
    units: Vec<UnitType>,
}

impl Shapes {
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ShapesFileError> {
        let mut units: Vec<UnitType> = Vec::new();
        let mut file = std::fs::File::open(path)?;
        let mut offset = 0;
        loop {
            let unit = UnitType::from_file_offset(&mut file, &mut offset);
            match unit {
                Err(ShapesFileError::BadVersion(v)) => {
                    break;
                }
                Err(ShapesFileError::EOF) => {
                    break;
                }
                Err(e) => return Err(e),
                Ok(u) => {
                    units.push(u);
                }
            }
        }
        Ok(Shapes { units })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), ShapesFileError> {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/empty_shapes.shp");
        let shapes = Shapes::from_file(path)?;
        assert_eq!(shapes.units, vec![UnitType::new(0x30312E31, 0, Vec::new())]);
        Ok(())
    }
}
