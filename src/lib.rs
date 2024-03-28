use embedded_hal::i2c::{self, I2c};

mod commands;

pub use commands::*;

#[derive(Debug)]
pub struct Lsm6dsr<I2C> {
	i2c: I2C,
	g_scale: GScale,
	filtering: bool // page 51
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// Accelerometer scale options (page 51)
pub enum GScale {
	Scale2 = 0,
	Scale4 = 2,
	Scale16 = 1,
	Scale8 = 3,
}

impl<I2C: I2c> Lsm6dsr<I2C> {
	const ADDRESS: u8 = 0x6A;

	
	pub fn new(i2c: I2C) -> Self {
		Self {
			i2c,
			g_scale: GScale::Scale2,
			filtering: false,
		}
	}

	#[inline]
	fn read(&mut self, read: &mut [u8]) -> Result<(), I2C::Error> {
		self.i2c.read(Self::ADDRESS, read)
	}

	#[inline]
	fn write(&mut self, write: &[u8]) -> Result<(), I2C::Error> {
		self.i2c.write(Self::ADDRESS, write)
	}

	#[inline]
	fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), I2C::Error> {
		self.i2c.write_read(Self::ADDRESS, write, read)
	}

	#[inline]
	fn read_register(&mut self, register: u8, read: &mut [u8]) -> Result<(), I2C::Error> {
		self.i2c.write_read(Self::ADDRESS, &[register], read)
	}

}