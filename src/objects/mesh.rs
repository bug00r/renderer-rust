use math::vec::vec3::*;
use math::mat::mat3::*;
use objects::shape::*;
use std::f32::{MIN, MAX};
use math::utils::{interpolate_lin, rand_path_deg};
use color::*;

#[derive(Clone)]
pub struct BoundingBox {
	pub min: Vec3,
	pub max: Vec3,
	pub created: bool,
}

#[derive(Clone)]
pub struct Mesh {
	pub bbox: BoundingBox,
	pub shapes: Vec<Shape>,
}

impl BoundingBox {
	pub fn new(min: &Vec3, max: &Vec3) -> BoundingBox {
		BoundingBox{ min: min.clone(), max: max.clone(), created: false }
	}

	pub fn new_empty() -> BoundingBox {
		BoundingBox{ min: Vec3::new(MAX, MAX, MAX), max: Vec3::new(MIN, MIN, MIN), created: false }
	}
	
	pub fn set(&mut self, min: &Vec3, max: &Vec3) {
		self.min.set_from(min);
		self.max.set_from(max);
		self.created = false;
	}
	
	pub fn set_from(&mut self, bbox: &BoundingBox) {
		self.min.set_from(&bbox.min);
		self.max.set_from(&bbox.max);
		self.created = bbox.created;
	}
	
	pub fn set_to(&self, bbox: &mut BoundingBox) {
		bbox.min.set_from(&self.min);
		bbox.max.set_from(&self.max);
		bbox.created = self.created;
	}
	
	pub fn clear(&mut self) {
		self.min.set(MAX, MAX, MAX);
		self.max.set(MIN, MIN, MIN);
		self.created = false;
	}
}

impl Mesh {

	pub fn new( cnt_mesh: usize) -> Mesh {
		Mesh { bbox: BoundingBox::new_empty(), shapes: Vec::with_capacity(cnt_mesh) }
	}
	
	pub fn new_init(bbox: &BoundingBox, shapes: &Vec<Shape>) -> Mesh {
		Mesh { bbox: bbox.clone() , shapes: shapes.to_vec() }
	}

	pub fn transform(&mut self, mat: &Mat3) {
		for shape in self.shapes.iter_mut() {
			shape.transform(mat);
		}
	}
	
	pub fn scale(&mut self, x: f32, y: f32, z: f32) {
		for shape in self.shapes.iter_mut() {
			shape.scale(x, y, z);
		}
	}
	
	pub fn translate(&mut self, x: f32, y: f32, z: f32) {
		for shape in self.shapes.iter_mut() {
			shape.translate(x, y, z);
		}
	}
	
	pub fn compute_bbox(&mut self) {
		for shape in self.shapes.iter_mut() {
			for vertex in shape.vertices.iter() {
				self.bbox.min.x = self.bbox.min.x.min(vertex.vec.x);
				self.bbox.min.y = self.bbox.min.y.min(vertex.vec.y);
				self.bbox.min.z = self.bbox.min.z.min(vertex.vec.z);
				self.bbox.max.x = self.bbox.max.x.max(vertex.vec.x);
				self.bbox.max.y = self.bbox.max.y.max(vertex.vec.y);
				self.bbox.max.z = self.bbox.max.z.max(vertex.vec.z);
			}
		}
		self.bbox.created = true;
	}
	
	pub fn color_by_bbox(&mut self) {
		for shape in self.shapes.iter_mut() {
			for vertex in shape.vertices.iter_mut() {
				vertex.color.r = interpolate_lin(vertex.vec.x, self.bbox.min.x, 0., self.bbox.max.x, 1.);
				vertex.color.g = interpolate_lin(vertex.vec.y, self.bbox.min.y, 0., self.bbox.max.y, 1.);
				vertex.color.b = interpolate_lin(vertex.vec.z, self.bbox.min.z, 0., self.bbox.max.z, 1.);
			}
		}
	}	
}


pub struct MeshBuilder {}

