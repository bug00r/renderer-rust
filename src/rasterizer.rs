use color::*;
use math::vec::vec2::*;
use math::vec::vec3::*;
use math::mat::mat4::*;
use math::utils::*;
use texture::*;
use camera::*;
use objects::shape::*;
use objects::mesh::*;
use objects::scene::*;
use std;
use std::io::Write;                                                                                                                                                                                                                                                                                                                            
use std::fs::File;   

pub struct Renderer {
	pub img_width: i32,
	pub img_height: i32,
	pub img_width_half: f32,
	pub img_height_half: f32,
	pub buf_width : i32,
	pub buf_height: i32,
	pub camera: Camera,
	pub samplestep: u32,
	pub used_samples: u32,
	pub samples: Vec<Vec3>,
	pub sample_factor: f32,
	pub frame_buffer: Vec<Color>,
	pub z_buffer: Vec<f32>,
	pub texture: Vec<Texture>, //TODO here we need a list of textures...currently we use one as test
	pub bgcolor: Color,
	pub min_z: f32,
	pub max_z: f32,
}

impl Renderer {

	pub fn new(img_width: i32, img_height: i32, bgcolor: &Color, samplestep: u32) -> Renderer {
		let us: u32 = samplestep * samplestep;
		let bw: u32 = img_width as u32 * us;
		let buffersize: usize = (img_width * img_height * us as i32) as usize;
		
		let mut renderer = Renderer {
			camera: Camera::new(),
			min_z: std::f32::MAX, max_z: 0.0,
			texture: Vec::new(),
			img_width, img_height,
			img_width_half: img_width as f32 * 0.5,
			img_height_half: img_height as f32 * 0.5,
			buf_width : bw as i32,
			buf_height: img_height,
			samplestep,
			frame_buffer: Vec::with_capacity(buffersize),
			z_buffer: Vec::with_capacity(buffersize),
			used_samples: us,
			sample_factor: 1.0 / us as f32,
			samples: Vec::with_capacity(us as usize),
			bgcolor: bgcolor.clone()
		};
		
		for i in 0..buffersize {
			renderer.z_buffer.push(std::f32::MAX);
			renderer.frame_buffer.push(BLACK.clone());
		}
		
		for i in 0..us {
			renderer.samples.push(Vec3::new_empty());
		}
		
		let stepstart: f32 = 0.5 / samplestep as f32; //for st = 2 step is .25  for st = 4 0.125
		let step: f32 = 2.0*stepstart; //distance between 
		for sy in 0..samplestep {
			for sx in 0..samplestep {
				let cursample: Vec3 = Vec3::new(stepstart + (sx as f32 * step), stepstart + (sy as f32 * step), 0.0);
				renderer.samples[(sy * samplestep + sx) as usize].set_from(&cursample);
			}
		}
		
		renderer
	}
	
	fn _set_color_to_fb_(&mut self, bi: &u32, new_color: &Color) {
		let fbc = &mut self.frame_buffer[*bi as usize];
		let sf: &f32 = &self.sample_factor;
		fbc.r = new_color.r * sf;
		fbc.g = new_color.g * sf;
		fbc.b = new_color.b * sf;
	}
	

	fn _compute_px_color( &self, color: &mut Color, bc: &Barycentric, weight1: &f32 , weight2: &f32 , weight3: &f32 ,
						  img_w: &i32, v1c: &Color, v2c: &Color, v3c: &Color,
						  v1t: &Vec2, v2t: &Vec2, v3t: &Vec2, tex_id: &i32 ) {
	
		let z0:f32  = bc.bc0*(*weight1);
		let z1:f32  = bc.bc1*(*weight2);
		let z2:f32  = bc.bc2*(*weight3);
		let z3:f32  = 1.0/(z0 + z1 + z2);
		
		match *tex_id {
			-1 => {
				color.r = (z0*v1c.r + z1*v2c.r + z2*v3c.r ) * z3;
				color.g = (z0*v1c.g + z1*v2c.g + z2*v3c.g ) * z3;
				color.b = (z0*v1c.b + z1*v2c.b + z2*v3c.b ) * z3;
				}
			_ => {
				let texx: usize = (( z0*v1t.x + z1*v2t.x + z2*v3t.x ) * z3 * 512.0) as usize;
				let texy: usize = (( z0*v1t.y + z1*v2t.y + z2*v3t.y ) * z3 * 512.0) as usize;
				
				let txc: &Color = &self.texture[0].buffer[texy * (*img_w as usize) + texx];
				color.r = txc.r;
				color.g = txc.g;
				color.b = txc.b;
			}						
		}
	}
	
