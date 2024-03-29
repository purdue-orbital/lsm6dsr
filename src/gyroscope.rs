#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// Gyroscope scale options (page 52)
pub enum GyroScale {
	/// ± 125 degrees per second
	Scale125 = 0b0000_0010,
	/// ± 250 degrees per second
	Scale250 = 0b0000_0000,
	/// ± 500 degrees per second
	Scale500 = 0b0000_0100,
	/// ± 1,000 degrees per second
	Scale1k = 0b0000_1000,
	/// ± 2,000 degrees per second
	Scale2k = 0b0000_1100,
	/// ± 4,000 degrees per second
	Scale4k = 0b0000_0001,
}

impl GyroScale {
	/// bit mask that covers the bits in the register that set gyroscope scale
	pub const BIT_MASK: u8 = 0b00001111;

	/// bit mask that covers the bits that don't set gyroscope scale
	pub const INVERSE_BIT_MASK: u8 = !Self::BIT_MASK;

	/// convert to the raw bits for the register (handles the bitshifting)
	pub fn to_bits(self) -> u8 {
		self as u8
	}

	/// masks and bitshifts the raw bits from the register to `GyroScale`
	pub fn from_bits(bits: u8) -> Self {
		// TODO!! figure out wether 4k or 125 has priority
		if (bits & 0b00000001) != 0 {
			Self::Scale4k
		} else if (bits & 0b00000010) != 0 {
			Self::Scale125
		} else {
			match (bits & Self::BIT_MASK) >> 2 {
				0 => Self::Scale250,
				1 => Self::Scale500,
				2 => Self::Scale1k,
				3 => Self::Scale2k,
				// it should not be possible to get here, but if somehow it does, it means like a bit flipped, so its safe to assume Scale250
				_ => Self::Scale250,
			}
		}
	}

	/// get miliG/LSB
	pub fn coefficient(self) -> f64 {
		// from page 9
		match self {
			GyroScale::Scale125 => 4.375,
			GyroScale::Scale250 => 8.75,
			GyroScale::Scale500 => 17.5,
			GyroScale::Scale1k => 35.0,
			GyroScale::Scale2k => 70.0,
			GyroScale::Scale4k => 140.0,
		}
	}

	/// converts a raw `i16` measured with the given scale to degrees per second
	#[inline]
	pub fn convert(self, raw: i16) -> f64 {
		let val = raw as f64;

		val * self.coefficient() / 1000.0
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GyroSampleRate {
	/// Gyroscope is powered down
	Off = 0,
	/// 12.5 Hz
	Hz12_5 = 1,
	/// 26 Hz
	Hz26 = 2,
	/// 52 Hz
	Hz52 = 3,
	/// 104 Hz
	Hz104 = 4,
	/// 208 Hz
	Hz208 = 5,
	/// 416 Hz
	Hz416 = 6,
	/// 833 Hz
	Hz833 = 7,
	/// 1.66 kHz
	KHz1_66 = 8,
	/// 3.33 kHz
	KHz3_33 = 9,
	/// 6.66 kHz
	KHz6_66 = 10,
}

impl GyroSampleRate {
	/// bit mask that covers the bits in the register that set gyroscope sample rate
	pub const BIT_MASK: u8 = 0b11110000;

	/// bit mask that covers the bits that don't set gyroscope sample rate
	pub const INVERSE_BIT_MASK: u8 = !Self::BIT_MASK;

	/// convert to the raw bits for the register (handles the bitshifting)
	pub fn to_bits(self) -> u8 {
		(self as u8) << 4
	}

	/// masks and bitshifts the raw bits from the register
	pub fn from_bits(bits: u8) -> Option<Self> {
		match (bits & Self::BIT_MASK) >> 4 {
			0 => Some(Self::Off),
			1 => Some(Self::Hz12_5),
			2 => Some(Self::Hz26),
			3 => Some(Self::Hz52),
			4 => Some(Self::Hz104),
			5 => Some(Self::Hz208),
			6 => Some(Self::Hz416),
			7 => Some(Self::Hz833),
			8 => Some(Self::KHz1_66),
			9 => Some(Self::KHz3_33),
			10 => Some(Self::KHz6_66),
			_ => None
		}
	}
}

use embedded_hal::i2c::I2c;
use crate::Lsm6dsr;

impl <I2C: I2c> Lsm6dsr<I2C> {
	/// reads the gyroscope scale from the chip
	pub fn get_gyro_scale(&mut self) -> Result<GyroScale, I2C::Error> {
		let bits = self.read_byte(0x11)?;

		Ok(GyroScale::from_bits(bits))
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

	/// turn on/off the gyro and set the sample rate
	pub fn set_gyro_sample_rate(&mut self, sample_rate: GyroSampleRate) -> Result<(), I2C::Error> {
		let mut bits = self.read_byte(0x11)?;
		bits &= GyroSampleRate::INVERSE_BIT_MASK;
		bits |= sample_rate.to_bits();

		self.write_byte(0x11, bits)
	}
}