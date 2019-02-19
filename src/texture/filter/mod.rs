use texture::*;
use std::cmp::Ordering::Equal;
use std::f32::consts::PI;

pub trait TextureFilter {
	fn filter(&self, texture: &mut Texture);
}

pub struct InvertFilter {}

impl TextureFilter for InvertFilter {
	fn filter(&self, texture: &mut Texture) {
		for color in texture.buffer.iter_mut() {
			color.invert();
		}
	}
}

pub struct MedianBoxFilter {
	pub pxrange: i32,
	pub factor: f32,
}

impl MedianBoxFilter {
	pub fn new(	pxrange: i32, factor: f32) -> MedianBoxFilter {
		MedianBoxFilter { pxrange, factor }
	}
}

impl TextureFilter for MedianBoxFilter {
	fn filter(&self, texture: &mut Texture) {
		let mut maxlenght: usize = (self.pxrange*2+1) as usize;
		maxlenght *= maxlenght;
		let mut values_r: Vec<f32> = Vec::with_capacity(maxlenght);
		let mut values_g: Vec<f32> = Vec::with_capacity(maxlenght);
		let mut values_b: Vec<f32> = Vec::with_capacity(maxlenght);
		
		let maxidx: i32 =  texture.buffer.capacity() as i32;
		let twidth: i32 = texture.width as i32;
		
		let mut cntvalues: usize;
		
		let pxupper = self.pxrange + 1;
		
		for h in 0..texture.height {
			for w in 0..texture.width {
				cntvalues = 0;
				for pry in -self.pxrange..pxupper {
					for prx in -self.pxrange..pxupper {
						let curidx = (h as i32+pry) * twidth + (w as i32+prx);
						if curidx >= 0 && curidx < maxidx {
							let color = &texture.buffer[curidx as usize];
							values_r[cntvalues] = color.r;
							values_g[cntvalues] = color.g;
							values_b[cntvalues] = color.b;
						}
					}
				}
				
				values_r.sort_by(| a, b | a.partial_cmp(b).unwrap_or(Equal));
				values_g.sort_by(| a, b | a.partial_cmp(b).unwrap_or(Equal));
				values_b.sort_by(| a, b | a.partial_cmp(b).unwrap_or(Equal));
				
				let mut curcolor = &mut texture.buffer[h as usize * twidth as usize + w as usize];
				curcolor.set_from_rgb(values_r[values_r.capacity() >> 1] * self.factor,
									  values_g[values_g.capacity() >> 1] * self.factor,
									  values_b[values_b.capacity() >> 1] * self.factor);
			}
		}
	}
}


pub struct CubicBoxFilter {
	pub pxrange: i32,
	pub factor: f32,
}

impl CubicBoxFilter {
	pub fn new(	pxrange: i32, factor: f32) -> CubicBoxFilter {
		CubicBoxFilter { pxrange, factor }
	}
}

impl TextureFilter for CubicBoxFilter {
	fn filter(&self, texture: &mut Texture) {
	
		let mut curval = BLACK.clone();
	
		let maxidx: i32 =  texture.buffer.capacity() as i32;
		let twidth: i32 = texture.width as i32;
		let mut cntvalues: i32;
		
		let pxupper = self.pxrange + 1;
		
		for h in 0..texture.height {
			for w in 0..texture.width {
				curval.reset();
				cntvalues = 0;
				for pry in -self.pxrange..pxupper {
					for prx in -self.pxrange..pxupper {
					
						let curidx = (h as i32+pry) * twidth + (w as i32+prx);
						if curidx >= 0 && curidx < maxidx {
							let color = &texture.buffer[curidx as usize];
							curval.r += color.r*color.r*color.r;
							curval.r += color.g*color.g*color.g;
							curval.r += color.b*color.b*color.b;
							cntvalues += 1;
 						}
					}
				}
				
				let mul: f32 = 1./cntvalues as f32;
				let mut curcolor = &mut texture.buffer[h as usize * twidth as usize + w as usize];
				
				curcolor.r = (curval.r * mul).cbrt() * self.factor;
				curcolor.g = (curval.g * mul).cbrt() * self.factor;
				curcolor.b = (curval.b * mul).cbrt() * self.factor;
			}
		}
	}
}

pub struct MedianCrossFilter {
	pub pxrange: i32,
	pub factor: f32,
}

impl MedianCrossFilter {
	pub fn new(	pxrange: i32, factor: f32) -> MedianCrossFilter {
		MedianCrossFilter { pxrange, factor }
	}
}