	fn _compute_and_set_z(&mut self, rz1: &f32, rz2: &f32, rz3: &f32, bc: &Barycentric, bi: &u32) -> bool {
		let mut z = *rz1 * bc.bc0;
		z += *rz2 * bc.bc1;
		z += *rz3 * bc.bc2;
		
		let old_z: &mut f32 = &mut self.z_buffer[*bi as usize];
		
		if z > *old_z  { return true; }
		
		*old_z = z;
		
		//only for z buffer print 
		self.min_z = self.min_z.min(z);
		self.max_z = self.max_z.max(z);
		
		false
	}
	
	fn _compute_and_set_z_line(&mut self, rz1: &f32, rz2: &f32, bc: &Barycentric, bi: &u32) -> bool {
		let mut z = *rz1 * bc.bc0;
		z += *rz2 * bc.bc1;
		
		let old_z: &mut f32 = &mut self.z_buffer[*bi as usize];
		
		if z > *old_z  { return true; }
		
		*old_z = z;
		
		//only for z buffer print 
		self.min_z = self.min_z.min(z);
		self.max_z = self.max_z.max(z);
		
		false
	}

	fn _compute_and_set_z_point(&mut self, rz1: &f32, bi: &u32) -> bool {
		
		let old_z: &mut f32 = &mut self.z_buffer[*bi as usize];
		
		if *rz1 > *old_z  { return true; }
		
		*old_z = *rz1;
		
		//only for z buffer print 
		self.min_z = self.min_z.min(*rz1);
		self.max_z = self.max_z.max(*rz1);
		
		false
	}
	
	fn update_sample(pixel_sample: &mut Vec3, cursample: &Vec3, cur_w: &u32, cur_h: &u32) {
		pixel_sample.x = *cur_w as f32;
		pixel_sample.x += cursample.x;
		pixel_sample.y = *cur_h as f32;
		pixel_sample.y += cursample.y;
	}
	
	fn _compute_sample_bc_and_check(cursample: &Vec3, cur_w: &u32, cur_h: &u32,
										 bc: &mut Barycentric, p_raster1: &Vec3, p_raster2: &Vec3,p_raster3: &Vec3) -> bool {
	
		let mut pixel_sample = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
		Renderer::update_sample(&mut pixel_sample, cursample, cur_w, cur_h);
		
		bc.w0_12 = (pixel_sample.x - p_raster2.x) * (p_raster3.y - p_raster2.y) - (pixel_sample.y - p_raster2.y) * (p_raster3.x - p_raster2.x);
		if bc.w0_12 < 0.0 { return true; }
		
		bc.w1_20 = (pixel_sample.x - p_raster3.x) * (p_raster1.y - p_raster3.y) - (pixel_sample.y - p_raster3.y) * (p_raster1.x - p_raster3.x);
		if bc.w1_20 < 0.0 { return true; }
		
		bc.w2_01 = (pixel_sample.x - p_raster1.x) * (p_raster2.y - p_raster1.y) - (pixel_sample.y - p_raster1.y) * (p_raster2.x - p_raster1.x);
		if bc.w2_01 < 0.0 { return true; }
		
		bc.bc0 = bc.w0_12 * bc.area;
		bc.bc1 = bc.w1_20 * bc.area;
		bc.bc2 = bc.w2_01 * bc.area;
	
		false
	}
	
	fn _compute_sample_bc_and_check_line(cursample: &Vec3, cur_w: &u32, cur_h: &u32,
										 bc: &mut Barycentric, p_raster1: &Vec3, p_raster2: &Vec3) -> bool {
			
			let mut pixel_sample = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
			Renderer::update_sample(&mut pixel_sample, cursample, cur_w, cur_h);
			
			bc.inside = false;
			bc.bc2 = (pixel_sample.x - p_raster1.x ) / (p_raster2.x - p_raster1.x);
			let edge: f32 = place_of_vec3(p_raster1, p_raster2, &pixel_sample);
			let mut limitvec = p_raster2.clone();
			limitvec -= p_raster1;
			let limit: f32 = limitvec.len() * 0.5;
			if  (edge <= limit && edge >= 0.0) || (edge >= -limit && edge <= 0.0)  {		 
				return false;
			}
			
			true
	}
	