impl MeshBuilder {
	pub fn point(p: &Vec3) -> Mesh {
		let mut point: Mesh = Mesh::new(1);
		point.shapes.push(Shape::new_point(p));
		point
	}
	pub fn line(p: &Vec3, p2: &Vec3) -> Mesh {
		let mut line: Mesh = Mesh::new(1);
		line.shapes.push(Shape::new_line(p, p2));
		line
	}
	pub fn triangle(p: &Vec3, p2: &Vec3, p3: &Vec3) -> Mesh {
		let mut triangle: Mesh = Mesh::new(1);
		triangle.shapes.push(Shape::new_triangle(p, p2, p3));
		triangle
	}
	pub fn quad(lb: &Vec3, rb: &Vec3, lt: &Vec3,rt: &Vec3) -> Mesh {
		let mut quad: Mesh = Mesh::new(2);
		quad.shapes.push(Shape::new_triangle(lb, rb, lt));
		quad.shapes.push(Shape::new_triangle(lt, rb, rt));
		quad.shapes[1].vertices[2].color.set_from_rgb(1., 1., 1.);
		quad
	}
	pub fn cube(center: &Vec3, sidelen: f32) -> Mesh {
		MeshBuilder::block(center, sidelen, sidelen, sidelen, 1, 1, 1)
	}

	pub fn block(center: &Vec3, width: f32, height:f32, depth: f32, cntx: u32, cnty: u32, cntz: u32) -> Mesh {
		let mut block = Mesh::new(((cntx * cnty * 4) + ( cntz * cnty * 4 ) + ( cntz * cntx * 4 )) as usize);
		let hw: f32 = width  * 0.5;
		let hh: f32 = height * 0.5;
		let hd: f32 = depth  * 0.5;
		let wseg: f32 = width / cntx as f32;
		let hseg: f32 = height / cnty as f32;
		let dseg: f32 = depth / cntz as f32;
		let mut curx: u32;
		let mut cury: u32 = 0;
		let mut curz: u32;
		
		let (mut p1, mut p2, mut p3, mut p4) = (Vec3::new_empty(), Vec3::new_empty(), 
											    Vec3::new_empty(), Vec3::new_empty());
		
		//every side segment could be mirrored front to back, left to right, top to bottom
		//front + back
		let mut start: Vec3 = Vec3::new(center.x - hw, center.y - hh, center.z + hd);
		let mut temp2: f32 = start.z - depth; //used for back z
		
		loop {
			curx = 0;
			loop {
			
				p1.set(start.x +  (curx as f32 * wseg)   ,  start.y + (cury as f32 * hseg)    , start.z);
				p2.set(start.x + ((curx+1) as f32 * wseg),  start.y + (cury as f32 * hseg)    , start.z);
				p3.set(start.x +  (curx as f32 * wseg)	 ,  start.y + ((cury+1) as f32 * hseg), start.z);
			    p4.set(start.x + ((curx+1) as f32 * wseg),  start.y + ((cury+1) as f32 * hseg), start.z);
			
				block.shapes.push(Shape::new_triangle(&p1, &p2, &p3));
				block.shapes.push(Shape::new_triangle(&p3, &p2, &p4));
			
				p1.z = temp2; p2.z = temp2; p3.z = temp2; p4.z = temp2; 
				
				block.shapes.push(Shape::new_triangle(&p2, &p1, &p4));
				block.shapes.push(Shape::new_triangle(&p4, &p1, &p3));
				
				curx += 1;
				if curx == cntx { break; }
			}	
			
			cury += 1;
			if cury == cnty { break; }
		}
		
		//left + right
		start.set(center.x - hw, center.y - hh, center.z - hd);
		temp2 = start.x + width; //used for back z
		cury = 0;
		
		loop {
			curz = 0;
			loop {
			
				p1.set( start.x ,  start.y + (cury as f32 * hseg)    , start.z + (curz as f32 * dseg));
				p2.set( start.x ,  start.y + (cury as f32 * hseg)    , start.z + ((curz+1) as f32 * dseg));
				p3.set( start.x ,  start.y + ((cury+1) as f32 * hseg), start.z + (curz as f32 * dseg));
			    p4.set( start.x ,  start.y + ((cury+1) as f32 * hseg), start.z + ((curz+1) as f32 * dseg));
			
				block.shapes.push(Shape::new_triangle(&p1, &p2, &p3));
				block.shapes.push(Shape::new_triangle(&p3, &p2, &p4));
			
				p1.x = temp2; p2.x = temp2; p3.x = temp2; p4.x = temp2;
				
				block.shapes.push(Shape::new_triangle(&p2, &p1, &p4));
				block.shapes.push(Shape::new_triangle(&p4, &p1, &p3));
				
				curz += 1;
				if curz == cntz { break; }
			}	
			
			cury += 1;
			if cury == cnty { break; }
		}

		//top + bottom
		start.set(center.x - hw, center.y + hh, center.z + hd);
		temp2 = start.y - height; //used bottom yz
		curz = 0;
		
		loop {
			curx = 0;
			loop {
			
				p1.set( start.x + (curx as f32 * wseg),    start.y, start.z - (curz as f32 * dseg));
				p2.set( start.x + ((curx+1) as f32 * wseg),start.y, start.z - (curz as f32 * dseg));
				p3.set( start.x + (curx as f32 * wseg),    start.y, start.z - ((curz+1) as f32 * dseg));
			    p4.set( start.x + ((curx+1) as f32 * wseg),start.y, start.z - ((curz+1) as f32 * dseg));
			
				block.shapes.push(Shape::new_triangle(&p1, &p2, &p3));
				block.shapes.push(Shape::new_triangle(&p3, &p2, &p4));
			
				p1.y = temp2; p2.y = temp2; p3.y = temp2; p4.y = temp2;
				
				block.shapes.push(Shape::new_triangle(&p2, &p1, &p4));
				block.shapes.push(Shape::new_triangle(&p4, &p1, &p3));
				
				curx += 1;
				if curx == cntx { break; }
			}	
			
			curz += 1;
			if curz == cntz { break; }
		}
		
		block
	}
	
