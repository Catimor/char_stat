use super::{ RoundingHelper, Bounds };

#[derive(Clone, PartialEq, Debug)]
pub struct BaseMultConf {
	base: f64,
	exponent: f64,
	rounding_fn: RoundingHelper,
	bounds_base: Bounds,
	bounds_exp: Bounds,
	
	multiplier: f64,
}// BaseMultConf

impl BaseMultConf {
	pub fn new ( base: f64, exponent: f64, bounds_base: Bounds, bounds_exp: Bounds, rounding_fn: RoundingHelper ) -> Option< Self > {
		if base.is_nan() || exponent.is_nan() {
			
			return None
		}
		
		if base < bounds_base.min() || base > bounds_base.max() {
			
			return None
		}
		
		if exponent < bounds_exp.min() || exponent > bounds_exp.max() {
			
			return None
		}
		
		let multiplier = base.powf( exponent );
		
		Some( BaseMultConf {
			base,
			exponent,
			rounding_fn,
			bounds_base,
			bounds_exp,
			multiplier,
		})
	}// new
	
	pub fn new_clamping ( mut base: f64, mut exponent: f64, bounds_base: Bounds, bounds_exp: Bounds, rounding_fn: RoundingHelper ) -> Option< Self > {
		if base.is_nan() || exponent.is_nan() {
			
			return None
		}
		
		base = base.clamp( bounds_base.min(), bounds_base.max() );
		
		exponent = exponent.clamp( bounds_exp.min(), bounds_exp.max() );
		
		let multiplier = base.powf( exponent );
		
		Some( BaseMultConf {
			base,
			exponent,
			rounding_fn,
			bounds_base,
			bounds_exp,
			multiplier,
		})
	}// new_clamping
	
	pub fn update ( &mut self ) {
		let val = self.base.powf( self.exponent );
		let mlt = self.rounding_fn.do_rounding( val );
		
		self.multiplier = mlt;
	}
	
	pub fn calculate ( &self, value: f64 ) -> f64 {
		value * self.multiplier
	}// calculate
	
	pub fn set_base ( &mut self, new_val: f64 ) -> Result<(),()> {
		if new_val.is_nan() || new_val < self.bounds_base.min() || new_val > self.bounds_base.max() {
			return Err(())
		}
		
		self.base = new_val;
		self.update();
		
		Ok(())
	}// set_base
	
	pub fn set_base_clamping ( &mut self, new_val: f64 ) -> Result<(),()> {
		if new_val.is_nan() {
			return Err(())
		}
		
		self.base = new_val.clamp( self.bounds_base.min(), self.bounds_base.max() );
		self.update();
		
		Ok(())
	}// set_base_clamping
	
	pub fn inc_base ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.set_base( self.base + new_val )
	}
	
	pub fn dec_base ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.set_base( self.base - new_val )
	}
	
	pub fn set_exponent ( &mut self, new_val: f64 ) -> Result<(),()> {
		if new_val.is_nan() || new_val < self.bounds_exp.min() || new_val > self.bounds_exp.max() {
			return Err(())
		}
		
		self.exponent = new_val;
		self.update();
		
		Ok(())
	}// set_exponent
	
	pub fn set_exponent_clamping ( &mut self, new_val: f64 ) -> Result<(),()> {
		if new_val.is_nan() {
			return Err(())
		}
		
		self.exponent = new_val.clamp( self.bounds_exp.min(), self.bounds_exp.max() );
		self.update();
		
		Ok(())
	}// set_exponent_clamping
	
	pub fn inc_exp ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.set_exponent( self.exponent + new_val )
	}
	
	pub fn dec_exp ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.set_exponent( self.exponent - new_val )
	}
	
	pub fn set_rounding ( &mut self, new_val: RoundingHelper ) {
		self.rounding_fn = new_val;
	}
}// BaseMultConf

#[cfg(test)]
mod tests {
	use crate::{ Bounds, BaseMultConf, RoundingHelper };//, RoundingFunctionEnum
	
	#[test]
	fn nan_handling() {
		// base
		let base_mult = BaseMultConf::new( f64::NAN, 1.0, Bounds::default(), Bounds::default(), RoundingHelper::default() );
		assert_eq!( base_mult, None );
		
		// exponent
		let base_mult = BaseMultConf::new( 1.0, f64::NAN, Bounds::default(), Bounds::default(), RoundingHelper::default() );
		assert_eq!( base_mult, None );
	}// nan_handling
}
