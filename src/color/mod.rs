use std::ops::*;

#[derive(Clone)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

pub static BLACK: Color 	= Color {r: 0.	, g: 0.	 , b: 0. };
pub static WHITE: Color 	= Color {r: 255., g: 255., b: 255. };
pub static GREY: Color 		= Color {r: 128., g: 128., b: 128. };
pub static SILVER: Color 	= Color {r: 192., g: 192., b: 192. };
pub static RED: Color 		= Color {r: 255., g: 0.	 , b: 0. };
pub static MAROON: Color 	= Color {r: 128., g: 0.	 , b: 0. };
pub static OLIVE: Color 	= Color {r: 128., g: 128., b: 0. };
pub static MAGENTA: Color 	= Color {r: 255., g: 0.	 , b: 255. };
pub static PURPLE: Color 	= Color {r: 128., g: 0.	 , b: 128. };
pub static LIME: Color 		= Color {r: 0.	, g: 255., b: 0. };
pub static GREEN: Color		= Color {r: 0.	, g: 128., b: 0. };
pub static BLUE: Color 		= Color {r: 0.	, g: 0.	 , b: 255. };
pub static NAVY: Color 		= Color {r: 0.	, g: 0.	 , b: 128. };
pub static CYAN: Color 		= Color {r: 0.	, g: 255., b: 255. };
pub static TEAL: Color 		= Color {r: 0.	, g: 128., b: 128. };
pub static YELLOW: Color 	= Color {r: 255., g: 255., b: 0. };

impl Color {
	pub fn new(r: f32, g: f32, b: f32) -> Color {
		Color { r, g, b }
	}
	
	pub fn copy_to(&self, col: &mut Color) {
		col.r = self.r;
		col.g = self.g;
		col.b = self.b;
	}
	
	pub fn set_from_color(&mut self, col: &Color) {
		self.r = col.r;
		self.g = col.g;
		self.b = col.b;
	}
	
	pub fn set_from_rgb(&mut self, r: f32, g: f32, b: f32) {
		self.r = r;
		self.g = g;
		self.b = b;
	}
	
	pub fn set_all(&mut self, value: f32) {
		self.r = value;
		self.g = value;
		self.b = value;
	}
	
	pub fn reset(&mut self) {
		self.r = 0.;
		self.g = 0.;
		self.b = 0.;
	}
	
	pub fn invert(&mut self) {
		self.r = 255. - self.r;
		self.g = 255. - self.g;
		self.b = 255. - self.b;
	}
	
	fn truncate_channel(channel: &mut f32) {
		if *channel > 255. { *channel = 255.; }
		else if *channel < 0. { *channel = 0.; }
	}
	
	fn truncate(&mut self) {
		Color::truncate_channel(&mut self.r);
		Color::truncate_channel(&mut self.g);
		Color::truncate_channel(&mut self.b);
	}
	
	pub fn set_brightness(&mut self, brightness: f32) {
		*self += brightness;
		self.truncate();
	}
	
	pub fn set_brightness_to(&self, brightness: f32, target: &mut Color) {
		target.set_from_color(self);
		*target += brightness;
		target.truncate();
	}
	
	fn contrast_factor(contrast: &f32) -> f32 {
		let mut usedcontrast = *contrast;
		if usedcontrast > 255. { usedcontrast = 255.; }
		else if usedcontrast < -255. { usedcontrast = -255.; }
		(259.*(usedcontrast + 255.)) / (255.*(259. - usedcontrast))
	}
	
	pub fn set_contrast(&mut self, contrast: f32) {
		let contrast_factor = Color::contrast_factor(&contrast);
		self.r = (contrast_factor * (self.r - 128.)) + 128.;
		self.g = (contrast_factor * (self.g - 128.)) + 128.;
		self.b = (contrast_factor * (self.b - 128.)) + 128.;
		self.truncate();
	}
	
	pub fn set_contrast_to(&mut self, contrast: f32, target: &mut Color) {
		target.set_from_color(self);
		let contrast_factor = Color::contrast_factor(&contrast);
		target.r = (contrast_factor * (target.r - 128.)) + 128.;
		target.g = (contrast_factor * (target.g - 128.)) + 128.;
		target.b = (contrast_factor * (target.b - 128.)) + 128.;
		target.truncate();
	}
	
}

