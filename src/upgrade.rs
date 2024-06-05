use super::{ Bounds, CharStatError, CsInvalidValue };

#[derive(Clone, PartialEq, Debug)]
pub struct UpgradeConf {
	value: f64,
	bounds: Bounds,
}// UpgradeConf

impl UpgradeConf {
	#[inline]
	pub fn new ( value: f64, bounds: Bounds, ) -> Result< Self, CharStatError > {
		UpgradeConf::check_inval( value, &bounds )?;
		
		Ok( UpgradeConf {
			value,
			bounds,
		})
	}// new
	
	#[inline]
	pub fn new_clamping ( mut value: f64, bounds: Bounds, ) -> Result< Self, CharStatError > {
		UpgradeConf::check_nan( value )?;
		
		value = value.clamp( bounds.min(), bounds.max() );
		
		Ok( UpgradeConf {
			value,
			bounds,
		})
	}// new_clamping
	
	#[inline]
	pub fn set_value ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		UpgradeConf::check_inval( value, &self.bounds )?;
		
		self.value = value;
		
		Ok(())
	}// set_value
	
	#[inline]
	pub fn set_value_clamping ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		UpgradeConf::check_nan( value )?;
		
		self.value = value.clamp( self.bounds.min(), self.bounds.max() );
		
		Ok(())
	}
	
	#[inline]
	pub fn value ( &self ) -> f64 {
		self.value
	}
}// UpgradeConf

// indirect calls to Bounds
impl UpgradeConf {
	#[inline]
	pub fn set_bounds_min ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.bounds.set_min( new_val )?;
		
		Ok(())
	}// set_min
	
	#[inline]
	pub fn set_bounds_max ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.bounds.set_max( new_val )?;
		
		Ok(())
	}// set_max
	
	#[inline]
	pub fn set_bounds_min_const ( &mut self ) {
		self.bounds.set_min_const();
	}
	
	#[inline]
	pub fn set_bounds_max_const ( &mut self ) {
		self.bounds.set_max_const();
	}
	
	#[inline]
	pub fn bounds_min ( &self ) -> f64 {
		self.bounds.min()
	}
	
	#[inline]
	pub fn bounds_max ( &self ) -> f64 {
		self.bounds.max()
	}
}// UpgradeConf

//priv
impl UpgradeConf {
	#[inline(always)]
	fn check_inval( value: f64, bounds: &Bounds ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( "value".to_string() ) )?
		}
		if value < bounds.min() {
			
			return Err( CsInvalidValue::BelowMinimum( "value".to_string() ) )?
		}
		if value > bounds.max() {
			
			return Err( CsInvalidValue::AboveMaximum( "value".to_string() ) )?
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn check_nan( value: f64 ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( "value".to_string() ) )?
		}
		
		Ok(())
	}
}// UpgradeConf

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn basic_functional() {
		let bounds_upgr = Bounds::new_const( 0.0, 50.0 ).unwrap();
		let mut upgrade = UpgradeConf::new( 0.0, bounds_upgr ).unwrap();
		
		let expected: CharStatError = CsInvalidValue::AboveMaximum( "value".to_string() ).into();
		
		assert_eq!( upgrade.set_value( 69.0 ), Err( expected ) );
		assert_eq!( upgrade.value(), 0.0 );
		
		assert_eq!( upgrade.set_value( 5.0 ), Ok(()) );
	}// basic_functional
	
	#[test]
	fn nan_handling() {
		let expected: CharStatError = CsInvalidValue::Nan( "value".to_string() ).into();
		
		let upgrade = UpgradeConf::new( f64::NAN, Bounds::default() );
		assert_eq!( upgrade, Err( expected.clone() ) );
		
		let upgrade = UpgradeConf::new_clamping( f64::NAN, Bounds::default() );
		assert_eq!( upgrade, Err( expected ) );
	}// nan_handling
}
