pub mod converter;
pub mod filter;

use math::algorithm::fractals::julia::*;
use math::algorithm::fractals::mandelbrot::*;
use math::algorithm::noise::core::*;

use texture::converter::noise::*;
use texture::converter::julia::*;
use texture::converter::mandelbrot::*;

use texture::filter::{TextureFilter};

use std::io::Write;                                                                                                                                                                                                                                                                                                                            
use std::fs::File;    

use color::*;

#[derive(Clone)]
pub struct Texture {
	pub width: u32,
	pub height: u32,
	pub buffer: Vec<Color>,
}

impl Texture {
	
	pub fn new( width: u32, height: u32) -> Texture {
		let mut texture = Texture {
			width: width,
			height: height,
			buffer: Vec::with_capacity((width*height) as usize),
		};
		
		for _i in 0..texture.buffer.capacity() {
			texture.buffer.push(BLACK.clone());
		}
		texture
	}
	
	pub fn from_noise(&mut self, noise: &Noise) {
		noise_to_texture(noise, self);
	}
	
	pub fn from_mandelbrot(&mut self, mandelbrot: &Mandelbrot, mb_color_func: fn(&Mandelbrot, &MandelbrotPoint, &mut Color)) {
		mandelbrot_to_texture(mandelbrot, self, mb_color_func);
	}
	
	pub fn from_julia(&mut self, julia: &Julia, jul_color_func: fn(&Julia, &JuliaPoint, &mut Color) ) {
		julia_to_texture(julia, self, jul_color_func);
	}
	
	pub fn filter(&mut self, filter: &TextureFilter) {
		filter.filter(self);
	}

	pub fn write_as_ppm(&self, filename: &'static str) {
		let mut f = File::create(filename).expect("Unable to create file");
		write!(f, "P6\n{} {}\n255\n", self.width, self.height).expect("Unable to write header");
		for color in &self.buffer {
			let col: &[u8] = &[color.r as u8, color.g as u8, color.b as u8];
			f.write_all(col).expect("Unable to write data");
		}
	}
	
}


#[cfg(test)]
mod texture_test;
