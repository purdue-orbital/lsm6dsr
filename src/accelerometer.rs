
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// Accelerometer scale options (page 51)
pub enum AccelScale {
	/// ± 2 g
	Scale2 = 0,
	/// ± 4 g
	Scale4 = 2,
	/// ± 8 g
	Scale8 = 3,
	/// ± 16 g
	Scale16 = 1,
}

impl AccelScale {
	/// bit mask that covers the bits in the register that set accelerometer scale
	pub const BIT_MASK: u8 = 0b00001100;

	/// bit mask that covers the bits that don't set accelerometer scale
	pub const INVERSE_BIT_MASK: u8 = !Self::BIT_MASK;

	/// convert to the raw bits for the register (handles the bitshifting)
	pub fn to_bits(self) -> u8 {
		(self as u8) << 2
	}

	/// masks and bitshifts the raw bits from the register to `AccelScale`
	pub fn from_bits(bits: u8) -> Self {
		match (bits & Self::BIT_MASK) >> 2 {
			0 => Self::Scale2,
			2 => Self::Scale4,
			3 => Self::Scale8,
			1 => Self::Scale16,
			// it should not be possible to get here, but if somehow it does, it means like a bit flipped, so its safe to assume Scale2
			_ => Self::Scale2,
		}
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum AccelSampleRate {
	/// Accelerometer is powered down
	Off = 0b0000_0000,
	/// 12.5 Hz
	Hz12_5 = 0b0000_1011,
	/// 26 Hz
	Hz26 = 0b0000_0010,
	/// 52 Hz
	Hz52 = 0b0000_0011,
	/// 104 Hz
	Hz104 = 0b0000_0100,
	/// 208 Hz
	Hz208 = 0b0000_0101,
	/// 416 Hz
	Hz416 = 0b0000_0110,
	/// 833 Hz
	Hz833 = 0b0000_0111,
	/// 1.66 kHz
	KHz1_66 = 0b0000_1000,
	/// 3.33 kHz
	KHz3_33 = 0b0000_1001,
	/// 6.66 kHz
	KHz6_66 = 0b0000_1010,
}

impl AccelSampleRate {
	/// bit mask that covers the bits in the register that set accelerometer sample rate
	pub const BIT_MASK: u8 = 0b11110000;

	/// bit mask that covers the bits that don't set accelerometer sample rate
	pub const INVERSE_BIT_MASK: u8 = !Self::BIT_MASK;

	/// convert to the raw bits for the register (handles the bitshifting)
	pub fn to_bits(self) -> u8 {
		(self as u8) << 4
	}

	/// masks and bitshifts the raw bits from the register
	pub fn from_bits(bits: u8) -> Option<Self> {
		match (bits & Self::BIT_MASK) >> 4 {
			0b0000_0000 => Some(Self::Off),
			0b0000_1011 => Some(Self::Hz12_5),
			0b0000_0010 => Some(Self::Hz26),
			0b0000_0011 => Some(Self::Hz52),
			0b0000_0100 => Some(Self::Hz104),
			0b0000_0101 => Some(Self::Hz208),
			0b0000_0110 => Some(Self::Hz416),
			0b0000_0111 => Some(Self::Hz833),
			0b0000_1000 => Some(Self::KHz1_66),
			0b0000_1001 => Some(Self::KHz3_33),
			0b0000_1010 => Some(Self::KHz6_66),
			_ => None
		}
	}
}