	fn _compute_sample_and_check_point(cur_w: &mut u32, cur_h: &mut u32,
									   img_w: &i32, img_h: &i32, p_raster1: &Vec3) -> bool {
						
		if (p_raster1.x >= 0.0) &&
			 (p_raster1.x <= *img_w as f32) &&
			 (p_raster1.y >= 0.0) &&
			 (p_raster1.y <= *img_h as f32) {
				*cur_w = p_raster1.x as u32;
				*cur_h = p_raster1.y as u32;
				return false;
			 }
			
		true
	}

	fn _compute_min_max_w_h(maxx: &mut u32, maxy: &mut u32, minx: &mut u32,miny: &mut u32,
							cur_w: &mut u32, cur_h: &mut u32, img_w: &i32, img_h: &i32,
							p_raster1: &Vec3, p_raster2: &Vec3,p_raster3: &Vec3) {
		*maxx = (*img_w as u32).min((p_raster1.x as u32).max((p_raster2.x as u32).max(p_raster3.x as u32)));
		*maxy = (*img_h as u32).min((p_raster1.y as u32).max((p_raster2.y as u32).max(p_raster3.y as u32)));
		*minx = (0 as u32).max((p_raster1.x as u32).min((p_raster2.x as u32).min(p_raster3.x as u32)));
		*miny = (0 as u32).max((p_raster1.y as u32).min((p_raster2.y as u32).min(p_raster3.y as u32)));
		*cur_h = *miny;
		*cur_w = *minx;
	}
	
	fn _compute_min_max_w_h_line(maxx: &mut u32, maxy: &mut u32, minx: &mut u32,miny: &mut u32,
								 cur_w: &mut u32, cur_h: &mut u32, img_w: &i32, img_h: &i32,
								 p_raster1: &Vec3, p_raster2: &Vec3) {
								 
		*maxx = (*img_w as u32).min((p_raster1.x as u32).max(p_raster2.x as u32));
		*maxy = (*img_h as u32).min((p_raster1.y as u32).max(p_raster2.y as u32));
		*minx = (0 as u32).max((p_raster1.x as u32).min(p_raster2.x as u32));
		*miny = (0 as u32).max((p_raster1.y as u32).min(p_raster2.y as u32));
		*cur_h = *miny;
		*cur_w = *minx;
	}
	
	fn _world_to_raster(v: &Vec3, ndc: &mut Vec3, raster: &mut Vec3, weight: &mut f32,
					   img_w_h: &f32, img_h_h: &f32, rz3: &mut f32, ct: &Mat4) -> bool {	
							 
		ndc.x += (v.x * ct._11) + (v.y * ct._12) + (v.z * ct._13);// + ct._14;
		ndc.y += (v.x * ct._21) + (v.y * ct._22) + (v.z * ct._23);// + ct._24;
		ndc.z += (v.x * ct._31) + (v.y * ct._32) + (v.z * ct._33);// + ct._34;
		*weight += (v.x * ct._41) + (v.y * ct._42) + (v.z * ct._43);// + ct._44;
		if *weight < 0.0 { return true; }
		if *weight != 1.0 && *weight != 0.0 {
			*weight = 1.0/(*weight); ndc.x *= *weight; ndc.y *= *weight; ndc.z *= *weight;
		}	
		raster.x = (ndc.x + 1.0) * (*img_w_h);
		raster.y = (1.0-ndc.y) * (*img_h_h);
		raster.z = -ndc.z;
		*rz3 = 1.0/raster.z;
		false
	}
	
