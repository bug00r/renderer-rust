use math::vec::vec2::*;
use math::vec::vec3::*;
use math::mat::mat3::*;
use color::*;

#[derive(Clone)]
pub struct Vertex {
	pub vec: Vec3,
	pub color: Color,
	pub tex_coord: Vec2,
}

#[derive(Clone)]
pub struct Shape {
	pub tex_id: i32,
	pub vertices: Vec<Vertex>,
} 

impl Vertex {
	pub fn new(&mut self) -> Vertex {
		Vertex{  
			vec: Vec3::new_empty(),
			color: BLACK.clone(),
			tex_coord: Vec2::new_empty(),
		}
	}
	
	pub fn new_init(vec: Vec3, color: Color, tex_coord: Vec2) -> Vertex {
		Vertex{ vec, color, tex_coord}
	}
	
	pub fn set_from(&mut self, vertex: &Vertex) {
		self.vec.set_from(&vertex.vec);
		self.tex_coord.set_from(&vertex.tex_coord);
		self.color.set_from_color(&vertex.color);
	}
	
	pub fn set_to(&self, vertex: &mut Vertex) {
		vertex.vec.set_from(&self.vec);
		vertex.tex_coord.set_from(&self.tex_coord);
		vertex.color.set_from_color(&self.color);
	}
	
}

impl Shape {
	
	pub fn new( cnt_vertex: usize) -> Shape {
		Shape { tex_id: -1, vertices: Vec::with_capacity(cnt_vertex) }
	}
	
	pub fn new_init(tex_id: i32, vertices: &Vec<Vertex>) -> Shape {
		Shape { tex_id , vertices: vertices.to_vec() }
	}
	
	pub fn new_point(vec: &Vec3) -> Shape {
		let mut newshape = Shape::new(1);
		newshape.vertices.push( Vertex::new_init(vec.clone(), RED.clone(), Vec2::new(0., 0.)));
		newshape
	}
	
	pub fn new_line(start: &Vec3, end: &Vec3) -> Shape {
		let mut newshape = Shape::new(2);
		newshape.vertices.push( Vertex::new_init(start.clone(), RED.clone(), Vec2::new(0., 0.)));
		newshape.vertices.push( Vertex::new_init(end.clone(), LIME.clone(), Vec2::new(0., 0.)));
		newshape
	}
	
	pub fn new_triangle(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> Shape {
		let mut newshape = Shape::new(3);
		newshape.vertices.push( Vertex::new_init(v1.clone(), RED.clone(), Vec2::new(0., 0.)));
		newshape.vertices.push( Vertex::new_init(v2.clone(), WHITE.clone(), Vec2::new(0., 0.)));
		newshape.vertices.push( Vertex::new_init(v3.clone(), BLUE.clone(), Vec2::new(0., 0.)));
		newshape
	}
	
	pub fn set_from(&mut self, shape: &Shape){
		self.tex_id = shape.tex_id;
		self.vertices = shape.vertices.to_vec();
	}
	
	pub fn set_to(&self, shape: &mut Shape){
		shape.tex_id = self.tex_id;
		shape.vertices = self.vertices.to_vec();
	}
	
	pub fn set_color(&mut self, color: &Color) {
		for vertex in self.vertices.iter_mut() {
			vertex.color.set_from_color(color);
		}
	}
	
	pub fn scale(&mut self, x: f32, y: f32, z: f32) {
		for vertex in self.vertices.iter_mut() {
			vertex.vec.x *= x;
			vertex.vec.y *= y;
			vertex.vec.z *= z;
		}
	}
	
	pub fn translate(&mut self, x: f32, y: f32, z: f32) {
		for vertex in self.vertices.iter_mut() {
			vertex.vec.x += x;
			vertex.vec.y += y;
			vertex.vec.z += z;
		}
	}
	
	pub fn transform(&mut self, mat: &Mat3) {
		for vertex in self.vertices.iter_mut() {
			vertex.vec *= mat;
		}
	}
	
}