impl TextureFilter for MedianCrossFilter {
	fn filter(&self, texture: &mut Texture) {
		let mut maxlenght: usize = (self.pxrange*2+1) as usize;
		maxlenght *= maxlenght;
	
		let mut values_r: Vec<f32> = Vec::with_capacity(maxlenght);
		let mut values_g: Vec<f32> = Vec::with_capacity(maxlenght);
		let mut values_b: Vec<f32> = Vec::with_capacity(maxlenght);
	
		let mut cntvalues: usize;
		
		let twidth: i32 = texture.width as i32;
		let theight: i32 = texture.height as i32;
		
		let pxupper = self.pxrange + 1;
		
		for h in 0..texture.height {
			for w in 0..texture.width {
				cntvalues = 0;
				
				for delta in -self.pxrange..pxupper {
					if delta == 0 { continue; }
					
					//handle x
					let temp = w as i32 + delta;
					if temp >= 0 && temp < twidth {
						let curcol: &Color = &texture.buffer[(h as i32 * twidth + temp) as usize];
						values_r[cntvalues] = curcol.r;
						values_g[cntvalues] = curcol.g;
						values_b[cntvalues] = curcol.b;
						cntvalues += 1;
					}
					
					//handle y
					let temp = h as i32 + delta;
					if temp >= 0 && temp < theight {
						let curcol: &Color = &texture.buffer[(temp * twidth + w as i32) as usize];
						values_r[cntvalues] = curcol.r;
						values_g[cntvalues] = curcol.g;
						values_b[cntvalues] = curcol.b;
						cntvalues += 1;
					}
				
				}
				
				values_r.sort_by(| a, b | a.partial_cmp(b).unwrap_or(Equal));
				values_g.sort_by(| a, b | a.partial_cmp(b).unwrap_or(Equal));
				values_b.sort_by(| a, b | a.partial_cmp(b).unwrap_or(Equal));
				
				let mut curcolor = &mut texture.buffer[h as usize * twidth as usize + w as usize];
				curcolor.set_from_rgb(values_r[values_r.capacity() >> 1] * self.factor,
									  values_g[values_g.capacity() >> 1] * self.factor,
									  values_b[values_b.capacity() >> 1] * self.factor);
				
			}
		}
	}
}

pub struct ArithmeticBoxFilter {
	pub pxrange: i32,
	pub factor: f32,
}

impl ArithmeticBoxFilter {
	pub fn new(	pxrange: i32, factor: f32) -> ArithmeticBoxFilter {
		ArithmeticBoxFilter { pxrange, factor }
	}
}

impl TextureFilter for ArithmeticBoxFilter {
	fn filter(&self, texture: &mut Texture) {
		let mut curval = BLACK.clone();
	
		let maxidx: i32 =  texture.buffer.capacity() as i32;
		let twidth: i32 = texture.width as i32;
		let mut cntvalues: i32;
		
		let pxupper = self.pxrange + 1;
		
		for h in 0..texture.height {
			for w in 0..texture.width {
				curval.reset();
				cntvalues = 0;
				for pry in -self.pxrange..pxupper {
					for prx in -self.pxrange..pxupper {
					
						let curidx = (h as i32+pry) * twidth + (w as i32+prx);
						if curidx >= 0 && curidx < maxidx {
							let color = &texture.buffer[curidx as usize];
							curval += color;
							cntvalues += 1;
 						}
					}
				}
				
				let mul: f32 = 1./cntvalues as f32 * self.factor;
				let mut curcolor = &mut texture.buffer[h as usize * twidth as usize + w as usize];
				
				curval *= mul;
	
				curcolor.set_from_color(&curval);
			}
		}
	}
}

pub struct GaussFilter {
	pub pxrange: i32,
	pub deviation: f32,
}

impl GaussFilter {
	pub fn new(	pxrange: i32, deviation: f32) -> GaussFilter {
		GaussFilter { pxrange, deviation }
	}
}

impl TextureFilter for GaussFilter {
	fn filter(&self, texture: &mut Texture) {
		
		let kernelside: i32 = 1+(2*self.pxrange);
		let mut gausskernel: Vec<f32> = Vec::with_capacity((kernelside*kernelside) as usize);
		
		for _i in 0..gausskernel.capacity() {
			gausskernel.push(0.);
		}
		
		let useddeviavtion:f32 = 2.*self.deviation*self.deviation;
		let base:f32=1./(PI*useddeviavtion);
		
		let pxupper = self.pxrange + 1;
		
		let mut ky = 0;
		for pry in -self.pxrange..pxupper {
			let y_2: f32 = (pry*pry) as f32;
			let cur_h: usize = (ky * kernelside) as usize;
			let mut kx: usize = 0;
			
			for prx in -self.pxrange..pxupper {
				let x_2: f32 = (prx*prx) as f32;
				gausskernel[cur_h + kx] = base * (-((y_2+x_2)/useddeviavtion)).exp();
				kx += 1;
			}
			ky += 1;
		}		
		
		let mut curval = BLACK.clone();
		
		let maxidx: i32 = texture.buffer.capacity() as i32;
		let twidth: i32 = texture.width as i32;
		let theight: i32 = texture.height as i32;
		
		for h in 0..texture.height {
			for w in 0..texture.width {
				curval.reset();
				
				ky = 0;
				for pry in -self.pxrange..pxupper {

					let cur_h: usize = (ky * kernelside) as usize;
					let mut kx: usize = 0;
					
					for prx in -self.pxrange..pxupper {
						
						let dy:i32 = h as i32 + pry;
						let dx:i32 = w as i32 + prx;
						let curidx = dy * twidth + dx;
						
						if dx >= 0 && dx < twidth &&
						   dy >= 0 && dy < theight &&
						   curidx >= 0 && curidx < maxidx {
							
							let color = &texture.buffer[curidx as usize];
							let factor: f32 = gausskernel[cur_h + kx];
							
							curval.r += color.r * factor;
							curval.g += color.g * factor;
							curval.b += color.b * factor;
			
 						}
						
						kx += 1;
					}
					ky += 1;
				}
				
				let mut curcolor = &mut texture.buffer[h as usize * twidth as usize + w as usize];
				curcolor.set_from_color(&curval);
				
			}
		}
		
	}
}
