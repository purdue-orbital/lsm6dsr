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
		let mut buf = [0x00; 2];

		self.read_buf(0x20, &mut buf)?;

		// convert using the correct byte order
		let raw_temp = i16::from_le_bytes(buf) as f32;
		let temperature = (raw_temp / 16.0) + 25.0;

		Ok(temperature)
	}
}