	fn render_point(&mut self, shape: &Shape ) {
		
		//let ct: &Mat4 = &self.camera.transformation;
		let mut p_ndc1 = Vec3{x: self.camera.transformation._14, y: self.camera.transformation._24, z: self.camera.transformation._34};
		let mut p_raster1: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
		let mut weight1 = self.camera.transformation._44;
		let rz1: &mut f32 = &mut 0.0;
		let v1: &Vertex = &shape.vertices[0]; 
		let v1v: &Vec3 = &v1.vec;
		let v1c: &Color = &v1.color;
		
		if Renderer::_world_to_raster(v1v, &mut p_ndc1, &mut p_raster1, &mut weight1, 
									  &self.img_width_half, &self.img_height_half, rz1, &self.camera.transformation) { return; }
		
		let maxx: u32 = 1; 
		let maxy: u32 = 1; 
		let minx: u32 = 0; 
		let miny: u32 = 0;
		let mut cur_h: u32 = miny; 
		
		let used_samples: u32 = self.samples.len() as u32;
		
		while cur_h < maxy {
			let mut cur_w: u32 = minx;
			while cur_w < maxx {				
	
				for sample in 0..used_samples {
				
					if Renderer::_compute_sample_and_check_point(
									&mut cur_w, &mut cur_h, &self.img_width, &self.img_height, &p_raster1) { continue; }
					
					let bi: u32 = cur_h * self.buf_width  as u32 + (cur_w * used_samples) + sample as u32;		
					
					if self._compute_and_set_z_point(&rz1, &bi) { continue; }
					
					self._set_color_to_fb_(&bi, v1c);
				}
				
				cur_w += 1;
			}
			
			cur_h += 1;
		}
		
	}
	fn render_line(&mut self, shape: &Shape ) {
	
		let vertices: &Vec<Vertex> = &shape.vertices;
		let v1: &Vertex = &vertices[0]; 
		let v2: &Vertex = &vertices[1];
		let v1v: &Vec3 = &v1.vec;
		let v2v: &Vec3 = &v2.vec;
		let v1c: &Color = &v1.color;
		let mut cursample: Vec3;
		let mut p_ndc1: Vec3 = Vec3{x: self.camera.transformation._14,y: self.camera.transformation._24,z: self.camera.transformation._34}; 
		let mut p_ndc2: Vec3 = Vec3{x: self.camera.transformation._14,y: self.camera.transformation._24,z: self.camera.transformation._34}; 
		let mut p_raster1: Vec3= Vec3 {x: 0.0, y: 0.0, z: 0.0}; 
		let mut p_raster2: Vec3= Vec3 {x: 0.0, y: 0.0, z: 0.0}; 
		let mut cur_w: u32 = 0;
		let mut cur_h: u32 = 0;
		let mut maxx: u32 = 0; 
		let mut maxy: u32 = 0;
		let mut minx: u32 = 0;
		let mut miny: u32 = 0; 
		let mut weight1: f32 = self.camera.transformation._44; 
		let mut weight2: f32 = self.camera.transformation._44; 
		let mut rz1: f32 = 0.0; 
		let mut rz2: f32 = 0.0;
		let mut bc: Barycentric = Barycentric::new();
		
		if Renderer::_world_to_raster(v1v, &mut p_ndc1, &mut p_raster1, &mut weight1, 
									  &self.img_width_half, &self.img_height_half, &mut rz1, &self.camera.transformation) { return; }
		if Renderer::_world_to_raster(v2v, &mut p_ndc2, &mut p_raster2, &mut weight2, 
									  &self.img_width_half, &self.img_height_half, &mut rz2, &self.camera.transformation) { return; }
		
		//println!("v1: {} {} {}", v1v.x, v1v.y, v1v.z);
		//println!("n1: {} {} {}", p_ndc1.x, p_ndc1.y, p_ndc1.z);
		//println!("r1: {} {} {}", p_raster1.x, p_raster1.y, p_raster1.z);
		//println!("v2: {} {} {}", v2v.x, v2v.y, v2v.z);
		//println!("n2: {} {} {}", p_ndc2.x, p_ndc2.y, p_ndc2.z);
		//println!("r2: {} {} {}", p_raster2.x, p_raster2.y, p_raster2.z);
		
		Renderer::_compute_min_max_w_h_line(&mut maxx, &mut maxy, &mut minx, &mut miny, &mut cur_w, &mut cur_h, 
											&self.img_width, &self.img_height, &p_raster1, &p_raster2);
		
		//println!("minx: {} maxy: {}", minx, maxy);
		
		while cur_h < maxy {
			let mut cur_w: u32 = minx;
			let cur_hbuf_width: u32 = cur_h * self.buf_width as u32;
			while cur_w < maxx {				
				let cur_wused_samples: u32 = cur_hbuf_width + (cur_w * self.used_samples);
				for sample in 0..self.used_samples {
					{
						let cursample: &Vec3 = &self.samples[sample as usize];
					
						if Renderer::_compute_sample_bc_and_check_line(cursample,&cur_w, &cur_h, &mut bc,
											 &p_raster1, &p_raster2) { continue; }
					}
					let bi: u32 = cur_wused_samples + sample as u32;		
					
					if self._compute_and_set_z_line(&mut rz1, &mut rz2, &bc, &bi) { continue; }
					
					self._set_color_to_fb_(&bi, v1c);
				}
				cur_w += 1;
			}
			
			cur_h += 1;
		}
	}
	
