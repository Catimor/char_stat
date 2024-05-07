use super::Bounds;
//use super::{ Bounds };

#[derive(Clone, PartialEq, Debug)]
pub struct UpgradeConf {
	value: f64,
	bounds: Bounds,
}// UpgradeConf

impl UpgradeConf {
	pub fn new ( value: f64, bounds: Bounds, ) -> Option< Self > {
		if value.is_nan() || value < bounds.min() || value > bounds.max() {
			
			return None
		}
		
		Some( UpgradeConf {
			value,
			bounds,
		})
	}// new
	
	pub fn new_clamping ( mut value: f64, bounds: Bounds, ) -> Option< Self > {
		if value.is_nan() {
			
			return None
		}
		
		value = value.clamp( bounds.min(), bounds.max() );
		
		Some( UpgradeConf {
			value,
			bounds,
		})
	}// new_clamping
	
	pub fn set_value ( &mut self, value: f64 ) -> Result<(),()> {
		if value.is_nan() || value < self.bounds.min() || value > self.bounds.max() {
			
			return Err(())
		}
		
		self.value = value;
		
		Ok(())
	}// set_value
	
	pub fn set_value_clamping ( &mut self, value: f64 ) -> Result<(),()> {
		if value.is_nan() {
			
			return Err(())
		}
		
		self.value = value.clamp( self.bounds.min(), self.bounds.max() );
		
		Ok(())
	}
	
	pub fn value ( &self ) -> f64 {
		self.value
	}
}// UpgradeConf

// indirect calls to Bounds
impl UpgradeConf {
	pub fn set_bounds_min ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.bounds.set_min( new_val )?;
		
		Ok(())
	}// set_min
	
	pub fn set_bounds_max ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.bounds.set_max( new_val )?;
		
		Ok(())
	}// set_max
	
	pub fn set_bounds_min_const ( &mut self ) {
		self.bounds.set_min_const();
	}
	
	pub fn set_bounds_max_const ( &mut self ) {
		self.bounds.set_max_const();
	}
	
	pub fn bounds_min ( &self ) -> f64 {
		self.bounds.min()
	}
	
	pub fn bounds_max ( &self ) -> f64 {
		self.bounds.max()
	}
}// UpgradeConf

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn basic_functional() {
		let bounds_upgr = Bounds::new_const( 0.0, 50.0 ).unwrap();
		let mut upgrade = UpgradeConf::new( 0.0, bounds_upgr ).unwrap();
		
		assert_eq!( upgrade.set_value( 69.0 ), Err(()) );
		assert_eq!( upgrade.value(), 0.0 );
		
		assert_eq!( upgrade.set_value( 5.0 ), Ok(()) );
	}// basic_functional
	
	#[test]
	fn nan_handling() {
		let upgrade = UpgradeConf::new( f64::NAN, Bounds::default() );
		assert_eq!( upgrade, None );
		
		let upgrade = UpgradeConf::new_clamping( f64::NAN, Bounds::default() );
		assert_eq!( upgrade, None );
	}// nan_handling
}
