use embedded_hal::i2c::I2c;

use glam::i16::I16Vec3;

mod commands;
pub mod accelerometer;
pub mod gyroscope;
pub use glam::DVec3;

use accelerometer::AccelScale;
use gyroscope::GyroScale;

#[derive(Debug)]
pub struct Lsm6dsr<I2C> {
	i2c: I2C,
	accel_scale: AccelScale,
	gyro_scale: GyroScale,
}

impl<I2C: I2c> Lsm6dsr<I2C> {
	const ADDRESS: u8 = 0x6A;
	
	/// NOTE: you must turn on the accelerometer and gyroscope by using `set_accel_sample_rate` and `set_gyro_sample_rate`
	pub fn new(i2c: I2C) -> Self {
		Self {
			i2c,
			accel_scale: AccelScale::Scale2,
			gyro_scale: GyroScale::Scale250,
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

	/// value of `gyro_scale` feild. NOTE: doesn't ask the chip what the value is,
	/// use `get_gyro_scale` for that
	pub fn gyro_scale(&self) -> GyroScale {
		self.gyro_scale
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

	/// read a single `i16` from the given register where
	/// `register` is the low byte of the i16, and `register + 1` is the high byte
	fn read_i16(&mut self, register: u8) -> Result<i16, I2C::Error> {
		let mut buf = [0x00; 2];
		self.read_buf(register, &mut buf)?;

		Ok(i16::from_le_bytes(buf))
	}

	/// read 3 `i16`s from the given register where `register` is the low byte of the
	/// first i16, `register + 1` is the high byte,
	/// `register + 2` is the low byte of the second i16, and so on.
	fn read_trio_i16(&mut self, register: u8) -> Result<I16Vec3, I2C::Error> {
		let mut buf = [0x00; 6];
		self.read_buf(register, &mut buf)?;

		let x = i16::from_le_bytes([buf[0], buf[1]]);
		let y = i16::from_le_bytes([buf[2], buf[3]]);
		let z = i16::from_le_bytes([buf[4], buf[5]]);

		Ok(I16Vec3::new(x, y, z))
	}
}
