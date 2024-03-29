
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// Accelerometer scale options (page 51)
pub enum AccelScale {
	Scale2 = 0,
	Scale4 = 2,
	Scale8 = 3,
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