	fn render_triangle(&mut self, shape: &Shape ) {
		let vertices: &Vec<Vertex> = &shape.vertices;
		let v1: &Vertex = &vertices[0]; 
		let v2: &Vertex = &vertices[1];
		let v3: &Vertex = &vertices[2];
		let v1v: &Vec3 = &v1.vec;
		let v2v: &Vec3 = &v2.vec;
		let v3v: &Vec3 = &v3.vec;
		let v1c: &Color = &v1.color;
		let v2c: &Color = &v2.color;
		let v3c: &Color = &v3.color;
		let v1t: &Vec2 = &v1.tex_coord;
		let v2t: &Vec2 = &v2.tex_coord;
		let v3t: &Vec2 = &v3.tex_coord;
		let mut cursample: Vec3;
		let mut p_ndc1: Vec3 = Vec3{x: self.camera.transformation._14,y: self.camera.transformation._24,z: self.camera.transformation._34}; 
		let mut p_ndc2: Vec3 = Vec3{x: self.camera.transformation._14,y: self.camera.transformation._24,z: self.camera.transformation._34};
		let mut p_ndc3: Vec3 = Vec3{x: self.camera.transformation._14,y: self.camera.transformation._24,z: self.camera.transformation._34}; 
		let mut p_raster1: Vec3= Vec3 {x: 0.0, y: 0.0, z: 0.0}; 
		let mut p_raster2: Vec3= Vec3 {x: 0.0, y: 0.0, z: 0.0};
		let mut p_raster3: Vec3= Vec3 {x: 0.0, y: 0.0, z: 0.0}; 
		let mut cur_w: u32 = 0;
		let mut cur_h: u32 = 0;
		let mut maxx: u32 = 0; 
		let mut maxy: u32 = 0;
		let mut minx: u32 = 0;
		let mut miny: u32 = 0; 
		let mut weight1: f32 = self.camera.transformation._44; 
		let mut weight2: f32 = self.camera.transformation._44; 
		let mut weight3: f32 = self.camera.transformation._44;
		let mut rz1: f32 = 0.0; 
		let mut rz2: f32 = 0.0;
		let mut rz3: f32 = 0.0;
		let mut bc: Barycentric = Barycentric::new();
		
		if Renderer::_world_to_raster(v1v, &mut p_ndc1, &mut p_raster1, &mut weight1, 
									  &self.img_width_half, &self.img_height_half, &mut rz1, &self.camera.transformation) { return; }
		if Renderer::_world_to_raster(v2v, &mut p_ndc2, &mut p_raster2, &mut weight2, 
									  &self.img_width_half, &self.img_height_half, &mut rz2, &self.camera.transformation) { return; }
		if Renderer::_world_to_raster(v3v, &mut p_ndc3, &mut p_raster3, &mut weight3, 
									  &self.img_width_half, &self.img_height_half, &mut rz3, &self.camera.transformation) { return; }
		
		println!("v1: {} {} {}", v1v.x, v1v.y, v1v.z);
		println!("n1: {} {} {}", p_ndc1.x, p_ndc1.y, p_ndc1.z);
		println!("r1: {} {} {}", p_raster1.x, p_raster1.y, p_raster1.z);
		println!("v2: {} {} {}", v2v.x, v2v.y, v2v.z);
		println!("n2: {} {} {}", p_ndc2.x, p_ndc2.y, p_ndc2.z);
		println!("r2: {} {} {}", p_raster2.x, p_raster2.y, p_raster2.z);
		println!("v3: {} {} {}", v3v.x, v3v.y, v3v.z);
		println!("n3: {} {} {}", p_ndc3.x, p_ndc3.y, p_ndc3.z);
		println!("r3: {} {} {}", p_raster3.x, p_raster3.y, p_raster3.z);
		
		bc.area = 1.0/((p_raster3.x - p_raster1.x) * (p_raster2.y - p_raster1.y) - (p_raster3.y - p_raster1.y) * (p_raster2.x - p_raster1.x));
		
		Renderer::_compute_min_max_w_h(&mut maxx, &mut maxy, &mut minx, &mut miny, &mut cur_w, &mut cur_h, 
											&self.img_width, &self.img_height, &p_raster1, &p_raster2, &p_raster3);
		
		println!("minx: {} maxy: {}", minx, maxy);
		
		let mut cur_color: Color = Color {r: 0.0, g: 0.0, b: 0.0};
		
		while cur_h < maxy {
			let mut cur_w: u32 = minx;
			let cur_hbuf_width: u32 = cur_h * self.buf_width as u32;
			while cur_w < maxx {				
				let cur_wused_samples: u32 = cur_hbuf_width + (cur_w * self.used_samples);
				for sample in 0..self.used_samples {
					{
						let cursample: &Vec3 = &self.samples[sample as usize];
					
						if Renderer::_compute_sample_bc_and_check(cursample,&cur_w, &cur_h, &mut bc,
											 &p_raster1, &p_raster2, &p_raster3) { continue; }
					}
					let bi: u32 = cur_wused_samples + sample as u32;		
					
					if self._compute_and_set_z(&mut rz1, &mut rz2, &mut rz3, &bc, &bi) { continue; }
					
					self._compute_px_color(&mut cur_color, &bc, &weight1, &weight2, &weight3,
										   &self.img_width, v1c, v2c, v3c, v1t, v2t, v3t, &shape.tex_id);
					
					self._set_color_to_fb_(&bi, &cur_color);
				}
				cur_w += 1;
			}
			
			cur_h += 1;
		}
		
		println!("minz: {} maxz: {}", self.min_z, self.max_z);
	}
	
