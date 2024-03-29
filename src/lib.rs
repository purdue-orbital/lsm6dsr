use embedded_hal::i2c::{self, I2c};

mod commands;
pub mod accelerometer;

use accelerometer::AccelScale;

// pub use commands::*;

#[derive(Debug)]
pub struct Lsm6dsr<I2C> {
	i2c: I2C,
	accel_scale: AccelScale,
	accel_filtering: bool // page 51
}

impl<I2C: I2c> Lsm6dsr<I2C> {
	const ADDRESS: u8 = 0x6A;
	
	pub fn new(i2c: I2C) -> Self {
		Self {
			i2c,
			accel_scale: AccelScale::Scale2,
			accel_filtering: false,
		}
	}

	// #[inline]
	// /// NOTE: you must handle registers yourself
	// fn read(&mut self, read: &mut [u8]) -> Result<(), I2C::Error> {
	// 	self.i2c.read(Self::ADDRESS, read)
	// }

	// #[inline]
	// /// NOTE: you must handle registers yourself
	// fn write(&mut self, write: &[u8]) -> Result<(), I2C::Error> {
	// 	self.i2c.write(Self::ADDRESS, write)
	// }

	// #[inline]
	// /// NOTE: you must handle registers yourself
	// fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), I2C::Error> {
	// 	self.i2c.write_read(Self::ADDRESS, write, read)
	// }

	/// value of `accel_scale` feild. NOTE: doesn't ask the chip what the value is,
	/// use `get_accel_scale` for that
	pub fn accel_scale(&self) -> AccelScale {
		self.accel_scale
	}

	#[inline]
	fn read_buf(&mut self, register: u8, read: &mut [u8]) -> Result<(), I2C::Error> {
		self.i2c.write_read(Self::ADDRESS, &[register], read)
	}

	fn write_buf(&mut self, register: u8, write: &[u8]) -> Result<(), I2C::Error> {
		// create a buf with the register as the first value
		let mut buf = write.to_vec();
		buf.insert(0, register);
		
		self.i2c.write(Self::ADDRESS, &buf)?;
		
		Ok(())
	}
	
	#[inline]
	fn write_byte(&mut self, register: u8, value: u8) -> Result<(), I2C::Error> {
		self.i2c.write(Self::ADDRESS, &[register, value])
	}
	
	#[inline]
	fn read_byte(&mut self, register: u8) -> Result<u8, I2C::Error> {
		let mut buf = [0x00];
		self.read_buf(register, &mut buf)?;

		Ok(buf[0])
	}

	/// Where `register` is the low byte of the i16, and `register + 1` is the high byte
	fn read_i16(&mut self, register: u8) -> Result<i16, I2C::Error> {
		let mut buf = [0x00; 2];
		self.read_buf(register, &mut buf)?;

		Ok(i16::from_le_bytes(buf))
	}
}