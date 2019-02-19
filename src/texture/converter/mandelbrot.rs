use texture::*;
use color::*;
use math::algorithm::fractals::mandelbrot::*;
use math::utils::{interpolate_lin};

 
pub fn mandelbrot_color_normal_8_bit(mb: &Mandelbrot, mbt: &MandelbrotPoint, col: &mut Color){
	/*
		float colfactor = 1.f/curresult.abs;
		float its = ((float)curresult.iterations)/src->cntiterations;
		float re = crealf(curresult.cpoint)/curresult.iterations;
		float imag = cimagf(curresult.cpoint)/curresult.iterations;
		float diff = cimagf(curresult.cpoint)/crealf(curresult.cpoint);
	*/
	if !mbt.isin {
		let colfactor: f32 = 1./mbt.abs;
		col.r = colfactor * mb.min.re;
		col.g = colfactor * 255.;
		col.b = 0.;//colval;
	} else{
		col.r = 0.;
		col.g = 0.;
		col.b = 0.;
	}
}

pub fn mandelbrot_color_line_int_8_bit(mb: &Mandelbrot, mbt: &MandelbrotPoint, col: &mut Color){
	if !mbt.isin {	
		let cpabs: f32 = ((mbt.cpoint.re * mbt.cpoint.re) + (mbt.cpoint.im * mbt.cpoint.im)).sqrt();
		col.r = interpolate_lin(cpabs, mb.min.re, 0., mb.max.re, 255.); //real
		col.g = 0.;//interpolate_lin(cimagf(mbt->cpoint), mb.min.im, 0.f, mb.max.im, 255.); //imag
		col.b = interpolate_lin(mbt.iterations as f32, 0., 0., mbt.iterations as f32, 255.); //iterations	
	} else {
		col.r = interpolate_lin(mbt.cpoint.re, mb.min.re, 0., mb.max.re, 255.); //real
		col.g = interpolate_lin(mbt.cpoint.im, mb.min.im, 0., mb.max.im, 255.); //imag
		col.b = 0.;//interpolate_lin(mbt.iterations as f32, 0., 0., mbt.iterations as f32, 255.); //iterations	
	}
}
									
pub fn mandelbrot_color_line_int_rgb(mb: &Mandelbrot, mbt: &MandelbrotPoint, col: &mut Color){
	if !mbt.isin {	
		let cpabs: f32 = ((mbt.cpoint.re * mbt.cpoint.re) + (mbt.cpoint.im * mbt.cpoint.im)).sqrt();
		col.r = interpolate_lin(cpabs, mb.min.re, 0., mb.max.re, 255.); //real
		col.g = 0.;//interpolate_lin(mbt.cpoint.im, mb.min.im, 0., mb.max.im, 255.); //imag
		col.b = interpolate_lin( mbt.iterations as f32, 0., 0., mb.cntiterations as f32, 1.); //iterations	
	} else {
		col.r = interpolate_lin(mbt.cpoint.re, mb.min.re, 0., mb.max.re, 255.); //real
		col.g = interpolate_lin(mbt.cpoint.im, mb.min.im, 0., mb.max.im, 255.); //imag
		col.b = 0.;//interpolate_lin(mbt.iterations as f32, 0., 0., mb.cntiterations as f32, 255.); //iterations	
	}
}

pub fn mandelbrot_to_texture(mb: &Mandelbrot, target: &mut Texture, 
							 mb_color_func: fn(&Mandelbrot, &MandelbrotPoint, &mut Color) ) {
	if target.width == mb.width && target.height == mb.height {
		for (i, color) in target.buffer.iter_mut().enumerate() {
			mb_color_func(mb, &mb.map[i], color);
		}
	}
}


