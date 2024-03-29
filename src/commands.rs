use embedded_hal::i2c::I2c;

use glam::DVec3;

use crate::Lsm6dsr;

impl<I2C: I2c> Lsm6dsr<I2C> {

	// page 50
	const CORRECT_WHO_AM_I: u8 = 0b01101011;
	/// checks to see if the chip reports the correct `WHO_AM_I` value
	/// 
	/// returns `Ok(true)` if the chip reports the correct value, `Ok(false)` if it
	/// reports an incorrect value, and `Err()` if there was an error checking the value
	pub fn is_valid(&mut self) -> Result<bool, I2C::Error> {
		let id = self.read_byte(0x0f)?;

		Ok(id == Self::CORRECT_WHO_AM_I)
	}

	/// disable i3c (*not* i2c)
	/// 
	/// It is recomended to do this before doing any other configuration stuff
	pub fn disable_i3c(&mut self) -> Result<(), I2C::Error> {

		// get the other settings so we don't accidently change them
		let mut settings = self.read_byte(0x18)?;

		settings |= 0b00000010; // set the i3c disable bit to 1

		self.write_byte(0x18, settings)
	}

	/// get temperature in degrees Celsius
	pub fn get_temp(&mut self) -> Result<f32, I2C::Error> {
		let raw_temp = self.read_i16(0x20)?;

		Ok(Self::convert_temp(raw_temp))
	}

	// TODO: verify
	fn convert_temp(raw_temp: i16) -> f32 {
		let temp = raw_temp as f32;

		(temp / 16.0) + 25.0
	}

	/// get acceleration in the x axis in Gs
	pub fn x_accel(&mut self)  -> Result<f64, I2C::Error> {
		let raw_val = self.read_i16(0x28)?;

		Ok(self.accel_scale.convert(raw_val))
	}

	/// get acceleration in the y axis in Gs
	pub fn y_accel(&mut self)  -> Result<f64, I2C::Error> {
		let raw_val = self.read_i16(0x2A)?;

		Ok(self.accel_scale.convert(raw_val))
	}

	/// get acceleration in the z axis in Gs
	pub fn z_accel(&mut self)  -> Result<f64, I2C::Error> {
		let raw_val = self.read_i16(0x2C)?;

		Ok(self.accel_scale.convert(raw_val))
	}

	/// get acceleration in all 3 axis. this is more efficient than
	/// getting each individually
	pub fn acceleration(&mut self) -> Result<DVec3, I2C::Error> {
		let raw_accel = self.read_trio_i16(0x28)?;
		let mut accel = raw_accel.as_dvec3();
		let coeff = self.accel_scale.coefficient();

		accel *= coeff / 1000.0;

		Ok(accel)
	}
}
