use rasterizer::*;
use color::*;
use math::vec::vec3::*;
use math::mat::mat3::*;
use objects::mesh::*;
use objects::scene::*;
use objects::shape::*;

fn create_renderer(	width: i32, height: i32,
					from: &Vec3, to: &Vec3, 
					zoom: f32, 
					left: f32,right: f32, 
					top: f32, bottom: f32, 
					near: f32,far: f32,
					bgcolor: &Color,
					samplestep: u32) -> Renderer{
	let mut renderer: Renderer = Renderer::new(width, height, bgcolor, samplestep);
	renderer.camera.to_ortho(from, to, zoom * left, zoom * right, zoom * top, zoom * bottom, near, far);
	renderer
}

fn create_test_base_renderer(samplestep: u32) -> Renderer {
	let from: Vec3 = Vec3::new(0.0, 0.0, 1.0);
	let to: Vec3 = Vec3::new_empty();
	create_renderer(512 /*width*/, 512/*height*/,
					&from, &to, 
					0.5 /* zoom */,
					-2.0 /* left */, 2.0 /*right*/, 
					2.0 /* top */, -2.0 /*bottom*/, 
					1.0 /* near */, 5.0 /*far*/,
					&BLACK,
					samplestep)
}

fn create_renderer_perspective(	width: i32, height: i32,
					from: &Vec3, to: &Vec3, 
					zoom: f32, 
					left: f32,right: f32, 
					top: f32, bottom: f32, 
					near: f32,far: f32,
					bgcolor: &Color,
					samplestep: u32) -> Renderer{
	let mut renderer: Renderer = Renderer::new(width, height, bgcolor, samplestep);
	renderer.camera.to_perspective(from, to, zoom * left, zoom * right, zoom * top, zoom * bottom, near, far);
	renderer
}

fn create_test_base_renderer_perspective(samplestep: u32) -> Renderer {
	let from: Vec3 = Vec3::new(0.0, 0.0, 2.5);
	let to: Vec3 = Vec3::new_empty();
	create_renderer_perspective(512 /*width*/, 512/*height*/,
					&from, &to, 
					1.0 /* zoom */,
					-2.0 /* left */, 2.0 /*right*/, 
					2.0 /* top */, -2.0 /*bottom*/, 
					1.0 /* near */, 5.0 /*far*/,
					&BLACK,
					samplestep)
}


#[test]
#[cfg(any(feature = "all", all(feature = "point", feature = "orthogonal", feature = "nomsaa")))]
fn point_ortho() {	
	let p: Vec3 = Vec3{ x:0.0,y: 0.0,z: 0.001 };
	let points: Mesh = MeshBuilder::point(&p);
	
	let mut renderer: Renderer = create_test_base_renderer(1);
	renderer.render_mesh(&points);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/point_ortho.ppm");
		renderer.write_zb_ppm("target/point_ortho_z.ppm");
	}
}

#[test]
#[cfg(any(feature = "all", all(feature = "point", feature = "perspective", feature = "nomsaa")))]
fn point_perspective() {	
	let p: Vec3 = Vec3{ x:0.0,y: 0.0,z: 0.001 };
	let points: Mesh = MeshBuilder::point(&p);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(1);
	renderer.render_mesh(&points);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/point_perspective.ppm");
		renderer.write_zb_ppm("target/point_perspective_z.ppm");
	}
}

#[test]
#[cfg(any(feature = "all", all(feature = "point", feature = "orthogonal", feature = "msaa2x2")))]
fn point_ortho_msaa2x2() {	
	let p: Vec3 = Vec3{ x:0.0,y: 0.0,z: 0.001 };
	let points: Mesh = MeshBuilder::point(&p);
	
	let mut renderer: Renderer = create_test_base_renderer(2);
	renderer.render_mesh(&points);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/point_ortho_msaa2x2.ppm");
		renderer.write_zb_ppm("target/point_ortho_msaa2x2_z.ppm");
	}
}

