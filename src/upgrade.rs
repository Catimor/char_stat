#[cfg( feature = "serde" )]
use serde::{ Serialize, Deserialize };

// --Imports
//------------------------------------------------------------------------------
// --Modules

use super::{ Bounds, RoundingHelper, CharStatError, CsInvalidValue };

// --Modules
//------------------------------------------------------------------------------
// struct - UpgradeConf

/// Manages upgrade value.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq,  )]
pub struct UpgradeConf {
	value: f64,
	bounds: Bounds,
	rounding_fn: RoundingHelper,
}

impl UpgradeConf {
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `value` is not within `bounds` <br>
	#[inline]
	pub fn new ( mut value: f64, bounds: Bounds, rounding_fn: RoundingHelper, ) -> Result< Self, CharStatError > {
		UpgradeConf::check_inval( value, &bounds )?;
		
		value = rounding_fn.do_rounding( value );
		
		Ok( UpgradeConf {
			value,
			bounds,
			rounding_fn,
		})
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	#[inline]
	pub fn new_clamping ( mut value: f64, bounds: Bounds, rounding_fn: RoundingHelper, ) -> Result< Self, CharStatError > {
		UpgradeConf::check_nan( value )?;
		
		value = rounding_fn.do_rounding( value );
		value = value.clamp( bounds.min(), bounds.max() );
		
		Ok( UpgradeConf {
			value,
			bounds,
			rounding_fn,
		})
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `value` is not within `self.bounds` <br>
	#[inline]
	pub fn set_value ( &mut self, mut value: f64 ) -> Result<(), CharStatError > {
		UpgradeConf::check_inval( value, &self.bounds )?;
		
		value = self.rounding_fn.do_rounding( value );
		
		self.value = value;
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	#[inline]
	pub fn set_value_clamping ( &mut self, mut value: f64 ) -> Result<(), CharStatError > {
		UpgradeConf::check_nan( value )?;
		
		value = self.rounding_fn.do_rounding( value );
		
		self.value = value.clamp( self.bounds.min(), self.bounds.max() );
		
		Ok(())
	}
	
	#[inline]
	pub fn value ( &self ) -> f64 {
		self.value
	}
}

// bounds
impl UpgradeConf {
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

//priv
impl UpgradeConf {
	#[inline( always )]
	#[doc( hidden )]
	fn check_inval( value: f64, bounds: &Bounds ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( "value".to_string() ).into() )
		}
		if value < bounds.min() {
			
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

// struct - UpgradeConf
//------------------------------------------------------------------------------
// --Tests

#[cfg( test )]
mod tests {
	use super::*;
	
	#[test]
	fn basic_functional() {
		let rounding_fn = RoundingHelper::new_none();
		let bounds_upgr = Bounds::new_const( 0.0, 50.0 ).unwrap();
		let mut upgrade = UpgradeConf::new( 0.0, bounds_upgr, rounding_fn ).unwrap();
		
		let expected: CharStatError = CsInvalidValue::AboveMaximum( "value".to_string() ).into();
		
		assert_eq!( upgrade.set_value( 69.0 ), Err( expected ) );
		assert_eq!( upgrade.value(), 0.0 );
		
		assert_eq!( upgrade.set_value( 5.0 ), Ok(()) );
	}
	
	#[test]
	fn nan_handling() {
		let expected: CharStatError = CsInvalidValue::Nan( "value".to_string() ).into();
		let rounding_fn = RoundingHelper::new_none();
		let bounds = Bounds::new_const( 0.0, 1.0 ).unwrap();
		
		let upgrade = UpgradeConf::new( f64::NAN, bounds.clone(), rounding_fn.clone() );
		assert_eq!( upgrade, Err( expected.clone() ) );
		
		let upgrade = UpgradeConf::new_clamping( f64::NAN, bounds.clone(), rounding_fn.clone() );
		assert_eq!( upgrade, Err( expected ) );
	}
}

// --Tests
//------------------------------------------------------------------------------
