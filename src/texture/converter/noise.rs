use texture::*;
use math::algorithm::noise::core::*;
use math::utils::{interpolate_lin};

pub fn noise_to_texture(noise: &Noise, target: &mut Texture) {
	if target.width == noise.width && target.height == noise.height {
		for (i, color) in target.buffer.iter_mut().enumerate() {
			color.set_all(interpolate_lin(noise.map[i], noise.min, 0., noise.max, 255.));
		}
	}
}