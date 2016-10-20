use std::fmt::Display;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

use super::ModError;

#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Result<Color, ModError> {
        let me = Color { red: red, green: green, blue: blue };
        if red == green || green == blue || red == blue {
            Err(ModError::Ugly(me))
        } else {
            Ok(me)
        }

    }

    pub fn save<P: AsRef<Path>>(&self, file_path: P) -> Result<(), ModError> {
        let mut f = try!(File::create(file_path));
        try!(write!(f, "[R:{}, G:{}, B:{}]\n", self.red, self.green, self.blue));
        Ok(())
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[R:{}, G:{}, B:{}]", self.red, self.green, self.blue)
    }
}