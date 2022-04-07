use std::ops::{Deref, DerefMut};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NoDataAvailableError;

impl Error for NoDataAvailableError {}
impl fmt::Display for NoDataAvailableError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\t [ NoDataAvailableError ] \n\t WARNING: there is no data \n\t stored in readings vector!")
    }
}

#[derive(Debug)]
pub struct Sensor<T> {
    pub readings: Vec<T>,
}

impl<T> Sensor<T> {
    pub fn new(readings: Vec<T>) -> Self {
        Sensor {
            readings,
        }
    }

    pub fn has_data(&self) -> Result<bool, NoDataAvailableError> {
        let x = &self.readings;
        match x {
            x if x.is_empty() => Err(NoDataAvailableError),
                            _ => Ok(true),
        }
    }
}

impl<T> Deref for Sensor<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.readings
    }
}

impl<T> DerefMut for Sensor<T> {
    
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.readings
    }
}