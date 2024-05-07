use super::{ Bounds, BaseMultConf, RoundingFunctionEnum, RoundingHelper };

#[derive(Clone, PartialEq, Debug)]
pub struct BaseConf {
	value: f64,
	is_mut: bool,
	bounds: Bounds,
	rounding_fn: RoundingHelper,
	mult: Option< BaseMultConf >,
}// BaseConf

impl BaseConf {
	pub fn new ( value: f64, is_mut: bool, bounds: Bounds, rounding_fn: RoundingHelper, mult: Option< BaseMultConf > ) -> Option< Self > {
		if value.is_nan() || value < bounds.min() || value > bounds.max() {
			
			return None
		}
		
		Some( BaseConf {
			value,
			is_mut,
			bounds,
			rounding_fn,
			mult,
		})
	}// new
	
	pub fn new_clamping ( value: f64, is_mut: bool, bounds: Bounds, rounding_fn: RoundingHelper, mult: Option< BaseMultConf > ) -> Option< Self > {
		if value.is_nan() {
			
			return None
		}
		
		Some( BaseConf {
			value: value.clamp( bounds.min(), bounds.max() ),
			is_mut,
			bounds,
			rounding_fn,
			mult,
		})
	}// new_clamping
	
	pub fn set_value ( &mut self, value: f64 ) -> Result<(),()> {
		if !self.is_mut || value.is_nan() || value < self.bounds.min() || value > self.bounds.max() {
			
			return Err(())
		}
		
		self.value = value;
		
		Ok(())
	}// set_value
	
	pub fn set_value_clamping ( &mut self, value: f64 ) -> Result<(),()> {
		if !self.is_mut || value.is_nan() {
			
			return Err(())
		}
		
		self.value = value.clamp( self.bounds.min(), self.bounds.max() );
		
		Ok(())
	}// set_value
	
	pub fn value ( &self ) -> f64 {
		if let Some( mlt ) = &self.mult {
			return mlt.calculate( self.value )
		}
		
		self.value
	}// get_value
	
	pub fn set_value_const ( &mut self ) {
		self.is_mut = false;
	}
}// BaseConf

// bounds
impl BaseConf {
	/*
	pub fn modify_bounds ( &mut self, new_bounds: (f64, f64) ) -> Result<(),()> {
		self.bounds.modify_bounds( new_bounds )?;
		
		Ok(())
	}// modify_bounds
	*/
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
	/*
	pub fn get_bounds_modified ( &self ) -> (f64, f64) {
		(self.bounds.min_modified(), self.bounds.max_modified())
	}
	
	pub fn get_min_modified ( &self ) -> f64 {
		self.bounds.min_modified()
	}
	
	pub fn get_max_modified ( &self ) -> f64 {
		self.bounds.max_modified()
	}
	*/
}// BaseConf.bounds: Bounds

// mult
impl BaseConf {
	pub fn set_mult_base ( &mut self, new_val: f64 ) -> Result<(),()> {
		if let Some( mlt ) = &mut self.mult {
			mlt.set_base( new_val )?;
			
			return Ok(())
		}
		
		Err(())
	}// set_mult_base
	
	pub fn set_mult_exponent ( &mut self, new_val: f64 ) -> Result<(),()> {
		if let Some( mlt ) = &mut self.mult {
			mlt.set_exponent( new_val )?;
			
			return Ok(())
		}
		
		Err(())
	}// set_mult_exponent
	
	pub fn set_mult_base_clamping ( &mut self, new_val: f64 ) -> Result<(),()> {
		if let Some( mlt ) = &mut self.mult {
			mlt.set_base_clamping( new_val )?;
			
			return Ok(())
		}
		
		Err(())
	}// set_mult_base_clamping
	
	pub fn set_mult_exponent_clamping ( &mut self, new_val: f64 ) -> Result<(),()> {
		if let Some( mlt ) = &mut self.mult {
			mlt.set_exponent_clamping( new_val )?;
			
			return Ok(())
		}
		
		Err(())
	}// set_mult_exponent_clamping
}// BaseConf.mult: BaseMultConf

impl Default for BaseConf {
	fn default () -> Self {
		BaseConf {
			value: 0.0,
			is_mut: true,
			bounds: Bounds::default(),
			rounding_fn: RoundingHelper::new( RoundingFunctionEnum::None, None ),
			mult: None,
		}
	}// default
}// Default for BaseConf

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn basic_functional() {
		let rounding_helper = RoundingHelper::new( RoundingFunctionEnum::None, None );
		
		let bounds = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let base = BaseConf::new( 15.0, true, bounds, rounding_helper.clone(), None );
		assert_eq!( base, None );
		
		let bounds = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let base = BaseConf::new_clamping( 15.0, true, bounds, rounding_helper, None );
		assert!( base.is_some() );
		
		let mut base = base.unwrap();
		assert_eq!( base.value(), 10.0 );
		
		assert_eq!( base.set_value( 5.0 ), Ok(()) );
		assert_eq!( base.value(), 5.0 );
	}// basic_functional
	
	#[test]
	fn nan_handling() {
		let base = BaseConf::new( f64::NAN, true, Bounds::default(), RoundingHelper::default(), None );
		assert_eq!( base, None );
		let base = BaseConf::new_clamping( f64::NAN, true, Bounds::default(), RoundingHelper::default(), None );
		assert_eq!( base, None );
	}// nan_handling
}
