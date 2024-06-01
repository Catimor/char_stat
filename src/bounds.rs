use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bounds {
	v_min: f64,
	v_max: f64,
	is_min_mut: bool,
	is_max_mut: bool,
}// Bounds

impl Bounds {
	pub fn new ( v_min: f64, v_max: f64, is_min_mut: bool, is_max_mut: bool ) -> Option< Self > {
		if v_min > v_max || v_min.is_nan() || v_max.is_nan() {
			return None
		}
		
		Some( Bounds {
			v_min,
			v_max,
			is_min_mut,
			is_max_mut,
		})
	}// new
	
	pub fn new_const ( v_min: f64, v_max: f64 ) -> Option< Self > {
		if v_min > v_max || v_min.is_nan() || v_max.is_nan() {
			return None
		}
		
		Some( Bounds {
			v_min,
			v_max,
			is_min_mut: false,
			is_max_mut: false,
		})
	}// new_const
	
	pub fn new_mut ( v_min: f64, v_max: f64 ) -> Option< Self > {
		if v_min > v_max || v_min.is_nan() || v_max.is_nan() {
			return None
		}
		
		Some( Bounds {
			v_min,
			v_max,
			is_min_mut: true,
			is_max_mut: true,
		})
	}// new_mut
	
	pub fn set_min ( &mut self, new_val: f64 ) -> Result<(),()> {
		if !self.is_min_mut || new_val > self.v_max || new_val.is_nan() {
			return Err(())
		}
		
		self.v_min = new_val;
		
		Ok(())
	}// set_min
	
	pub fn set_max ( &mut self, new_val: f64 ) -> Result<(),()> {
		if !self.is_max_mut || new_val < self.v_min || new_val.is_nan() {
			return Err(())
		}
		
		self.v_max = new_val;
		
		Ok(())
	}// set_max
	
	pub fn set_min_const ( &mut self ) {
		self.is_min_mut = false;
	}
	
	pub fn set_max_const ( &mut self ) {
		self.is_max_mut = false;
	}
	
	pub fn min ( &self ) -> f64 {
		self.v_min
	}
	
	pub fn max ( &self ) -> f64 {
		self.v_max
	}
}// Bounds

impl Default for Bounds {
	#[inline]
	fn default () -> Self {
		Bounds{
			v_min: 0.0,
			v_max: 1.0,
			is_min_mut: true,
			is_max_mut: true,
		}
	}// new
}// Bounds

impl fmt::Display for Bounds {
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
		let min_m = if self.is_min_mut {
			"mut"
		} else {
			"const"
		};
		
		let max_m = if self.is_min_mut {
			"mut"
		} else {
			"const"
		};
		
		write!( f, "Bounds = ({2} min: {0}, {3} max: {1})", self.v_min, self.v_max, min_m, max_m )
	}
}

/*
#[derive(Clone, PartialEq, Debug)]
pub struct BoundsModified {
	bounds: Bounds,
	min_modified : f64,
	max_modified : f64,
}

impl BoundsModified {
	pub fn new ( bounds: Bounds ) -> Self {
		BoundsModified {
			min_modified: bounds.min(),
			max_modified: bounds.max(),
			bounds,
		}
	}// new
	
	pub(crate) fn modify_bounds( &mut self, new_bounds: (f64, f64) ) -> Result<(),()> {
		let mod_min = new_bounds.0 + self.bounds.min();
		let mod_max = new_bounds.1 + self.bounds.max();
		
		if mod_min > mod_max {
			return Err(())
		}
		
		self.min_modified = mod_min;
		self.max_modified = mod_max;
		
		Ok(())
	}
	
	pub fn min_modified( &self ) -> f64 {
		self.min_modified
	}
	
	pub fn max_modified( &self ) -> f64 {
		self.max_modified
	}
}// BoundsModified

impl Default for BoundsModified {
	#[inline]
	fn default () -> Self {
		BoundsModified::new( Bounds::default() )
	}// new
}// BoundsModified

// indirect calls to Bounds
impl BoundsModified {
	pub fn set_min ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.bounds.set_min( new_val )?;
		self.min_modified = new_val;
		
		Ok(())
	}// set_min
	
	pub fn set_max ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.bounds.set_max( new_val )?;
		self.max_modified = new_val;
		
		Ok(())
	}// set_max
	
	pub fn set_min_const ( &mut self ) {
		self.bounds.set_min_const();
	}
	
	pub fn set_max_const ( &mut self ) {
		self.bounds.set_max_const();
	}
	
	pub fn min ( &self ) -> f64 {
		self.bounds.min()
	}
	
	pub fn max ( &self ) -> f64 {
		self.bounds.max()
	}
}// BoundsModified
*/

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn nan_handling() {
		// Bounds - min
		let bad_min = Bounds::new_const( f64::NAN, 5.0, );
		assert_eq!( bad_min, None );
		let bad_min = Bounds::new_mut( f64::NAN, 5.0, );
		assert_eq!( bad_min, None );
		let bad_min = Bounds::new( f64::NAN, 5.0, true, false, );
		assert_eq!( bad_min, None );
		
		// Bounds - max
		let bad_max = Bounds::new_const( 5.0, f64::NAN, );
		assert_eq!( bad_max, None );
		let bad_max = Bounds::new_mut( 5.0, f64::NAN, );
		assert_eq!( bad_max, None );
		let bad_max = Bounds::new( 5.0, f64::NAN, true, false, );
		assert_eq!( bad_max, None );
	}// nan_handling
	
	#[test]
	fn mutability() {
		let mut bounds_1 = Bounds::new_const( 1.0, 10.0, ).expect( "..." );
		
		assert_eq!( bounds_1.set_min( 5.0 ), Err(()) );
		assert_eq!( bounds_1.set_max( 5.0 ), Err(()) );
		
		let mut bounds_2 = Bounds::new_mut( 1.0, 10.0, ).expect( "..." );
		
		assert_eq!( bounds_2.set_min( 5.0 ), Ok(()) );
		assert_eq!( bounds_2.set_max( 5.0 ), Ok(()) );
		
		bounds_2.set_min_const();
		bounds_2.set_max_const();
		
		assert_eq!( bounds_2.set_min( 1.0 ), Err(()) );
		assert_eq!( bounds_2.set_max( 10.0 ), Err(()) );
	}// mutability
}
