use super::{ RoundingHelper, Bounds, CharStatError, CsInvalidValue };

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
	#[inline]
	pub fn new ( base: f64, exponent: f64, bounds_base: Bounds, bounds_exp: Bounds, rounding_fn: RoundingHelper ) -> Result< Self, CharStatError > {
		BaseMultConf::check_inval( base, &bounds_base, "base".to_string() )?;
		BaseMultConf::check_inval( exponent, &bounds_exp, "exponent".to_string() )?;
		
		let multiplier = base.powf( exponent );
		
		Ok( BaseMultConf {
			base,
			exponent,
			rounding_fn,
			bounds_base,
			bounds_exp,
			multiplier,
		})
	}// new
	
	#[inline]
	pub fn new_clamping ( mut base: f64, mut exponent: f64, bounds_base: Bounds, bounds_exp: Bounds, rounding_fn: RoundingHelper ) -> Result< Self, CharStatError > {
		BaseMultConf::check_nan( base, "base".to_string() )?;
		BaseMultConf::check_nan( exponent, "exponent".to_string() )?;
		
		base = base.clamp( bounds_base.min(), bounds_base.max() );
		exponent = exponent.clamp( bounds_exp.min(), bounds_exp.max() );
		
		let multiplier = base.powf( exponent );
		
		Ok( BaseMultConf {
			base,
			exponent,
			rounding_fn,
			bounds_base,
			bounds_exp,
			multiplier,
		})
	}// new_clamping
	
	#[inline]
	pub fn update ( &mut self ) {
		let val = self.base.powf( self.exponent );
		let mlt = self.rounding_fn.do_rounding( val );
		
		self.multiplier = mlt;
	}
	
	#[inline]
	pub fn calculate ( &self, value: f64 ) -> f64 {
		value * self.multiplier
	}// calculate
	
	#[inline]
	pub fn set_base ( &mut self, base: f64 ) -> Result<(), CharStatError > {
		BaseMultConf::check_inval( base, &self.bounds_base, "base".to_string() )?;
		
		self.base = base;
		self.update();
		
		Ok(())
	}// set_base
	
	#[inline]
	pub fn set_base_clamping ( &mut self, base: f64 ) -> Result<(), CharStatError > {
		BaseMultConf::check_nan( base, "base".to_string() )?;
		
		self.base = base.clamp( self.bounds_base.min(), self.bounds_base.max() );
		self.update();
		
		Ok(())
	}// set_base_clamping
	
	#[inline]
	pub fn inc_base ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.set_base( self.base + new_val )
	}
	
	#[inline]
	pub fn dec_base ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.set_base( self.base - new_val )
	}
	
	#[inline]
	pub fn set_exponent ( &mut self, exponent: f64 ) -> Result<(), CharStatError > {
		BaseMultConf::check_inval( exponent, &self.bounds_exp, "exponent".to_string() )?;
		
		self.exponent = exponent;
		self.update();
		
		Ok(())
	}// set_exponent
	
	#[inline]
	pub fn set_exponent_clamping ( &mut self, exponent: f64 ) -> Result<(), CharStatError > {
		BaseMultConf::check_nan( exponent, "exponent".to_string() )?;
		
		self.exponent = exponent.clamp( self.bounds_exp.min(), self.bounds_exp.max() );
		self.update();
		
		Ok(())
	}// set_exponent_clamping
	
	#[inline]
	pub fn inc_exp ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.set_exponent( self.exponent + new_val )
	}
	
	#[inline]
	pub fn dec_exp ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.set_exponent( self.exponent - new_val )
	}
	
	#[inline]
	pub fn set_rounding ( &mut self, new_val: RoundingHelper ) {
		self.rounding_fn = new_val;
	}
}// BaseMultConf

//priv
impl BaseMultConf {
	#[inline(always)]
	fn check_inval( value: f64, bounds: &Bounds, name: String ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( name ) )?
		}
		if value < bounds.min() {
			
			return Err( CsInvalidValue::BelowMinimum( name ) )?
		}
		if value > bounds.max() {
			
			return Err( CsInvalidValue::AboveMaximum( name ) )?
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn check_nan( value: f64, name: String ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( name ) )?
		}
		
		Ok(())
	}
}// BaseMultConf

#[cfg(test)]
mod tests {
	use crate::{ BaseConf, BaseMultConf, Bounds, CharStatError, CsInvalidValue, RoundingHelper, RoundingFunctionEnum };
	
	#[test]
	fn basic_functional() {
		let bounds_mlt_base = Bounds::new_const( 1.1, 1.1 ).unwrap();
		let bounds_mlt_exp = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let bounds_base = Bounds::new_const( 0.0, 10000.0 ).unwrap();
		
		let base_mult = BaseMultConf::new( 1.1, 1.0, bounds_mlt_base, bounds_mlt_exp, RoundingHelper::new_none() ).unwrap();
		let mut base = BaseConf::new( 500.0, true, bounds_base, RoundingHelper::new( RoundingFunctionEnum::Floor, None ), Some( base_mult ) ).unwrap();
		
		let expected = [ 500.0, 550.0, 605.0, 665.0, 732.0, 805.0, 885.0, 974.0, 1071.0, 1178.0, 1296.0 ];
		
		let mut i = 1.0;
		for idx in 1..=10 {
			base.set_mult_exponent( i ).unwrap();
			
			assert_eq!( base.value(), expected[ idx ] );
			
			i += 1.0;
		}
	}// basic_functional
	
	#[test]
	fn nan_handling() {
		// base
		let base_mult = BaseMultConf::new( f64::NAN, 1.0, Bounds::default(), Bounds::default(), RoundingHelper::default() );
		let expected: CharStatError = CsInvalidValue::Nan( "base".to_string() ).into();
		assert_eq!( base_mult, Err( expected ) );
		
		// exponent
		let base_mult = BaseMultConf::new( 1.0, f64::NAN, Bounds::default(), Bounds::default(), RoundingHelper::default() );
		let expected: CharStatError = CsInvalidValue::Nan( "exponent".to_string() ).into();
		assert_eq!( base_mult, Err( expected ) );
	}// nan_handling
}
