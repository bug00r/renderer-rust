use objects::mesh::*;
use math::mat::mat3::*;
use math::vec::vec3::*;

pub struct Scene {
	pub meshes: Vec<Mesh>,
}

impl Scene {
	pub fn new( cnt_mesh: usize) -> Scene {
		Scene { meshes: Vec::with_capacity(cnt_mesh) }
	}
	
	pub fn transform(&mut self, mat: &Mat3) {
		for mesh in self.meshes.iter_mut() {
			mesh.transform(mat);
		}
	}
	
	pub fn scale(&mut self, x: f32, y: f32, z: f32) {
		for mesh in self.meshes.iter_mut() {
			mesh.scale(x, y, z);
		}
	}
	
	pub fn translate(&mut self, x: f32, y: f32, z: f32) {
		for mesh in self.meshes.iter_mut() {
			mesh.translate(x, y, z);
		}
	}
	
}

pub struct SceneBuilder {}

impl SceneBuilder {
	
	pub fn raster(linelen: f32) -> Scene {
		let mut scene: Scene = Scene::new(1);
		scene.meshes.push(MeshBuilder::raster(linelen));
		scene
	}
	
	pub fn triangle() -> Scene {
		let ( p1, p2, p3) = (Vec3::new(-0.5, -0.5, 0.5),
									    Vec3::new(0.5, -0.5, 0.5),
									    Vec3::new(0.0, 0.5, -0.5));

		let mut scene: Scene = Scene::new(1);
		scene.meshes.push(MeshBuilder::triangle(&p1,&p2,&p3));
		
		scene.meshes[0].compute_bbox();
		scene.meshes[0].color_by_bbox();
		
		scene
	}
	
	pub fn tree() -> Scene {
		let mut scene: Scene = Scene::new(1);
		
		let radius: f32 = 0.2;
		let cntelements: u32 = 20; 
		let height: f32 = 2.5; 
		let longs: u32 = 10;
		let scaleend: f32 = 0.85; //1.0 nothing scaled and scaleend = 0.0 max
		scene.meshes.push(MeshBuilder::path(radius, cntelements, height, longs, scaleend));
		
		scene
	}
	
	pub fn test_all() -> Scene {
		let mut scene: Scene = Scene::new(9);
		
		let mut center: Vec3 = Vec3::new_empty();
		
		scene.meshes.push(MeshBuilder::cube(&center, 0.2));
		scene.meshes[0].compute_bbox();
		scene.meshes[0].color_by_bbox();
		
		scene.meshes.push(MeshBuilder::sphere(0.2, 50, 50));
		scene.meshes[1].translate(0.0, 0.0, 0.5);
		scene.meshes[1].compute_bbox();
		scene.meshes[1].color_by_bbox();
		
		scene.meshes.push(MeshBuilder::cylinder(0.2, 0.3, 30, 30, true, true));
		scene.meshes[2].translate(0.0, 0.0, -0.5);
		scene.meshes[2].compute_bbox();
		scene.meshes[2].color_by_bbox();
		
		scene.meshes.push(MeshBuilder::cone(0.2, 0.3, 10, true));
		scene.meshes[3].translate(-0.5, 0.0, 0.0);
		scene.meshes[3].compute_bbox();
		scene.meshes[3].color_by_bbox();
		
		center.set( 0.5, 0.0, 0.0 );
		scene.meshes.push(MeshBuilder::block(&center, 0.2, 0.25, 0.3, 1, 2, 3));
		scene.meshes[4].compute_bbox();
		scene.meshes[4].color_by_bbox();
		
		center.set( 0.5, 0.0, 0.5 );
		scene.meshes.push(MeshBuilder::block(&center, 0.2, 0.3, 0.25, 1, 3, 2));
		scene.meshes[5].compute_bbox();
		scene.meshes[5].color_by_bbox();
		
		center.set( -0.5, 0.0, 0.5 );
		scene.meshes.push(MeshBuilder::block(&center, 0.25, 0.2, 0.3, 2, 1, 3));
		scene.meshes[6].compute_bbox();
		scene.meshes[6].color_by_bbox();
		
		center.set( 0.5, 0.0, -0.5 );
		scene.meshes.push(MeshBuilder::block(&center, 0.25, 0.3, 0.2, 2, 3, 1));
		scene.meshes[7].compute_bbox();
		scene.meshes[7].color_by_bbox();
		
		scene.meshes.push(MeshBuilder::sphere(0.2, 50, 50));
		
		let mut rotx_mat: Mat3 = Mat3::new_empty();
		rotx_mat.to_rotx(225.0);
		scene.meshes[8].transform(&rotx_mat);
		scene.meshes[8].translate(-0.5, 0.0, -0.5 );
		
		scene
	}
	
	pub fn point_raster() -> Scene {
		let mut scene: Scene = Scene::new(1);
		scene.meshes.push(MeshBuilder::point_raster());
		scene.meshes[0].compute_bbox();
		scene.meshes[0].color_by_bbox();
		
		scene
	}
	
	pub fn test_cube() -> Scene {
		let mut scene: Scene = Scene::new(1);
		scene.meshes.push(MeshBuilder::cube(&Vec3::new_empty(), 0.5));
		scene.meshes[0].compute_bbox();
		scene.meshes[0].color_by_bbox();
		
		let mut rotx_mat: Mat3 = Mat3::new_empty();
		rotx_mat.to_rotx(45.0);
		scene.meshes[0].transform(&rotx_mat);
		
		scene
	}
	
