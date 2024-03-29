#[repr(u8)]
#[derive(Debug, Clone, Copy)]
// TODO: add 125 and 4000
/// Gyroscope scale options (page 52)
pub enum GyroScale {
	/// ± 250 degrees per second
	Scale250 = 0,
	/// ± 500 degrees per second
	Scale500 = 1,
	/// ± 1,000 degrees per second
	Scale1k = 2,
	/// ± 2,000 degrees per second
	Scale2k = 3,
}

impl GyroScale {
	/// bit mask that covers the bits in the register that set gyroscope scale
	pub const BIT_MASK: u8 = 0b00001100;

	/// bit mask that covers the bits that don't set gyroscope scale
	pub const INVERSE_BIT_MASK: u8 = !Self::BIT_MASK;

	/// convert to the raw bits for the register (handles the bitshifting)
	pub fn to_bits(self) -> u8 {
		(self as u8) << 2
	}

	/// masks and bitshifts the raw bits from the register to `GyroScale`
	pub fn from_bits(bits: u8) -> Self {
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