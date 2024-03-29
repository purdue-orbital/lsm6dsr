#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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