	pub fn raster(linelen: f32) -> Mesh {
		let lines = 5;
		let lineslimit = lines + 1;
		let linestep: f32 = 0.5;
		let mut raster: Mesh = Mesh::new(22);

		let (mut start, mut end) = (Vec3::new_empty(), Vec3::new_empty());
		let mut curshape: Shape;
		for x in -lines..lineslimit {
			start.set(linestep*x as f32 , 0.0, -linelen as f32);
			end.set(linestep*x as f32 , 0.0 , linelen as f32);
			curshape = Shape::new_line(&start, &end);
			curshape.set_color(&GREY);
			raster.shapes.push(curshape);
		}
		for z in -lines..lineslimit {
			start.set( -linelen as f32  , 0.0, linestep*z as f32);
			end.set( linelen as f32 , 0.0,  linestep*z as f32);
			curshape = Shape::new_line(&start, &end);
			curshape.set_color(&GREY);
			raster.shapes.push(curshape);
		}
		raster
	}
	
	pub fn point_raster() -> Mesh {
		let pdiff: f32 = 0.08;
		let start  : i32 = 10;
		let startx : i32 = 10;
		let mut raster: Mesh = Mesh::new(((2*startx as usize)+1)*((2*startx as usize)+1)*((2*start as usize)+1));

		let mut curvec = Vec3::new_empty();
		
		for x in -startx..=startx {
			for y in -start..=start {
				for z in -startx..=startx {
					curvec.set(x as f32*pdiff, y as f32*pdiff, z as f32*pdiff);
					raster.shapes.push(Shape::new_point(&curvec));
				}
			}
		}
		
		raster
	}
	
	pub fn sphere(radius: f32, longs: u32, lats: u32) -> Mesh {
		let mut sphere = Mesh::new((longs * lats * 2) as usize);
		
		let degreelong: f32 = 180. / longs as f32;
		let degreelats: f32 = 360. / lats as f32;
		
		let mut curlongdeg: f32 = 90.;
		let mut curlatdeg: f32;
		
		let startvec: Vec3 = Vec3::new(radius, 0., 0.);
		
		let (mut p1, mut p2, mut p3, mut p4) = (Vec3::new_empty(), Vec3::new_empty(), 
											    Vec3::new_empty(), Vec3::new_empty());
		
		let longcolorstep: f32 = 1. / longs as f32;
		let mut curlong: u32 = 0;
		let mut color: Color = BLACK.clone();
		
		let mut z_rot_matrix: Mat3	 = Mat3::new_empty();
		let mut z_rot_matrix_2: Mat3 = Mat3::new_empty();
		let mut y_rot_matrix: Mat3	 = Mat3::new_empty();
		let mut y_rot_matrix_2: Mat3 = Mat3::new_empty();
		
		loop {
			
			let nextlongdegree: f32 = curlongdeg - degreelong;
			let mut curlat: u32 = 0;
			
			z_rot_matrix.to_rotz(curlongdeg);
			z_rot_matrix_2.to_rotz(nextlongdegree);
			curlatdeg = 0.;
			
			loop {
			
				let nextlatdegree = curlatdeg + degreelats;
                
				y_rot_matrix.to_roty(curlatdeg);
				y_rot_matrix_2.to_roty(nextlatdegree);
                
				p1.set_from(&startvec);
				p1 *= &y_rot_matrix;
				p1 *= &z_rot_matrix_2;
				
				p2.set_from(&startvec);
                p2 *= &y_rot_matrix_2;
				p2 *= &z_rot_matrix_2;
				
				p3.set_from(&startvec);
				p3 *= &y_rot_matrix;
				p3 *= &z_rot_matrix;
				
				p4.set_from(&startvec);
				p4 *= &y_rot_matrix_2;
				p4 *= &z_rot_matrix;
				
				let mut shape: Shape = Shape::new_triangle(&p3, &p2, &p4);
				shape.set_color(&color);
				sphere.shapes.push(shape);
				
				shape  = Shape::new_triangle(&p1, &p2, &p3);
				shape.set_color(&color);
				sphere.shapes.push(shape);
			
				curlatdeg += degreelats;
				curlat += 1;
				if curlat == lats { break; }
			}
			
			curlongdeg -= degreelong;
			curlong += 1;
			color.r += longcolorstep;
			color.g += longcolorstep;
			if curlong == longs { break; }
		}
		
		
		sphere
	}

