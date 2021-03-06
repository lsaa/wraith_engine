//
// color.rs - Color
//

use crate::util::hex;

pub struct RgbColor {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl RgbColor {
	pub fn new(r: u8, g: u8, b: u8) -> Self {
		Self { r, g ,b }
	}

	pub fn from_hex(hex: &str) -> Self {
		let data = hex::decode_hex(&hex[1..8]).unwrap();
		Self { r: data[0], g: data[1], b: data[2] }
	}

	pub fn to_hex(&self) -> String {
		format!("#{}",hex::encode_hex(&[self.r, self.g, self.b]))
	}

	pub fn as_float_tuple(&self) -> (f32, f32, f32) {
		(self.r as f32 / 255., self.g as f32 / 255., self.b as f32 / 255.)
	}
}

pub struct HsvColor {
	pub h: u8,
	pub s: u8,
	pub v: u8,
}

impl HsvColor {
	pub fn new(h: u8, s: u8, v: u8) -> Self {
		Self { h, s, v }
	}

	pub fn as_float_tuple(&self) -> (f32, f32, f32) {
		(self.h as f32 / 255., self.s as f32 / 255., self.v as f32 / 255.)
	}

	fn space_conversion(&self, n: u32) -> u8 {
		let (hue, saturation, value) = self.as_float_tuple();
		let k = (n as f32 + ((hue * 360.0)/60.0)) % 6.0;
		((value - (value * saturation * k.min(4.0 - k).min(1.0).max(0.0))) * 255.0) as u8
	}

	pub fn to_rgb(&self) -> RgbColor {
		RgbColor { r: self.space_conversion(5), g: self.space_conversion(3), b: self.space_conversion(1) }
	}
}