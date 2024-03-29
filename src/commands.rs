use super::*;

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

	/// get temperature in degrees celcius
	pub fn get_temp(&mut self) -> Result<f32, I2C::Error> {
		let raw_temp = self.read_i16(0x20)?;

		Ok(Self::convert_temp(raw_temp))
	}

	fn convert_temp(raw_temp: i16) -> f32 {
		let temp = raw_temp as f32;

		(temp / 16.0) + 25.0
	}

	/// reads the accelerometer scale from the chip
	pub fn get_accel_scale(&mut self) -> Result<AccelScale, I2C::Error> {
		let bits = self.read_byte(0x10)?;

		Ok(AccelScale::from_bits(bits))
	}

	/// reads the gyroscope scale from the chip
	pub fn get_gyro_scale(&mut self) -> Result<GyroScale, I2C::Error> {
		let bits = self.read_byte(0x11)?;

		Ok(GyroScale::from_bits(bits))
	}

	/// changes the accelerometer scale of the chip, only updates `accel_scale` feild if successful
	pub fn set_accel_scale(&mut self, scale: AccelScale) -> Result<(), I2C::Error> {
		let mut bits = self.read_byte(0x10)?;
		bits &= AccelScale::INVERSE_BIT_MASK; // set bits for scale to 0
		bits |= scale.to_bits(); // set bits for scale to their value

		self.write_byte(0x10, bits)?;

		// make sure we only change the stored value after setting it incase there was an issue with i2c
		self.accel_scale = scale;

		Ok(())
	}

	/// changes the gyroscope scale of the chip, only updates `gyro_scale` feild if successful
	pub fn set_gyro_scale(&mut self, scale: GyroScale) -> Result<(), I2C::Error> {
		let mut bits = self.read_byte(0x11)?;
		bits &= GyroScale::INVERSE_BIT_MASK; // set bits for scale to 0
		bits |= scale.to_bits(); // set bits for scale to their value

		self.write_byte(0x11, bits)?;

		// make sure we only change the stored value after setting it incase there was an issue with i2c
		self.gyro_scale = scale;

		Ok(())
	}

	// TODO: remove
	/// NOTE: this is just temporary
	pub fn raw_x_accel(&mut self)  -> Result<i16, I2C::Error> {
		self.read_i16(0x28)
	}
}
