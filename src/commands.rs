use super::*;

// use i2c::ErrorKind;

impl<I2C: I2c> Lsm6dsr<I2C> {

	// page 50
	const WHO_AM_I_VAL: u8 = 0b01101011;
	// const WHO_AM_I_REGISTER: u8 = 0x0F;
	pub fn is_valid(&mut self) -> Result<bool, I2C::Error> {
		let mut dst = [0x00];
		self.read_register(0x0F, &mut dst)?;

		Ok(dst[0] == Self::WHO_AM_I_VAL)
	}
}