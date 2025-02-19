//! Shared bus implementations
use core::fmt::Debug;

use embedded_hal_1::{i2c, spi};

#[cfg(feature = "nightly")]
pub mod asynch;

pub mod blocking;

/// Error returned by I2C device implementations in this crate.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum I2cDeviceError<BUS> {
    /// An operation on the inner I2C bus failed.
    I2c(BUS),
}

impl<BUS> i2c::Error for I2cDeviceError<BUS>
where
    BUS: i2c::Error + Debug,
{
    fn kind(&self) -> i2c::ErrorKind {
        match self {
            Self::I2c(e) => e.kind(),
        }
    }
}

/// Error returned by SPI device implementations in this crate.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SpiDeviceError<BUS, CS> {
    /// An operation on the inner SPI bus failed.
    Spi(BUS),
    /// Setting the value of the Chip Select (CS) pin failed.
    Cs(CS),
}

impl<BUS, CS> spi::Error for SpiDeviceError<BUS, CS>
where
    BUS: spi::Error + Debug,
    CS: Debug,
{
    fn kind(&self) -> spi::ErrorKind {
        match self {
            Self::Spi(e) => e.kind(),
            Self::Cs(_) => spi::ErrorKind::Other,
        }
    }
}
