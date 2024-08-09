#[cfg( feature = "serde" )]
use serde::{ Serialize, Deserialize };

// --Imports
//------------------------------------------------------------------------------
// --Modules

use crate::{ Bounds, BaseMultConf, RoundingHelper, CharStatError, CsInvalidValue, CsLogicIssue, CsMissingComponent };

// --Modules
//------------------------------------------------------------------------------
// struct - BaseConf

/// Manages the base value, whether it's mutable, its bounds, rounding and optional multiplier.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq,  )]
pub struct BaseConf {
	value: f64,
	is_mut: bool,
	bounds: Bounds,
	rounding_fn: RoundingHelper,
	mult: Option< Box< BaseMultConf > >,
}

impl BaseConf {
	/// constructor
	/// 
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `value` is not within `bounds` <br>
	#[inline]
	pub fn new ( value: f64, is_mut: bool, bounds: Bounds, rounding_fn: RoundingHelper, mult: Option< BaseMultConf > ) -> Result< Self, CharStatError > {
		BaseConf::check_inval( value, &bounds )?;
		
		Ok( BaseConf {
			value,
			is_mut,
			bounds,
			rounding_fn,
			mult: mult.map( Box::new ),
		})
	}
	
	/// constructor <br>
	/// if `value` is outside the `bounds` then it's set to nearest valid value.
	/// 
	/// # Errors
	/// CsInvalidValue::Nan when `value` is `f64::NAN` <br>
	#[inline]
	pub fn new_clamping ( value: f64, is_mut: bool, bounds: Bounds, rounding_fn: RoundingHelper, mult: Option< BaseMultConf > ) -> Result< Self, CharStatError > {
		BaseConf::check_nan( value )?;
		
		Ok( BaseConf {
			value: value.clamp( bounds.min(), bounds.max() ),
			is_mut,
			bounds,
			rounding_fn,
			mult: mult.map( Box::new ),
		})
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `value` is not within `self.bounds` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.is_mut` is false <br>
	#[inline]
	pub fn set_value ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		if !self.is_mut {
			
			return Err( CsLogicIssue::FieldIsConst.into() )
		}
		BaseConf::check_inval( value, &self.bounds )?;
		
		self.value = value;
		
		Ok(())
	}
	
	/// if `value` is outside the `self.bounds` then it's set to nearest valid value.
	/// 
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.is_mut` is false <br>
	#[inline]
	pub fn set_value_clamping ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		if !self.is_mut {
			
			return Err( CsLogicIssue::FieldIsConst.into() )
		}
		BaseConf::check_nan( value )?;
		
		self.value = value.clamp( self.bounds.min(), self.bounds.max() );
		
		Ok(())
	}
	
	#[inline]
	pub fn value ( &self ) -> f64 {
		let mut out = self.value;
		
		if let Some( mlt ) = &self.mult {
			out = mlt.calculate( out );
		}
		
		self.rounding_fn.do_rounding( out )
	}
	
	/// disables mutability of `self.value`
	#[inline]
	pub fn set_value_const ( &mut self ) {
		self.is_mut = false;
	}
	
	#[inline]
	pub fn set_rounding ( &mut self, new_val: RoundingHelper ) {
		self.rounding_fn = new_val;
	}
}

// bounds
impl BaseConf {
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `new_val` > `self.v_max` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.bounds.is_min_mut` is false <br>
	#[inline]
	pub fn set_bounds_min ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.bounds.set_min( new_val )?;
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `self.v_min` > `new_val` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.bounds.is_max_mut` is false <br>
	#[inline]
	pub fn set_bounds_max ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.bounds.set_max( new_val )?;
		
		Ok(())
	}
	
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
}// bounds

// mult
impl BaseConf {
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `new_val` is not within `self.bounds` <br>
	/// `CsMissingComponent::BaseMult` when `BaseMultConf` is missing <br>
	#[inline]
	pub fn set_mult_base ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		if let Some( mlt ) = &mut self.mult {
			mlt.set_base( new_val )?;
			
			return Ok(())
		}
		
		Err( CsMissingComponent::BaseMult.into() )
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `new_val` is not within `self.bounds` <br>
	/// `CsMissingComponent::BaseMult` when `BaseMultConf` is missing <br>
	#[inline]
	pub fn set_mult_exponent ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		if let Some( mlt ) = &mut self.mult {
			mlt.set_exponent( new_val )?;
			
			return Ok(())
		}
		
		Err( CsMissingComponent::BaseMult.into() )
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsMissingComponent::BaseMult` when `BaseMultConf` is missing <br>
	#[inline]
	pub fn set_mult_base_clamping ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		if let Some( mlt ) = &mut self.mult {
			mlt.set_base_clamping( new_val )?;
			
			return Ok(())
		}
		
		Err( CsMissingComponent::BaseMult.into() )
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsMissingComponent::BaseMult` when `BaseMultConf` is missing <br>
	#[inline]
	pub fn set_mult_exponent_clamping ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		if let Some( mlt ) = &mut self.mult {
			mlt.set_exponent_clamping( new_val )?;
			
			return Ok(())
		}
		
		Err( CsMissingComponent::BaseMult.into() )
	}
}// mult

//priv
impl BaseConf {
	#[inline( always )]
	#[doc( hidden )]
	fn check_inval( value: f64, bounds: &Bounds ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( "value".to_string() ).into() )
		}
		if value < bounds.min() {
			
			//return Err( CsInvalidValue::BelowMinimum( "value".to_string() ).into() )
			return Err( CsInvalidValue::BelowMinimum( "value".to_string() ).into() )
		}
		if value > bounds.max() {
			
			return Err( CsInvalidValue::AboveMaximum( "value".to_string() ).into() )
		}
		
		Ok(())
	}
	
	#[inline( always )]
	#[doc( hidden )]
	fn check_nan( value: f64 ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( "value".to_string() ).into() )
		}
		
		Ok(())
	}
}// priv

// struct - BaseConf
//------------------------------------------------------------------------------
// --Tests

#[cfg( test )]
mod tests {
	use crate::{ BaseConf, Bounds, RoundingFnEnum, RoundingHelper, CharStatError, CsInvalidValue };
	
	#[test]
	fn basic_functional() {
		let rounding_helper = RoundingHelper::new( RoundingFnEnum::None, None );
		
		let bounds = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let base = BaseConf::new( 15.0, true, bounds, rounding_helper.clone(), None );
		
		let expected: CharStatError = CsInvalidValue::AboveMaximum( "value".to_string() ).into();
		assert_eq!( base, Err( expected ) );
		
		let bounds = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let base = BaseConf::new_clamping( 15.0, true, bounds, rounding_helper, None );
		assert!( base.is_ok() );
		
		let mut base = base.unwrap();
		assert_eq!( base.value(), 10.0 );
		
		assert_eq!( base.set_value( 5.0 ), Ok(()) );
		assert_eq!( base.value(), 5.0 );
	}
	
	#[test]
	fn nan_handling() {
		let expected: CharStatError = CsInvalidValue::Nan( "value".to_string() ).into();
		let bounds = Bounds::new_const( 0.0, 1.0 ).unwrap();
		
		let base = BaseConf::new( f64::NAN, true, bounds.clone(), RoundingHelper::default(), None );
		assert_eq!( base, Err( expected.clone() ) );
		let base = BaseConf::new_clamping( f64::NAN, true, bounds.clone(), RoundingHelper::default(), None );
		assert_eq!( base, Err( expected ) );
	}
}

// --Tests
//------------------------------------------------------------------------------
