use objects::shape::*;
use math::vec::vec3::*;
use math::mat::mat3::*;
use color::*;

#[test]
fn point() {
	let vec: Vec3 = Vec3::new(1., 2., 3.);
	let shape: Shape = Shape::new_point(&vec);
	assert_eq!(shape.vertices.len() == 1, true);
	assert_eq!(shape.vertices[0].vec == vec, true);
	assert_eq!(shape.vertices[0].color == RED, true);
}

#[test]
fn line() {
	let vec: Vec3 = Vec3::new(1., 2., 3.);
	let vec2: Vec3 = Vec3::new(2., 3., 4.);
	let shape: Shape = Shape::new_line(&vec, &vec2);
	assert_eq!(shape.vertices.len() == 2, true);
	assert_eq!(shape.vertices[0].vec == vec, true);
	assert_eq!(shape.vertices[0].color == RED, true);
	assert_eq!(shape.vertices[1].vec == vec2, true);
	assert_eq!(shape.vertices[1].color == LIME, true);
}

#[test]
fn triangle() {
	let vec: Vec3 = Vec3::new(1., 2., 3.);
	let vec2: Vec3 = Vec3::new(2., 3., 4.);
	let vec3: Vec3 = Vec3::new(4., 5., 6.);
	let shape: Shape = Shape::new_triangle(&vec, &vec2, &vec3);
	assert_eq!(shape.vertices.len() == 3, true);
	assert_eq!(shape.vertices[0].vec == vec, true);
	assert_eq!(shape.vertices[0].color == RED, true);
	assert_eq!(shape.vertices[1].vec == vec2, true);
	assert_eq!(shape.vertices[1].color == WHITE, true);
	assert_eq!(shape.vertices[2].vec == vec3, true);
	assert_eq!(shape.vertices[2].color == BLUE, true);
}