	pub fn render_shape(&mut self, shape: &Shape) {
		match shape.vertices.len() {
			3 => self.render_triangle(shape),
			2 => self.render_line(shape),
			1 => self.render_point(shape),
			_ => println!("WTF!!! Invalid Shape"),
		}
	}
	
	pub fn render_mesh(&mut self, mesh: &Mesh) {
		for shape in mesh.shapes.iter() {
			self.render_shape(shape);
		}
	}
	
	pub fn render_scene(&mut self, scene: &Scene) {
		for mesh in scene.meshes.iter() {
			self.render_mesh(mesh);
		}
	}	
	
	pub fn clear_frame(&mut self) {
		let buffersize: usize = (self.img_width as u32 * self.img_height as u32 * self.samplestep * self.samplestep) as usize;
		for i in 0..buffersize {
			self.z_buffer[i] = std::f32::MAX;
			self.frame_buffer[i].set_from_color(&BLACK);
		}
		self.min_z = std::f32::MAX;
		self.max_z = 0.0;
	}
	
	pub fn write_fb_ppm(&self, filename: &'static str) {
		let mut f = File::create(filename).expect("Unable to create file");
		write!(f, "P6\n{} {}\n255\n", self.img_width, self.img_height).expect("Unable to write header");
		let mut fc: Color = BLACK.clone();

		for j in 0..self.img_height {
			let bi: usize = (j * self.buf_width) as usize;
			for i in 0..self.img_width {
				fc.set_from_rgb(0.0, 0.0, 0.0);

				let samplestart: usize = bi + (i as u32 * self.used_samples) as usize;
				for sample in 0..self.used_samples{
					let c: &Color = &self.frame_buffer[samplestart + sample as usize];
					fc.r += c.r;
					fc.g += c.g;
					fc.b += c.b;
				}
				
				let col: &[u8] = &[fc.r as u8, fc.g  as u8, fc.b as u8];
				f.write_all(col).expect("Unable to write data");				
			}
		}
	}	
	
	pub fn write_zb_ppm(&self, filename: &'static str) {
		let mut f = File::create(filename).expect("Unable to create file");
		write!(f, "P6\n{} {}\n255\n", self.img_width, self.img_height).expect("Unable to write header");
		let mut _color: f32= 0.0;

		for j in 0..self.img_height {
			let bi: usize = (j * self.buf_width) as usize;
			for i in 0..self.img_width {
				_color = 0.0;

				let samplestart: usize = bi + (i as u32 * self.used_samples) as usize;
				for sample in 0..self.used_samples{
					_color += &self.z_buffer[samplestart + sample as usize];
				}
				
				_color *= self.sample_factor;
				
				if ( _color != std::f32::MAX ){
					_color = interpolate_lin(_color, self.max_z, 0.0, self.min_z, 255.0);
				} else {
					_color = 0.0;
				}
				
				let col: &[u8] = &[_color as u8, _color as u8, _color as u8];
				f.write_all(col).expect("Unable to write data");				
			}
		}
	}		
	
}