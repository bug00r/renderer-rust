use texture::*;
use texture::filter::*;
use texture::converter::julia::*;
use texture::converter::mandelbrot::*;

use math::algorithm::noise::md::*;
use math::algorithm::noise::ds::*;
use math::algorithm::fractals::mandelbrot::*;
use math::algorithm::fractals::julia::*;

#[test]
#[cfg(any(feature = "all", feature = "ds"))]
fn noise_texture_ds() {	
	let w: i32 = 513;
	let h: i32 = w;
	let maxreduction: f32 = 1.;
	let reduction: f32 = 0.5;
	let mut ds: DiamondSquare = DiamondSquare::new(w as u32, h as u32);
	ds.length = w-1;
	ds.startseed = 1.;
	ds.seed = maxreduction;
	ds.reduction = reduction;
	ds.create();
	
	let mut texture: Texture =  Texture::new(w as u32, h as u32);
	texture.from_noise(&ds.noise);
	
	if cfg!(feature = "output") {
		texture.write_as_ppm("target/ds_noise.ppm");
	}
	
	if cfg!(feature = "filter_arith") {
		let texfilter = ArithmeticBoxFilter::new(2, 1.0);
		let mut tex_arith = texture.clone();
		tex_arith.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_arith.write_as_ppm("target/sd_noise_arith_filter.ppm");
		}
	}
	
	if cfg!(feature = "filter_gauss") {
		let texfilter = GaussFilter::new(2, 1.0);

		let mut tex_gauss = texture.clone();
		tex_gauss.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_gauss.write_as_ppm("target/sd_noise_gauss_filter.ppm");
		}
	}
	
}

#[test]
#[cfg(any(feature = "all", feature = "md"))]
fn noise_texture_md() {	
	let w: i32 = 513;
	let h: i32 = w;
	let maxreduction: f32 = 1.;
	let reduction: f32 = 0.5;
	let mut md: MidpointDisplacement = MidpointDisplacement::new(w as u32, h as u32);
	md.length = w-1;
	md.startseed = 1.;
	md.seed = maxreduction;
	md.reduction = reduction;
	md.create();
	
	let mut texture: Texture =  Texture::new(w as u32, h as u32);
	texture.from_noise(&md.noise);
	
	if cfg!(feature = "output") {
		texture.write_as_ppm("target/md_noise.ppm");
	}
	
	if cfg!(feature = "filter_arith") {
		let texfilter = ArithmeticBoxFilter::new(2, 1.0);
		let mut tex_arith = texture.clone();
		tex_arith.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_arith.write_as_ppm("target/md_noise_arith_filter.ppm");
		}
	}
	
	if cfg!(feature = "filter_gauss") {
		let texfilter = GaussFilter::new(2, 1.0);

		let mut tex_gauss = texture.clone();
		tex_gauss.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_gauss.write_as_ppm("target/md_noise_gauss_filter.ppm");
		}
	}
	
}

#[test]
#[cfg(any(feature = "all", feature = "mandelbrot"))]
fn fractal_texture_mandelbrot() {	
	let mut mb: Mandelbrot = Mandelbrot::new(512, 512);		
	
	mb.min.re = -2.;
	mb.max.re = 0.5;
	mb.min.im = -1.;
	mb.max.im = 1.;
	mb.cntiterations = 20;
	mb.create();
	
	let mut texture: Texture =  Texture::new(mb.width, mb.height);
	texture.from_mandelbrot(&mb, mandelbrot_color_line_int_rgb);
	
	if cfg!(feature = "output") {
		texture.write_as_ppm("target/mandelbrot.ppm");
	}
	
	if cfg!(feature = "filter_arith") {
		let texfilter = ArithmeticBoxFilter::new(2, 1.0);
		let mut tex_arith = texture.clone();
		tex_arith.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_arith.write_as_ppm("target/mandelbrot_arith_filter.ppm");
		}
	}
	
	if cfg!(feature = "filter_gauss") {
		let texfilter = GaussFilter::new(2, 1.0);

		let mut tex_gauss = texture.clone();
		tex_gauss.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_gauss.write_as_ppm("target/mandelbrot_gauss_filter.ppm");
		}
	}
	
}

#[test]
#[cfg(any(feature = "all", feature = "julia"))]
fn fractal_texture_julia() {	
	let mut julia: Julia = Julia::new(512, 512);
	julia.min.re = -1.1;
	julia.max.re = 1.1;
	julia.min.im = -1.1;
	julia.max.im = 1.1;
	julia.cntiterations = 200;
	julia.c.re = 0.;
	julia.c.im = 1.;
	
	julia.create();
	
	let mut texture: Texture =  Texture::new(julia.width, julia.height);
	texture.from_julia(&julia, julia_color_line_int_8_bit);
	
	if cfg!(feature = "output") {
		texture.write_as_ppm("target/julia.ppm");
	}
	
	if cfg!(feature = "filter_arith") {
		let texfilter = ArithmeticBoxFilter::new(2, 1.0);
		let mut tex_arith = texture.clone();
		tex_arith.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_arith.write_as_ppm("target/julia_arith_filter.ppm");
		}
	}
	
	if cfg!(feature = "filter_gauss") {
		let texfilter = GaussFilter::new(2, 1.0);

		let mut tex_gauss = texture.clone();
		tex_gauss.filter(&texfilter);
		if cfg!(feature = "output") {
			tex_gauss.write_as_ppm("target/julia_gauss_filter.ppm");
		}
	}
	
}