	pub fn texture_test() -> Scene {
		let center: Vec3 = Vec3::new_empty();
		
		let mut scene: Scene = Scene::new(1);

		scene.meshes.push(MeshBuilder::cube(&center, 0.25));
		let len: usize = scene.meshes[0].shapes.len();
		
		for i in 0..len {
			
			scene.meshes[0].shapes[i].tex_id = 0;
			scene.meshes[0].shapes[i].vertices[0].tex_coord.x = 0.;//0.;
			scene.meshes[0].shapes[i].vertices[0].tex_coord.y = 1.;//0.;
			scene.meshes[0].shapes[i].vertices[1].tex_coord.x = 1.;//1.;
			scene.meshes[0].shapes[i].vertices[1].tex_coord.y = 1.;//0.;
			scene.meshes[0].shapes[i].vertices[2].tex_coord.x = 0.;//0.;
			scene.meshes[0].shapes[i].vertices[2].tex_coord.y = 0.;//1.;
												
			let j = i + 1;
												
			scene.meshes[0].shapes[j].tex_id = 0;                     
			scene.meshes[0].shapes[j].vertices[0].tex_coord.x = 0.;//0.;
			scene.meshes[0].shapes[j].vertices[0].tex_coord.y = 0.;//1.;
			scene.meshes[0].shapes[j].vertices[1].tex_coord.x = 1.;//1.;
			scene.meshes[0].shapes[j].vertices[1].tex_coord.y = 1.;//0.;
			scene.meshes[0].shapes[j].vertices[2].tex_coord.x = 1.;//1.;
			scene.meshes[0].shapes[j].vertices[2].tex_coord.y = 0.;//1.;
		}
		
		scene
	}

	pub fn test() -> Scene {
		let mut scene: Scene = Scene::new(42);
		
		scene.meshes[40] = MeshBuilder::raster(6.);
		scene.meshes[41] = MeshBuilder::point_raster();
		scene.meshes[29] = MeshBuilder::sphere(0.2, 28, 28);
		scene.meshes[29].translate(-2., 0.0, -1.);
		scene.meshes[30] = MeshBuilder::sphere(0.1, 28, 28);
		scene.meshes[30].translate(-1.5, 0.0, -1.);
		scene.meshes[31] = MeshBuilder::sphere(0.3, 28, 28);
		scene.meshes[31].translate(-1., 0.0, -1.);
		scene.meshes[32] = MeshBuilder::sphere(0.5, 28, 28);
		scene.meshes[32].translate(-2., 0.0, 1.);
		scene.meshes[33] = MeshBuilder::sphere(0.2, 28, 28);
		scene.meshes[33].translate(-1.5, 0.0, 1.);
		scene.meshes[34] = MeshBuilder::sphere(0.8, 28, 28);
		scene.meshes[34].translate(-1., 0.0, 1.);
		scene.meshes[35] = MeshBuilder::sphere(0.5, 28, 28);
		scene.meshes[35].translate(1., 0.0, 1.5);
		scene.meshes[36] = MeshBuilder::sphere(0.4, 28, 28);
		scene.meshes[36].translate(1.5, 0.0, 1.5);
		scene.meshes[37] = MeshBuilder::sphere(0.5, 28, 28);
		scene.meshes[37].translate(2., 0.0, 1.5);
		scene.meshes[38] = MeshBuilder::sphere(0.4, 28, 28);
		scene.meshes[38].translate(1., 0.0, 2.);
		scene.meshes[39] = MeshBuilder::sphere(0.5, 10, 10);
		scene.meshes[39].translate(2., 0.0, 2.);
		
		scene.meshes[27] = MeshBuilder::sphere(0.5, 10, 10);
		scene.meshes[28] = MeshBuilder::sphere(0.5, 10, 10);
		scene.meshes[28].scale(1., 0.1, 1.);
		scene.meshes[27].translate(-1.5, 0.0, 0.);
		scene.meshes[28].translate(1.5, 0.0, 0.);

		let mut center: Vec3 = Vec3::new_empty();
		let sidelen: f32 = 0.3;
	
		scene.meshes[0] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, 0.0, 0.0);
		scene.meshes[1] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, 0.0, 0.0);
		scene.meshes[2] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		center.set(0.0, -0.5, 0.0);
		scene.meshes[3] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, -0.5, 0.0);
		scene.meshes[4] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, -0.5, 0.0);
		scene.meshes[5] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		center.set(0.0, 0.5, 0.0);
		scene.meshes[6] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, 0.5, 0.0);
		scene.meshes[7] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, 0.5, 0.0);
		scene.meshes[8] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		center.set(0.0, 0.0, 0.5);
		scene.meshes[9] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, 0.0, 0.5);
		scene.meshes[10] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, 0.0, 0.5);
		scene.meshes[11] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		center.set(0.0, -0.5, 0.5);
		scene.meshes[12] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, -0.5, 0.5);
		scene.meshes[13] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, -0.5, 0.5);
		scene.meshes[14] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		center.set(0.0, 0.5, 0.5);
		scene.meshes[15] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, 0.5, 0.5);
		scene.meshes[16] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, 0.5, 0.5);
		scene.meshes[17] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		center.set(0.0, 0.0, -0.5);
		scene.meshes[18] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, 0.0, -0.5);
		scene.meshes[19] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, 0.0, -0.5);
		scene.meshes[20] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		center.set(0.0, -0.5, -0.5);
		scene.meshes[21] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, -0.5, -0.5);
		scene.meshes[22] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, -0.5, -0.5);
		scene.meshes[23] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		center.set(0.0, 0.5, -0.5);
		scene.meshes[24] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(-0.5, 0.5, -0.5);
		scene.meshes[25] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		center.set(0.5, 0.5, -0.5);
		scene.meshes[26] = MeshBuilder::cube(&center, sidelen);//create_raster(2.);
		
		scene
	}
}