#[test]
#[cfg(any(feature = "all", all(feature = "point", feature = "perspective", feature = "msaa2x2")))]
fn point_perspective_msaa2x2() {	
	let p: Vec3 = Vec3{ x:0.0,y: 0.0,z: 0.001 };
	let points: Mesh = MeshBuilder::point(&p);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(2);
	renderer.render_mesh(&points);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/point_perspective_msaa2x2.ppm");
		renderer.write_zb_ppm("target/point_perspective_msaa2x2_z.ppm");
	}
}

#[test]
#[cfg(any(feature = "all", all(feature = "point", feature = "orthogonal", feature = "msaa4x4")))]
fn point_ortho_msaa4x4() {	
	let p: Vec3 = Vec3{ x:0.0,y: 0.0,z: 0.001 };
	let points: Mesh = MeshBuilder::point(&p);
	
	let mut renderer: Renderer = create_test_base_renderer(4);
	renderer.render_mesh(&points);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/point_ortho_msaa4x4.ppm");
		renderer.write_zb_ppm("target/point_ortho_msaa4x4_z.ppm");
	}
}

#[test]
#[cfg(any(feature = "all", all(feature = "point", feature = "perspective", feature = "msaa4x4")))]
fn point_perspective_msaa4x4() {	
	let p: Vec3 = Vec3{ x:0.0,y: 0.0,z: 0.001 };
	let points: Mesh = MeshBuilder::point(&p);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(4);
	renderer.render_mesh(&points);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/point_perspective_msaa4x4.ppm");
		renderer.write_zb_ppm("target/point_perspective_msaa4x4_z.ppm");
	}
}

