#[cfg(feature = "serde")]
use serde::{ Serialize, Deserialize };

// --Imports
//------------------------------------------------------------------------------
// --Modules

use super::{ RoundingHelper, Bounds, CharStatError, CsInvalidValue };

// --Modules
//------------------------------------------------------------------------------
// struct - BaseMultConf

#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq,  )]
pub struct BaseMultConf {
	base: f64,
	exponent: f64,
	rounding_fn: RoundingHelper,
	bounds_base: Bounds,
	bounds_exp: Bounds,
	
	multiplier: f64,
}

impl BaseMultConf {
	/// # Errors
	/// `CsInvalidValue::Nan` when either `base` or `exponent` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when either `base` or `exponent` is not within its' `bounds` <br>
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
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when either `base` or `exponent` is `f64::NAN` <br>
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
	}
	
	#[inline]
	pub fn calculate ( &self, value: f64 ) -> f64 {
		value * self.multiplier
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `base` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `base` is not within `self.bounds_base` <br>
	#[inline]
	pub fn set_base ( &mut self, base: f64 ) -> Result<(), CharStatError > {
		BaseMultConf::check_inval( base, &self.bounds_base, "base".to_string() )?;
		
		self.base = base;
		self.update();
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `base` is `f64::NAN` <br>
	#[inline]
	pub fn set_base_clamping ( &mut self, base: f64 ) -> Result<(), CharStatError > {
		BaseMultConf::check_nan( base, "base".to_string() )?;
		
		self.base = base.clamp( self.bounds_base.min(), self.bounds_base.max() );
		self.update();
		
		Ok(())
	}
	
	/// attempts to increment `base` by 1.0
	/// 
	/// # Errors
	/// `CsInvalidValue::AboveMaximum` when `base` + 1.0 is above `self.bounds_base.max()` <br>
	#[inline]
	pub fn inc_base ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.set_base( self.base + new_val )
	}
	
	/// attempts to decrement `base` by 1.0
	/// 
	/// # Errors
	/// `CsInvalidValue::BelowMinimum` when `base` - 1.0 is below `self.bounds_base.min()` <br>
	#[inline]
	pub fn dec_base ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.set_base( self.base - new_val )
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `exponent` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `exponent` is not within `self.bounds_exponent` <br>
	#[inline]
	pub fn set_exponent ( &mut self, exponent: f64 ) -> Result<(), CharStatError > {
		BaseMultConf::check_inval( exponent, &self.bounds_exp, "exponent".to_string() )?;
		
		self.exponent = exponent;
		self.update();
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `exponent` is `f64::NAN` <br>
	#[inline]
	pub fn set_exponent_clamping ( &mut self, exponent: f64 ) -> Result<(), CharStatError > {
		BaseMultConf::check_nan( exponent, "exponent".to_string() )?;
		
		self.exponent = exponent.clamp( self.bounds_exp.min(), self.bounds_exp.max() );
		self.update();
		
		Ok(())
	}
	
	/// attempts to increment `exponent` by 1.0
	/// 
	/// # Errors
	/// `CsInvalidValue::AboveMaximum` when `exponent` + 1.0 is above `self.bounds_exponent.max()` <br>
	#[inline]
	pub fn inc_exp ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.set_exponent( self.exponent + new_val )
	}
	
	/// attempts to decrement `exponent` by 1.0
	/// 
	/// # Errors
	/// `CsInvalidValue::AboveMaximum` when `exponent` - 1.0 is below `self.bounds_exponent.min()` <br>
	#[inline]
	pub fn dec_exp ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.set_exponent( self.exponent - new_val )
	}
	
	#[inline]
	pub fn set_rounding ( &mut self, new_val: RoundingHelper ) {
		self.rounding_fn = new_val;
	}
}

//priv
impl BaseMultConf {
	#[inline(always)]
	#[doc(hidden)]
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
	#[doc(hidden)]
	fn check_nan( value: f64, name: String ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( name ) )?
		}
		
		Ok(())
	}
	
	#[inline(always)]
	#[doc(hidden)]
	fn update ( &mut self ) {
		let val = self.base.powf( self.exponent );
		let mlt = self.rounding_fn.do_rounding( val );
		
		self.multiplier = mlt;
	}
}// priv

