use math::vec::vec3::*;
use math::mat::mat4::*;
use math::mat::mat3::*;
use std::f32::consts::PI;

pub struct Camera {
	pub view: Mat4,
	pub projection: Mat4,
	pub transformation: Mat4,
	pub forward: Vec3,
	pub left: Vec3,
	pub up: Vec3,
	pub from: Vec3,
	pub to: Vec3,
	pub l: f32,
	pub r: f32,
	pub t: f32,
	pub b: f32,
	pub n: f32,
	pub f: f32,
}

impl Camera {
	pub fn new() -> Camera {
		Camera { view: 			Mat4::new_empty(), 
				 projection: 	Mat4::new_empty(),
				 transformation:Mat4::new_empty(),
				 forward: 	    Vec3::new_empty(),
				 up: 		    Vec3::new_empty(),
				 left:          Vec3::new_empty(),
				 from: 		    Vec3::new_empty(),
				 to:            Vec3::new_empty(),
				 l: 0.0, r: 0.0, t: 0.0, b: 0.0, n: 0.0, f: 0.0,
		}
	}

	pub fn new_init(from: &Vec3, to: &Vec3, l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Camera {
		Camera { view: 			Mat4::new_empty(), 
				 projection: 	Mat4::new_empty(),
				 transformation:Mat4::new_empty(),
				 forward: 	    Vec3::new_empty(),
				 up: 		    Vec3::new_empty(),
				 left:          Vec3::new_empty(),
				 from: 		    from.clone(),
				 to:            to.clone(),
				 l, r, t, b, n, f,
		}
	}
	
	pub fn set_viewport(&mut self, l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) {
		self.l = l;
		self.r = r;
		self.t = t;
		self.b = b;
		self.n = n;
		self.f = f;
	}
	
	pub fn look_at_ortho(&mut self, from: &Vec3, to: &Vec3) {
	
		self.forward.set_from(from);
		self.forward -= to;
		self.forward.normalize();
		
		let mut tmp = Vec3::new(0.0, 1.0, 0.0);
		tmp.normalize();
		
		//self.left.set_from(&tmp);
		//self.left = self.left.cross(&self.forward);
		tmp.cross_to(&self.forward, &mut self.left);
		self.left.normalize();
		
		//self.up.set_from(&self.forward);
		//self.up = self.up.cross(&self.left);
		self.forward.cross_to(&self.left, &mut self.up);
		self.up.normalize();
		
		self.view._11 = self.left.x;
		self.view._21 = self.left.y;
		self.view._31 = self.left.z;
		self.view._41 = 0.0;//-(right*from)
		
		self.view._12 = self.up.x;
		self.view._22 = self.up.y;
		self.view._32 = self.up.z;
		self.view._42 = 0.0;//-(up*from)
		
		self.view._13 = self.forward.x;
		self.view._23 = self.forward.y;
		self.view._33 = self.forward.z;
		self.view._43 = 0.0;//-(forward*from)
		
		self.view._14 = from.x;
		self.view._24 = from.y;
		self.view._34 = from.z;
		self.view._44 = 1.0;
		
		self.view.invert();
	}
	
	pub fn to_projection_ortho(&mut self) {
		
		self.projection._11 = 2.0/(self.r-self.l);
		self.projection._12 = 0.0;
		self.projection._13 = 0.0;
		self.projection._14 = -(self.r+self.l)/(self.r-self.l);

		self.projection._21 = 0.0;
		self.projection._22 = 2.0/(self.t-self.b);
		self.projection._23 = 0.0;
		self.projection._24 = -(self.t+self.b)/(self.t-self.b);

		self.projection._31 = 0.0;
		self.projection._32 = 0.0;
		self.projection._33 = -2.0/(self.f-self.n);
		self.projection._34 = -(self.f+self.n)/(self.f-self.n);

		self.projection._41 = 0.0;
		self.projection._42 = 0.0;
		self.projection._43 = 0.0;
		self.projection._44 = 1.0;
	}
	
	pub fn to_projection_ortho2(&mut self) {
		self.projection._11 = 1.0/self.r;
		self.projection._12 = 0.0;
		self.projection._13 = 0.0;
		self.projection._14 = 0.0;
		   
		self.projection._21 = 0.0;
		self.projection._22 = 1.0/self.t;
		self.projection._23 = 0.0;
		self.projection._24 = 0.0;
		   
		self.projection._31 = 0.0;
		self.projection._32 = 0.0;
		self.projection._33 = -2.0/(self.f-self.n);
		self.projection._34 = -(self.f+self.n)/(self.f-self.n);
		   
		self.projection._41 = 0.0;
		self.projection._42 = 0.0;
		self.projection._43 = 1.0;
		self.projection._44 = 1.0;
	}
	
	
	pub fn to_ortho(&mut self, from: &Vec3, to: &Vec3, l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) {
		self.set_viewport(l,r,t,b,n,f);
		self.look_at_ortho(from, to);
		self.to_projection_ortho();
		self.transformation.set_from(&self.projection);
		self.transformation *= &self.view;
	}
	
	/*
		this function uses only matrix and vector. As Reason is a maybe performace boost by avooding function calls.
		Reason for tryout performance boost is a fps depend camera rotation
	*/
	pub fn look_at_perspective(&mut self, from: &Vec3, to: &Vec3) {
	
		self.forward.set_from(from);
		self.forward -= to;
		self.forward.normalize();
		
		let mut tmp = Vec3::new(0.0, 1.0, 0.0);
		tmp.normalize();
		
		tmp.cross_to(&self.forward, &mut self.left);
		self.left.normalize();
		
		self.forward.cross_to(&self.left, &mut self.up);
		self.up.normalize();
		
		let m: Mat4 = Mat4::new( self.left.x, self.up.x, -self.forward.x, from.x,	
							     self.left.y, self.up.y, -self.forward.y, from.y,	
							     self.left.z, self.up.z, -self.forward.z, from.z, 	
							     0.0		, 0.0      , 0.0		    , 1.0 );
		
		
		//inverse: Base matrix
		{
			let mut t = Mat3::new( m._22, m._23, m._24, m._32, m._33, m._34, m._42, m._43, m._44);
			
			self.view._11 = (t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								  (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&temp);
			
			t._11 = m._21; t._21 = m._31; t._31 = m._41;
			self.view._21 = -(t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								   (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._12 = m._22; t._22 = m._32; t._32 = m._42;
			self.view._31 = (t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								  (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._13 = m._23; t._23 = m._33; t._33 = m._43;
			self.view._41 = -(t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								   (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._11 = m._12; t._12 = m._13; t._13 = m._14; 
			t._21 = m._32; t._22 = m._33; t._23 = m._34; 
			t._31 = m._42; t._32 = m._43; t._33 = m._44;
			self.view._12 = -(t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								   (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._11 = m._11; t._21 = m._31; t._31 = m._41;
			self.view._22 = (t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								  (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._12 = m._12; t._22 = m._32; t._32 = m._42;
			self.view._32 = -(t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								   (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._13 = m._13; t._23 = m._33; t._33 = m._43;
			self.view._42 = (t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								  (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._11 = m._12; t._12 = m._13; t._13 = m._14; 
			t._21 = m._22; t._22 = m._23; t._23 = m._24; 
			t._31 = m._42; t._32 = m._43; t._33 = m._44;
			self.view._13 = (t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								  (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._11 = m._11; t._21 = m._21; t._31 = m._41;
			self.view._23 = -(t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								   (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._12 = m._12; t._22 = m._22; t._32 = m._42;
			self.view._33 = (t._11*t._22*t._33) +(t._12*t._23*t._31) + (t._13*t._21*t._32) -
								  (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._13 = m._13; t._23 = m._23; t._33 = m._43;
			self.view._43 = -(t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								   (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._11 = m._12; t._12 = m._13; t._13 = m._14; 
			t._21 = m._22; t._22 = m._23; t._23 = m._24; 
			t._31 = m._32; t._32 = m._33; t._33 = m._34;
			self.view._14 = -(t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) - 
								   (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._11 = m._11; t._21 = m._21; t._31 = m._31;
			self.view._24 = (t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								  (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._12 = m._12; t._22 = m._22; t._32 = m._32;
			self.view._34 = -(t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								   (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			t._13 = m._13; t._23 = m._23; t._33 = m._33;
			self.view._44 = (t._11*t._22*t._33) + (t._12*t._23*t._31) + (t._13*t._21*t._32) -
								  (t._13*t._22*t._31) - (t._12*t._21*t._33) - (t._11*t._23*t._32);//mat3_determinant(&t);
			
			//inverse: determinant inside inverse calc
			let det: f32 = (m._11 * self.view._11) + 
						   (m._12 * -self.view._21 ) + 
						   (m._13 * self.view._31) +
						   (m._14 * -self.view._41);
			
			//inverse: mul at with 1/det
			self.view._11 *= det; self.view._12 *= det; self.view._13 *= det; self.view._14 *= det;
			self.view._21 *= det; self.view._22 *= det; self.view._23 *= det; self.view._24 *= det;
			self.view._31 *= det; self.view._32 *= det; self.view._33 *= det; self.view._34 *= det;
			self.view._41 *= det; self.view._42 *= det; self.view._43 *= det; self.view._44 *= det;
		}
		
	}
	
	//scratch a pixel projection => right hand Matrix in use like directX
	pub fn to_projection_perspective_(&mut self) {
		let scale: f32 = 1.0 / (90.0 * 0.5 * PI / 180.0).tan(); 
		self.projection._11 = scale;
		self.projection._12 = 0.0;
		self.projection._13 = 0.0;
		self.projection._14 = 0.0;
		   
		self.projection._21 = 0.0;
		self.projection._22 = scale;
		self.projection._23 = 0.0;
		self.projection._24 = 0.0;
		   
		self.projection._31 = 0.0;
		self.projection._32 = 0.0;
		self.projection._33 = -self.f/(self.f-self.n);
		self.projection._34 = -(self.f*self.n)/(self.f-self.n);
		   
		self.projection._41 = 0.0;
		self.projection._42 = 0.0;
		self.projection._43 = 1.0;
		self.projection._44 = 0.0;
	}
	
	// open GL projection
	pub fn to_projection_perspective(&mut self) {
		let scale: f32 = 1.0 as f32 / (90.0 as f32 * 0.5 as f32 * PI / 180.0 as f32).tan(); 
		self.projection._11 = scale;//(2.0*self.n)/(self.r-self.l);//scale;//
		self.projection._12 = 0.0;
		self.projection._13 = (self.r+self.l)/(self.r-self.l);
		self.projection._14 = 0.0;
		   
		self.projection._21 = 0.0;
		self.projection._22 = scale;//(2.0*self.n)/(self.t-self.b); //scale;//
		self.projection._23 = (self.t+self.b)/(self.t-self.b);
		self.projection._24 = 0.0;
		   
		self.projection._31 = 0.0;
		self.projection._32 = 0.0;
		self.projection._33 = -(self.f+self.n)/(self.f-self.n);
		self.projection._34 = -(2.0*self.f*self.n)/(self.f-self.n);
		  
		self.projection._41 = 0.0;
		self.projection._42 = 0.0;
		self.projection._43 = 1.0;
		self.projection._44 = 0.0;
	}
	
	pub fn to_perspective(&mut self, from: &Vec3, to: &Vec3, l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) {
		self.set_viewport(l,r,t,b,n,f);
		self.look_at_perspective(from, to);
		self.to_projection_perspective();
		self.transformation.set_from(&self.projection);
		self.transformation *= &self.view;
	}
	
}