	pub fn cylinder(radius: f32, height: f32, longs: u32, lats: u32, showtop: bool, showbottom: bool) -> Mesh {
		let mut need_shapes: usize = (longs * lats * 2) as usize;
		if showtop { need_shapes +=	lats as usize; }
		if showbottom { need_shapes += lats as usize; }
		let mut cylinder: Mesh = Mesh::new(need_shapes);

		let degreelats: f32 = 360. / lats as f32;
		let heightseg: f32 = height / longs as f32;
		let startvec: Vec3 = Vec3::new(radius, 0., 0.);
		let centerbottom: Vec3 = Vec3::new_empty();
		let centertop: Vec3 = Vec3::new(0., height, 0.);
		let mut curlat: u32 = 0;
		let mut curdegreelats: f32 = 0.;

		let (mut p1, mut p2, mut p3, mut p4,
			 mut curvec, mut curvecnext) 	= (Vec3::new_empty(), Vec3::new_empty(), 
											   Vec3::new_empty(), Vec3::new_empty(),
											   Vec3::new_empty(), Vec3::new_empty());
		
		let mut y_rot_matrix: Mat3 = Mat3::new_empty();
		let mut y_rot_matrix_2: Mat3 = Mat3::new_empty();
			
			
		loop {
			
			let mut curlong: u32 = 0;
			let mut curheight: f32 = 0.;
			
			loop {
				
				curvec.set(startvec.x, curheight, startvec.z);
				curvecnext.set(startvec.x, curheight + heightseg, startvec.z);
				
				y_rot_matrix.to_roty(curdegreelats);
				y_rot_matrix_2.to_roty(curdegreelats + degreelats);
				
				p1.set(startvec.x, curheight, startvec.z);
				p1 *= &y_rot_matrix;
				
				p2.set(startvec.x, curheight, startvec.z);
				p2 *= &y_rot_matrix_2;
				
				p3.set(startvec.x, curheight + heightseg, startvec.z);
				p3 *= &y_rot_matrix;
				
				p4.set(startvec.x, curheight + heightseg, startvec.z);
				p4 *= &y_rot_matrix_2;
				
				cylinder.shapes.push(Shape::new_triangle(&p1, &p2, &p3));
				cylinder.shapes.push(Shape::new_triangle(&p3, &p2, &p4));
				
				if showbottom && (curlong == 0) {
					cylinder.shapes.push(Shape::new_triangle(&p2, &p1, &centerbottom));
				}
				
				curlong += 1;
				curheight += heightseg;

				if showtop && (curlong == longs) {
					cylinder.shapes.push(Shape::new_triangle(&p3, &p4, &centertop));
				}

				if curlong == longs { break; }
			}
			
			curdegreelats += degreelats;
			curlat += 1;
			if curlat == lats { break; }
		}
			
		cylinder
	}

