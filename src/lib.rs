use std::io;
use std::error::Error;

pub mod shapes;
pub mod colors;

pub use shapes::Shape;
pub use shapes::triangle::Triangle;
pub use shapes::circle::Circle;
pub use colors::Color;

#[derive(Debug)]
pub enum ModError {
    Io(io::Error),
    Ugly(colors::Color),
    InvalidBase(f64),
    InvalidHeight(f64),
    InvalidRadius(f64)
}

use std::fmt;

impl fmt::Display for ModError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModError::Io(ref err) => err.fmt(f),
            ModError::Ugly(ref color) => write!(f, "{} is really ugly!", color),
            ModError::InvalidBase(num) => write!(f, "{} is invalid for a triangle base", num),
            ModError::InvalidHeight(num) => write!(f, "{} is invalid for a triangle height", num),
            ModError::InvalidRadius(num) => write!(f, "{} is invalid for a radius", num)
        }
    }
}

impl Error for ModError {
    fn description(&self) -> &str {
        match *self {
            ModError::Io(ref err) => err.description(),
            ModError::Ugly(_) => "An ugly color",
            ModError::InvalidBase(_) => "Invalid Triangle Base",
            ModError::InvalidHeight(_) => "Invalid Triangle Height",
            ModError::InvalidRadius(_) => "Invalid Radius"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ModError::Io(ref err) => Some(err),
            ModError::Ugly(_) => None,
            ModError::InvalidBase(_) => None,
            ModError::InvalidHeight(_) => None,
            ModError::InvalidRadius(_) => None
        }
    }
}

impl From<io::Error> for ModError {
    fn from(err: io::Error) -> ModError {
        ModError::Io(err)
    }
}