impl Add<f32> for Color {
	type Output = Color;

	fn add(self, rhs: f32) -> Color {
		Color { r: self.r + rhs, g: self.g + rhs, b: self.b + rhs }
    }
}

impl Add<Color> for Color {
	type Output = Color;
	
	fn add(self, rhs: Color) -> Color {
		Color { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

impl<'a> Add<&'a Color> for &'a Color {
	type Output = Color;
	
	fn add(self, rhs: &'a Color) -> Color {
		Color { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

impl AddAssign<f32> for Color {
	fn add_assign(&mut self, rhs: f32){
		self.r += rhs;
		self.g += rhs;
		self.b += rhs;
    }
}

impl AddAssign<Color> for Color {
	
	fn add_assign(&mut self, rhs: Color){
		self.r += rhs.r;
		self.g += rhs.g;
		self.b += rhs.b;
    }
}

impl<'a> AddAssign<&'a Color> for Color {
	
	fn add_assign(&mut self, rhs: &'a Color){
		self.r += rhs.r;
		self.g += rhs.g;
		self.b += rhs.b;
    }
}

impl Sub<f32> for Color {
	type Output = Color;

	fn sub(self, rhs: f32) -> Color {
		Color { r: self.r - rhs, g: self.g - rhs, b: self.b - rhs }
    }
}

impl Sub<Color> for Color {
	type Output = Color;
	
	fn sub(self, rhs: Color) -> Color {
		Color { r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b }
    }
}

impl<'a> Sub<&'a Color> for &'a Color {
	type Output = Color;
	
	fn sub(self, rhs: &'a Color) -> Color {
		Color { r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b }
    }
}

impl SubAssign<f32> for Color {
	fn sub_assign(&mut self, rhs: f32){
		self.r -= rhs;
		self.g -= rhs;
		self.b -= rhs;
    }
}

impl SubAssign<Color> for Color {
	
	fn sub_assign(&mut self, rhs: Color){
		self.r -= rhs.r;
		self.g -= rhs.g;
		self.b -= rhs.b;
    }
}

impl<'a> SubAssign<&'a Color> for Color {
	
	fn sub_assign(&mut self, rhs: &'a Color){
		self.r -= rhs.r;
		self.g -= rhs.g;
		self.b -= rhs.b;
    }
}

impl Mul<f32> for Color {
	type Output = Color;

	fn mul(self, rhs: f32) -> Color {
		Color { r: self.r * rhs, g: self.g * rhs, b: self.b * rhs }
    }
}

impl Mul<Color> for Color {
	type Output = f32;
	
	fn mul(self, rhs: Color) -> f32 {
		(self.r * rhs.r) + (self.g * rhs.g) + (self.b * rhs.b)
    }
}

impl<'a> Mul<&'a Color> for &'a Color {
	type Output = f32;
	
	fn mul(self, rhs: &'a Color) -> f32 {
		(self.r * rhs.r) + (self.g * rhs.g) + (self.b * rhs.b)
    }
}

impl MulAssign<f32> for Color {
	fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
		self.g *= rhs;
		self.b *= rhs;
    }
}

impl<'a> MulAssign<&'a Color> for Color {
	
	fn mul_assign(&mut self, rhs: &'a Color){
		self.r *= rhs.r;
		self.g *= rhs.g;
		self.b *= rhs.b;
    }
}

impl ToString for Color {
	fn to_string(&self) -> String {
		format!("r:{} g:{} b:{}", self.r, self.g, self.b)
	}
}

impl PartialEq for Color {
	fn eq(&self, other: &Color) -> bool {
		(self as *const _ == other as *const _) || (self.r == other.r && self.g == other.g && self.b == other.b)
	}
	fn ne(&self, other: &Color) -> bool {
		(self as *const _ != other as *const _) && (self.r != other.r || self.g != other.g || self.b != other.b)
	}
}
impl Eq for Color {}

#[cfg(test)]
mod color_test;