	pub fn cone(radius: f32, height: f32, lats: u32, showbottom: bool) -> Mesh {
		let mut need_shapes: usize = lats as usize;
		if showbottom { need_shapes += lats as usize; }
		let mut cone: Mesh = Mesh::new(need_shapes);
		
		let degreelats: f32 = 360. / lats as f32;
		let startvec: Vec3 = Vec3::new(radius, 0., 0.);
		let centerbottom: Vec3 =  Vec3::new_empty();
		let centertop: Vec3 = Vec3::new(0., height, 0.);
		let mut curlat: u32 = 0;
		let mut curdegreelats: f32 = 0.;
		
		let (mut p1, mut p2) = (Vec3::new_empty(), Vec3::new_empty());
		
		let mut rot_matrix: Mat3 = Mat3::new_empty();
		
		loop {

			rot_matrix.to_roty(curdegreelats);

			p1.set_from(&startvec);
			p1 *= &rot_matrix;

			rot_matrix.to_roty(curdegreelats + degreelats);

			p2.set_from(&startvec);
			p2 *= &rot_matrix;
			
			cone.shapes.push(Shape::new_triangle(&centertop, &p1, &p2));
			
			if showbottom {
				cone.shapes.push(Shape::new_triangle(&p2, &p1, &centerbottom));
			}
			
			curdegreelats += degreelats;
			curlat += 1;
			if curlat == lats { break; }
		}
		
		cone
	}
	
	pub fn path(radius: f32, cntelements: u32, height: f32, longs: u32, scaleend: f32) -> Mesh {
		let mut path: Mesh = Mesh::new((cntelements * longs * 2) as usize);
		
		let ang_max_x: f32 = 32.; 
		let ang_max_y: f32 = 31.; 
		let ang_max_z: f32 = 30.;
		
		let mut startvec: Vec3 = Vec3::new_empty();
		let startvecring: Vec3 = Vec3::new(radius, 0., 0.);
		let mut curelement = 0;
		let heightseg: f32 = height / cntelements as f32;
		
		let mut curscaling: f32 = 1.0;
		let scalestep: f32 = (curscaling - scaleend) / cntelements as f32;

		let (mut roty, mut rotz, mut rotmat) = (Mat3::new_empty(), Mat3::new_empty(), Mat3::new_empty());

		let (mut p1, mut p2, mut p3, mut p4) = (Vec3::new_empty(), Vec3::new_empty(), 
											    Vec3::new_empty(), Vec3::new_empty());
												
		let degreelong: f32 = 360. / longs as f32;
		
		let mut y_rot_matrix = Mat3::new_empty();
		let mut y_rot_matrix_2 = Mat3::new_empty();
		
		let mut cur_shape: usize = 0;
		
		while curelement < cntelements {

			rotmat.to_rotx(rand_path_deg(ang_max_x));
			roty.to_roty(rand_path_deg(ang_max_y));
			rotz.to_rotz(rand_path_deg(ang_max_z));

			let mut basevec = Vec3::new(0., heightseg, 0.);
			basevec *= &rotmat;
			basevec *= &roty;
			basevec *= &rotz;

            let mut endvec: Vec3 = startvec.clone();
			endvec += &basevec;
			startvec.set_from(&endvec);
			
			let curvec = startvecring.clone();
			let mut curlongdeg: f32 = 0.;
            
			let scalevec: Vec3 = Vec3::new(curscaling, 1.0, curscaling);
			
			while (curlongdeg + degreelong) <= 360. {
				y_rot_matrix.to_roty(curlongdeg);
				y_rot_matrix_2.to_roty(curlongdeg + degreelong);
				
				//P1 und P2 sind die p3 und p4 des vorgänger rings. wenn nich vorhanden dann neu berechnen(erster ring)
				if  curelement == 0 {
					p1.set_from(&curvec);
					p1 *= &y_rot_matrix;
					
					p2.set_from(&curvec);
					p2 *= &y_rot_matrix_2;
				} else {
					let pasttriindx: usize =  cur_shape - ((longs*2) + 1) as usize;
					p1.set_from(&path.shapes[pasttriindx].vertices[0].vec);
					p2.set_from(&path.shapes[pasttriindx].vertices[2].vec)
				}

				//p3 und p4 sind multiplikation vom base vector und das addiert von p1 und p2 des letzten rings
				
				p3.set_from(&p1);
				p3 += &basevec;
				p4.set_from(&p2);
				p4 += &basevec;

				//scaling current vector
				p3.scale(scalevec.x, 1., scalevec.z);
				p4.scale(scalevec.x, 1., scalevec.z);
				
				path.shapes[cur_shape] = Shape::new_triangle(&p1, &p2, &p3);
				cur_shape += 1;          
				path.shapes[cur_shape] = Shape::new_triangle(&p3, &p2, &p4);
				cur_shape += 1;
				
				curlongdeg += degreelong;
			}
			
			curscaling -= scalestep;
			curelement += 1;
		}
		path 
	}
	
}


