use texture::*;
use color::*;
use math::algorithm::fractals::julia::*;
use math::utils::{interpolate_lin};


pub fn julia_color_normal_8_bit(julia: &Julia, jbt: &JuliaPoint, col: &mut Color){
	let colfactor: f32 = (jbt.iterations as f32)/(julia.cntiterations as f32 *0.2 );
	if  jbt.isin {
		col.r = colfactor * 255.;
		col.g = 0.;//colval;
		col.b = 0.;//colval;
	} else{
		col.r = 0.;
		col.g = 0.;
		col.b = 0.;
	}
}
									
pub fn julia_color_line_int_8_bit(julia: &Julia, jbt: &JuliaPoint, col: &mut Color) {
	if jbt.isin {	
		let colfactor_real: f32 = interpolate_lin(jbt.spoint.re, julia.min.re, 0., julia.max.re, 255.) / 255.;
		let colfactor_imag: f32 = interpolate_lin(jbt.spoint.im, julia.min.im, 0., julia.max.im, 255.) / 255.;
		col.g = interpolate_lin(jbt.iterations as f32, 0., 0., julia.cntiterations as f32 *0.18, 255.) * colfactor_real; //iterations
		col.r = interpolate_lin(jbt.iterations as f32, 0., 0., julia.cntiterations as f32 *0.18, 255.) * ((colfactor_real + colfactor_imag)/8.); //iterations
		col.b = interpolate_lin(jbt.iterations as f32, 0., 0., julia.cntiterations as f32 *0.18, 255.) * colfactor_imag; //iterations
		
	} else{
		col.r = interpolate_lin(jbt.iterations as f32, 0., 0., julia.cntiterations as f32 * 0.3, 255.); //iterations
		col.g = interpolate_lin(jbt.iterations as f32, 0., 0., julia.cntiterations as f32 * 0.2, 255.); //iterations
		col.b = interpolate_lin(jbt.iterations as f32, 0., 0., julia.cntiterations as f32 * 0.1, 255.); //iterations	
	}
}

pub fn julia_to_texture(julia: &Julia, target: &mut Texture, 
					    jul_color_func: fn(&Julia, &JuliaPoint, &mut Color) ) {
	if target.width == julia.width && target.height == julia.height {
		for (i, color) in target.buffer.iter_mut().enumerate() {
			jul_color_func(julia, &julia.map[i], color);
		}
	}
}