// struct - BaseMultConf
//------------------------------------------------------------------------------
// --Tests

#[cfg(test)]
mod tests {
	use crate::{ BaseConf, BaseMultConf, Bounds, CharStatError, CsInvalidValue, RoundingHelper, RoundingFnEnum };
	
	#[test]
	fn constructors() {
		let bounds = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let rnd_hlp = RoundingHelper::new_none();
		
		// new
		let bad = BaseMultConf::new( -10.0, 0.0, bounds.clone(), bounds.clone(), rnd_hlp.clone() );
		let expected: CharStatError = CsInvalidValue::BelowMinimum( "base".to_string() ).into();
		assert_eq!( bad, Err( expected ) );
		
		let bad = BaseMultConf::new( 10.0, 0.0, bounds.clone(), bounds.clone(), rnd_hlp.clone() );
		let expected: CharStatError = CsInvalidValue::AboveMaximum( "base".to_string() ).into();
		assert_eq!( bad, Err( expected ) );
		
		let bad = BaseMultConf::new( 0.0, -10.0, bounds.clone(), bounds.clone(), rnd_hlp.clone() );
		let expected: CharStatError = CsInvalidValue::BelowMinimum( "exponent".to_string() ).into();
		assert_eq!( bad, Err( expected ) );
		
		let bad = BaseMultConf::new( 0.0, 10.0, bounds.clone(), bounds.clone(), rnd_hlp.clone() );
		let expected: CharStatError = CsInvalidValue::AboveMaximum( "exponent".to_string() ).into();
		assert_eq!( bad, Err( expected ) );
		
		// new_clamping
		let correct_min = BaseMultConf::new( 0.0, 0.0, bounds.clone(), bounds.clone(), rnd_hlp.clone() ).unwrap();
		let correct_max = BaseMultConf::new( 1.0, 1.0, bounds.clone(), bounds.clone(), rnd_hlp.clone() ).unwrap();
		
		let clamped = BaseMultConf::new_clamping( -10.0, -10.0, bounds.clone(), bounds.clone(), rnd_hlp.clone() );
		assert_eq!( clamped, Ok( correct_min ) );
		
		let clamped = BaseMultConf::new_clamping( 10.0, 10.0, bounds.clone(), bounds.clone(), rnd_hlp.clone() );
		assert_eq!( clamped, Ok( correct_max ) );
	}
	
	#[test]
	fn basic_functional() {
		let bounds_mlt_base = Bounds::new_const( 1.1, 1.1 ).unwrap();
		let bounds_mlt_exp = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let bounds_base = Bounds::new_const( 0.0, 10000.0 ).unwrap();
		
		let base_mult = BaseMultConf::new( 1.1, 1.0, bounds_mlt_base, bounds_mlt_exp, RoundingHelper::new_none() ).unwrap();
		let mut base = BaseConf::new( 500.0, true, bounds_base, RoundingHelper::new( RoundingFnEnum::Floor, None ), Some( base_mult ) ).unwrap();
		
		let expected = [ 500.0, 550.0, 605.0, 665.0, 732.0, 805.0, 885.0, 974.0, 1071.0, 1178.0, 1296.0 ];
		
		let mut i = 1.0;
		for idx in 1..=10 {
			base.set_mult_exponent( i ).unwrap();
			
			assert_eq!( base.value(), expected[ idx ] );
			
			i += 1.0;
		}
	}
	
	#[test]
	fn nan_handling() {
		let bounds = Bounds::new_const( 0.0, 1.0 ).unwrap();
		
		// base
		let base_mult = BaseMultConf::new( f64::NAN, 1.0, bounds.clone(), bounds.clone(), RoundingHelper::default() );
		let expected: CharStatError = CsInvalidValue::Nan( "base".to_string() ).into();
		assert_eq!( base_mult, Err( expected ) );
		
		// exponent
		let base_mult = BaseMultConf::new( 1.0, f64::NAN, bounds.clone(), bounds.clone(), RoundingHelper::default() );
		let expected: CharStatError = CsInvalidValue::Nan( "exponent".to_string() ).into();
		assert_eq!( base_mult, Err( expected ) );
	}
}

// --Tests
//------------------------------------------------------------------------------