#[test]
#[cfg(any(feature = "all", all(feature = "line", feature = "orthogonal", feature = "nomsaa")))]
fn test_render_lines_ortho_nomsaa() {
	
	let p : Vec3=  Vec3::new( -1.0, -1.0, 0.0 );
	let p2: Vec3 = Vec3::new( -0.8, 1.0, 0.0  );
	let mut line: Mesh = MeshBuilder::line(&p, &p2);
	
	let mut renderer: Renderer = create_test_base_renderer(1);
	
	let mut step: f32 = 0.0;
	for i in 0..20 {
		line.shapes[0].vertices[1].vec.x += step;
		line.shapes[0].vertices[1].vec.y -= step;
		line.shapes[0].vertices[1].vec.z += step;
		renderer.render_mesh(&line);
		step += 0.05;
	}
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/line_orthogonal_nomsaa.ppm");
		renderer.write_zb_ppm("target/line_orthogonal_nomsaa_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "line", feature = "perspective", feature = "nomsaa")))]
fn test_render_lines_perspective_nomsaa() {
	
	let p : Vec3=  Vec3::new( -1.0, -1.0, 0.0 );
	let p2: Vec3 = Vec3::new( -0.8, 1.0, 0.0  );
	let mut line: Mesh = MeshBuilder::line(&p, &p2);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(1);

	let mut step: f32 = 0.0;
	for i in 0..20 {
		line.shapes[0].vertices[1].vec.x += step;
		line.shapes[0].vertices[1].vec.y -= step;
		line.shapes[0].vertices[1].vec.z += step;
		renderer.render_mesh(&line);
		step += 0.05;
	}
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/line_perspective_nomsaa.ppm");
		renderer.write_zb_ppm("target/line_perspective_nomsaa_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "line", feature = "orthogonal", feature = "msaa2x2")))]
fn test_render_lines_ortho_msaa2x2() {
	
	let p : Vec3=  Vec3::new( -1.0, -1.0, 0.0 );
	let p2: Vec3 = Vec3::new( -0.8, 1.0, 0.0  );
	let mut line: Mesh = MeshBuilder::line(&p, &p2);
	
	let mut renderer: Renderer = create_test_base_renderer(2);
	
	let mut step: f32 = 0.0;
	for i in 0..20 {
		line.shapes[0].vertices[1].vec.x += step;
		line.shapes[0].vertices[1].vec.y -= step;
		line.shapes[0].vertices[1].vec.z += step;
		renderer.render_mesh(&line);
		step += 0.05;
	}
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/line_orthogonal_nomsaa.ppm");
		renderer.write_zb_ppm("target/line_orthogonal_nomsaa_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "line", feature = "perspective", feature = "msaa2x2")))]
fn test_render_lines_perspective_msaa2x2() {
	
	let p : Vec3=  Vec3::new( -1.0, -1.0, 0.0 );
	let p2: Vec3 = Vec3::new( -0.8, 1.0, 0.0  );
	let mut line: Mesh = MeshBuilder::line(&p, &p2);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(2);

	let mut step: f32 = 0.0;
	for i in 0..20 {
		line.shapes[0].vertices[1].vec.x += step;
		line.shapes[0].vertices[1].vec.y -= step;
		line.shapes[0].vertices[1].vec.z += step;
		renderer.render_mesh(&line);
		step += 0.05;
	}
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/line_perspective_msaa2x2.ppm");
		renderer.write_zb_ppm("target/line_perspective_msaa2x2_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "line", feature = "orthogonal", feature = "msaa4x4")))]
fn test_render_lines_ortho_msaa4x4() {
	
	let p : Vec3=  Vec3::new( -1.0, -1.0, 0.0 );
	let p2: Vec3 = Vec3::new( -0.8, 1.0, 0.0  );
	let mut line: Mesh = MeshBuilder::line(&p, &p2);
	
	let mut renderer: Renderer = create_test_base_renderer(4);
	
	let mut step: f32 = 0.0;
	for i in 0..20 {
		line.shapes[0].vertices[1].vec.x += step;
		line.shapes[0].vertices[1].vec.y -= step;
		line.shapes[0].vertices[1].vec.z += step;
		renderer.render_mesh(&line);
		step += 0.05;
	}
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/line_orthogonal_msaa4x4.ppm");
		renderer.write_zb_ppm("target/line_orthogonal_msaa4x4_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "line", feature = "perspective", feature = "msaa4x4")))]
fn test_render_lines_perspective_msaa4x4() {
	
	let p : Vec3=  Vec3::new( -1.0, -1.0, 0.0 );
	let p2: Vec3 = Vec3::new( -0.8, 1.0, 0.0  );
	let mut line: Mesh = MeshBuilder::line(&p, &p2);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(4);

	let mut step: f32 = 0.0;
	for i in 0..20 {
		line.shapes[0].vertices[1].vec.x += step;
		line.shapes[0].vertices[1].vec.y -= step;
		line.shapes[0].vertices[1].vec.z += step;
		renderer.render_mesh(&line);
		step += 0.05;
	}
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/line_perspective_msaa4x4.ppm");
		renderer.write_zb_ppm("target/line_perspective_msaa4x4_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "triangle", feature = "orthogonal", feature = "nomsaa")))]
fn test_render_triangle_orthogonal_nomsaa() {
	
	let p1: Vec3 = Vec3{x: -0.75, y: -0.75, z: 0.75 };
	let p2: Vec3 = Vec3{x: 0.75,  y: -0.55, z: 0.25 };
	let p3: Vec3 = Vec3{x: 0.05,  y: 0.75,  z: -0.75 };
	let mut triangle: Mesh = MeshBuilder::triangle(&p1, &p2, &p3);
	
	let mut renderer: Renderer = create_test_base_renderer(1);

	renderer.render_mesh(&triangle);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/triangle_orthogonal_nomsaa.ppm");
		renderer.write_zb_ppm("target/triangle_orthogonal_nomsaa_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "triangle", feature = "orthogonal", feature = "msaa2x2")))]
fn test_render_triangle_orthogonal_msaa2x2() {
	
	let p1: Vec3 = Vec3{x: -0.75, y: -0.75, z: 0.75 };
	let p2: Vec3 = Vec3{x: 0.75,  y: -0.55, z: 0.25 };
	let p3: Vec3 = Vec3{x: 0.05,  y: 0.75,  z: -0.75 };
	let mut triangle: Mesh = MeshBuilder::triangle(&p1, &p2, &p3);
	
	let mut renderer: Renderer = create_test_base_renderer(2);

	renderer.render_mesh(&triangle);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/triangle_orthogonal_msaa2x2.ppm");
		renderer.write_zb_ppm("target/triangle_orthogonal_msaa2x2_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "triangle", feature = "orthogonal", feature = "msaa4x4")))]
fn test_render_triangle_orthogonal_msaa4x4() {
	
	let p1: Vec3 = Vec3{x: -0.75, y: -0.75, z: 0.75 };
	let p2: Vec3 = Vec3{x: 0.75,  y: -0.55, z: 0.25 };
	let p3: Vec3 = Vec3{x: 0.05,  y: 0.75,  z: -0.75 };
	let mut triangle: Mesh = MeshBuilder::triangle(&p1, &p2, &p3);
	
	let mut renderer: Renderer = create_test_base_renderer(4);

	renderer.render_mesh(&triangle);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/triangle_orthogonal_msaa4x4.ppm");
		renderer.write_zb_ppm("target/triangle_orthogonal_msaa4x4_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "triangle", feature = "perspective", feature = "nomsaa")))]
fn test_render_triangle_perspective_nomsaa() {
	
	let p1: Vec3 = Vec3{x: -0.75, y: -0.75, z: 0.55 };
	let p2: Vec3 = Vec3{x: 0.75,  y: -0.55, z: 0.25 };
	let p3: Vec3 = Vec3{x: 0.05,  y: 0.75,  z: -0.75 };
	let mut triangle: Mesh = MeshBuilder::triangle(&p1, &p2, &p3);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(1);

	renderer.render_mesh(&triangle);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/triangle_perspective_nomsaa.ppm");
		renderer.write_zb_ppm("target/triangle_perspective_nomsaa_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "triangle", feature = "perspective", feature = "msaa2x2")))]
fn test_render_triangle_perspective_msaa2x2() {
	
	let p1: Vec3 = Vec3{x: -0.75, y: -0.75, z: 0.75 };
	let p2: Vec3 = Vec3{x: 0.75,  y: -0.55, z: 0.25 };
	let p3: Vec3 = Vec3{x: 0.05,  y: 0.75,  z: -0.75 };
	let mut triangle: Mesh = MeshBuilder::triangle(&p1, &p2, &p3);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(2);

	renderer.render_mesh(&triangle);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/triangle_perspective_msaa2x2.ppm");
		renderer.write_zb_ppm("target/triangle_perspective_msaa2x2_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "triangle", feature = "perspective", feature = "msaa4x4")))]
fn test_render_triangle_perspective_msaa4x4() {
	
	let p1: Vec3 = Vec3{x: -0.75, y: -0.75, z: 0.75 };
	let p2: Vec3 = Vec3{x: 0.75,  y: -0.55, z: 0.25 };
	let p3: Vec3 = Vec3{x: 0.05,  y: 0.75,  z: -0.75 };
	let mut triangle: Mesh = MeshBuilder::triangle(&p1, &p2, &p3);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(4);

	renderer.render_mesh(&triangle);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/triangle_perspective_msaa4x4.ppm");
		renderer.write_zb_ppm("target/triangle_perspective_msaa4x4_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "quad", feature = "orthogonal", feature = "nomsaa")))]
fn test_render_quad_orthogonal_nomsaa() {
	
	let p1: Vec3 = Vec3{ x:-0.75,y: -0.75,z:  0.75 };
	let p2: Vec3 = Vec3{ x:0.75, y:-0.75, z: 0.75 };
	let p3: Vec3 = Vec3{ x:-0.75,y: 0.75, z: 0.75 };
	let p4: Vec3 = Vec3{ x:0.75, y:0.75,  z: 0.75 };
	let mut quad: Mesh = MeshBuilder::quad(&p1, &p2, &p3, &p4);
	
	let mut renderer: Renderer = create_test_base_renderer(1);

	renderer.render_mesh(&quad);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/quad_orthogonal_nomsaa.ppm");
		renderer.write_zb_ppm("target/quad_orthogonal_nomsaa_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "quad", feature = "orthogonal", feature = "msaa2x2")))]
fn test_render_quad_orthogonal_msaa2x2() {
	
	let p1: Vec3 = Vec3{ x:-0.75,y: -0.75,z:  0.75 };
	let p2: Vec3 = Vec3{ x:0.75, y:-0.75, z: 0.75 };
	let p3: Vec3 = Vec3{ x:-0.75,y: 0.75, z: 0.75 };
	let p4: Vec3 = Vec3{ x:0.75, y:0.75,  z: 0.75 };
	let mut quad: Mesh = MeshBuilder::quad(&p1, &p2, &p3, &p4);
	
	let mut renderer: Renderer = create_test_base_renderer(2);

	renderer.render_mesh(&quad);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/quad_orthogonal_msaa2x2.ppm");
		renderer.write_zb_ppm("target/quad_orthogonal_msaa2x2_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "quad", feature = "orthogonal", feature = "msaa4x4")))]
fn test_render_quad_orthogonal_msaa4x4() {
	
	let p1: Vec3 = Vec3{ x:-0.75,y: -0.75,z:  0.75 };
	let p2: Vec3 = Vec3{ x:0.75, y:-0.75, z: 0.75 };
	let p3: Vec3 = Vec3{ x:-0.75,y: 0.75, z: 0.75 };
	let p4: Vec3 = Vec3{ x:0.75, y:0.75,  z: 0.75 };
	let mut quad: Mesh = MeshBuilder::quad(&p1, &p2, &p3, &p4);
	
	let mut renderer: Renderer = create_test_base_renderer(4);

	renderer.render_mesh(&quad);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/quad_orthogonal_msaa4x4.ppm");
		renderer.write_zb_ppm("target/quad_orthogonal_msaa4x4_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "quad", feature = "perspective", feature = "nomsaa")))]
fn test_render_quad_perspective_nomsaa() {
	
	let p1: Vec3 = Vec3{ x:-0.75,y: -0.75,z:  0.75 };
	let p2: Vec3 = Vec3{ x:0.75, y:-0.75, z: 0.75 };
	let p3: Vec3 = Vec3{ x:-0.75,y: 0.75, z: 0.75 };
	let p4: Vec3 = Vec3{ x:0.75, y:0.75,  z: 0.75 };
	let mut quad: Mesh = MeshBuilder::quad(&p1, &p2, &p3, &p4);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(1);

	renderer.render_mesh(&quad);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/quad_perspective_nomsaa.ppm");
		renderer.write_zb_ppm("target/quad_perspective_nomsaa_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "quad", feature = "perspective", feature = "msaa2x2")))]
fn test_render_quad_perspective_msaa2x2() {
	
	let p1: Vec3 = Vec3{ x:-0.75,y: -0.75,z:  0.75 };
	let p2: Vec3 = Vec3{ x:0.75, y:-0.75, z: 0.75 };
	let p3: Vec3 = Vec3{ x:-0.75,y: 0.75, z: 0.75 };
	let p4: Vec3 = Vec3{ x:0.75, y:0.75,  z: 0.75 };
	let mut quad: Mesh = MeshBuilder::quad(&p1, &p2, &p3, &p4);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(2);

	renderer.render_mesh(&quad);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/quad_perspective_msaa2x2.ppm");
		renderer.write_zb_ppm("target/quad_perspective_msaa2x2_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "quad", feature = "perspective", feature = "msaa4x4")))]
fn test_render_quad_perspective_msaa4x4() {
	
	let p1: Vec3 = Vec3{ x:-0.75,y: -0.75,z:  0.75 };
	let p2: Vec3 = Vec3{ x:0.75, y:-0.75, z: 0.75 };
	let p3: Vec3 = Vec3{ x:-0.75,y: 0.75, z: 0.75 };
	let p4: Vec3 = Vec3{ x:0.75, y:0.75,  z: 0.75 };
	let mut quad: Mesh = MeshBuilder::quad(&p1, &p2, &p3, &p4);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(4);

	renderer.render_mesh(&quad);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/quad_perspective_msaa4x4.ppm");
		renderer.write_zb_ppm("target/quad_perspective_msaa4x4_z.ppm");
	}
	
}

#[test]
#[cfg(any(feature = "all", all(feature = "cube", feature = "orthogonal", feature = "nomsaa")))]
fn test_render_cube_orthogonal_nomsaa() {
	
	let center: Vec3 = Vec3{x: 0., y: 0., z: 0. };
	let mut cube: Mesh = MeshBuilder::cube(&center, 1.0);
	
	let mut renderer: Renderer = create_test_base_renderer(1);

	renderer.render_mesh(&cube);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/cube_orthogonal_nomsaa.ppm");
		renderer.write_zb_ppm("target/cube_orthogonal_nomsaa_z.ppm");
	}

	let mut rotmat: Mat3 = Mat3::new_empty();
	rotmat.to_rotz(45.); 
	let mut rotx: Mat3 = Mat3::new_empty();
	rotx.to_rotx(45.); 
	rotmat *= &rotx;

	cube.transform(&rotmat);
	
	renderer.clear_frame();
	
	renderer.render_mesh(&cube);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/cube_rot45_orthogonal_nomsaa.ppm");
		renderer.write_zb_ppm("target/cube_rot45_orthogonal_nomsaa_z.ppm");
	}
	
	rotmat.to_rotz(65.); 
	rotx.to_rotx(45.);
	rotmat *= &rotx;

	cube.transform(&rotmat);
	
	renderer.clear_frame();
	renderer.render_mesh(&cube);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/cube_rot65_orthogonal_nomsaa.ppm");
		renderer.write_zb_ppm("target/cube_rot65_orthogonal_nomsaa_z.ppm");
	}
}

#[test]
#[cfg(any(feature = "all", all(feature = "cube", feature = "perspective", feature = "nomsaa")))]
fn test_render_cube_perspective_nomsaa() {
	
	let center: Vec3 = Vec3{x: 0., y: 0., z: 0. };
	let mut cube: Mesh = MeshBuilder::cube(&center, 0.3);
	
	let mut renderer: Renderer = create_test_base_renderer_perspective(1);

	renderer.render_mesh(&cube);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/cube_perspective_nomsaa.ppm");
		renderer.write_zb_ppm("target/cube_perspective_nomsaa_z.ppm");
	}

	let mut rotmat: Mat3 = Mat3::new_empty();
	rotmat.to_rotz(45.); 
	let mut rotx: Mat3 = Mat3::new_empty();
	rotx.to_rotx(45.); 
	rotmat *= &rotx;

	cube.transform(&rotmat);
	
	renderer.clear_frame();
	
	renderer.render_mesh(&cube);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/cube_rot45_perspective_nomsaa.ppm");
		renderer.write_zb_ppm("target/cube_rot45_perspective_nomsaa_z.ppm");
	}
	
	rotmat.to_rotz(65.); 
	rotx.to_rotx(45.);
	rotmat *= &rotx;

	cube.transform(&rotmat);
	
	renderer.clear_frame();
	renderer.render_mesh(&cube);
	
	if cfg!(feature = "output") {
		renderer.write_fb_ppm("target/cube_rot65_perspective_nomsaa.ppm");
		renderer.write_zb_ppm("target/cube_rot65_perspective_nomsaa_z.ppm");
	}
}
