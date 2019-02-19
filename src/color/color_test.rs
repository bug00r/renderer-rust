use color::*;

#[test]
fn create() {
	let col = Color::new(1., 2., 3.);
	
	assert_eq!(col == BLACK, false);
	assert_eq!(WHITE == BLACK, false);
	assert_eq!(WHITE != BLACK, true);
	
}	