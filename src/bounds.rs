use std::fmt::{ Display, Formatter };

#[cfg(feature = "serde")]
use serde::{ Serialize, Deserialize };

// --Imports
//------------------------------------------------------------------------------
// --Modules

use crate::{ CharStatError, CsLogicIssue, CsInvalidValue };

// --Modules
//------------------------------------------------------------------------------
// struct - Bounds

/// Manages the allowed min/max values and whether those are mutable.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, Copy, PartialEq,  )]
pub struct Bounds {
	v_min: f64,
	v_max: f64,
	is_min_mut: bool,
	is_max_mut: bool,
}

impl Bounds {
	/// # Errors
	/// `CsInvalidValue::Nan` when either `v_min` or `v_max` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `v_min` > `v_max` <br>
	#[inline]
	pub fn new ( v_min: f64, v_max: f64, is_min_mut: bool, is_max_mut: bool ) -> Result< Self, CharStatError > {
		Bounds::check_nan( v_min, "v_min".to_string() )?;
		Bounds::check_nan( v_max, "v_max".to_string() )?;
		Bounds::check_values( v_min, v_max )?;
		
		Ok( Bounds {
			v_min,
			v_max,
			is_min_mut,
			is_max_mut,
		})
	}
	
	/// mutability within setters disabled
	/// 
	/// # Errors
	/// `CsInvalidValue::Nan` when either `v_min` or `v_max` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `v_min` > `v_max` <br>
	#[inline]
	pub fn new_const ( v_min: f64, v_max: f64 ) -> Result< Self, CharStatError > {
		Bounds::check_nan( v_min, "v_min".to_string() )?;
		Bounds::check_nan( v_max, "v_max".to_string() )?;
		Bounds::check_values( v_min, v_max )?;
		
		Ok( Bounds {
			v_min,
			v_max,
			is_min_mut: false,
			is_max_mut: false,
		})
	}
	
	/// mutability within setters enabled
	/// 
	/// # Errors
	/// `CsInvalidValue::Nan` when either `v_min` or `v_max` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `v_min` > `v_max` <br>
	#[inline]
	pub fn new_mut ( v_min: f64, v_max: f64 ) -> Result< Self, CharStatError > {
		Bounds::check_nan( v_min, "v_min".to_string() )?;
		Bounds::check_nan( v_max, "v_max".to_string() )?;
		Bounds::check_values( v_min, v_max )?;
		
		Ok( Bounds {
			v_min,
			v_max,
			is_min_mut: true,
			is_max_mut: true,
		})
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `new_val` > `self.v_max` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.is_min_mut` is false <br>
	#[inline]
	pub fn set_min ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		if !self.is_min_mut {
			return Err( CsLogicIssue::FieldIsConst )?
		}
		
		Bounds::check_nan( new_val, "new_val".to_string() )?;
		Bounds::check_values( new_val, self.v_max )?;
		
		self.v_min = new_val;
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `self.v_min` > `new_val` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.is_max_mut` is false <br>
	#[inline]
	pub fn set_max ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		if !self.is_min_mut {
			return Err( CsLogicIssue::FieldIsConst )?
		}
		
		Bounds::check_nan( new_val, "new_val".to_string() )?;
		Bounds::check_values( self.v_min, new_val )?;
		
		self.v_max = new_val;
		
		Ok(())
	}
	
	/// `set_min` method will not be allowed (runtime check)
	#[inline]
	pub fn set_min_const ( &mut self ) {
		self.is_min_mut = false;
	}
	
	/// `set_max` method will not be allowed (runtime check)
	#[inline]
	pub fn set_max_const ( &mut self ) {
		self.is_max_mut = false;
	}
	
	#[inline]
	pub fn min ( &self ) -> f64 {
		self.v_min
	}
	
	#[inline]
	pub fn max ( &self ) -> f64 {
		self.v_max
	}
}

//priv
impl Bounds {
	#[inline(always)]
	#[doc(hidden)]
	fn check_nan( value: f64, name: String ) -> Result<(), CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( name ).into() )
		}
		
		Ok(())
	}
	
	#[inline(always)]
	#[doc(hidden)]
	fn check_values( min: f64, max: f64 ) -> Result<(), CharStatError > {
		if min > max {
			
			return Err( CsLogicIssue::MinGreaterThanMax.into() )
		}
		
		Ok(())
	}
}// priv

impl Display for Bounds {
	#[inline]
	fn fmt( &self, f: &mut Formatter<'_> ) -> std::fmt::Result {
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

// struct - Bounds
//------------------------------------------------------------------------------
// --Tests

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn nan_handling() {
		// Bounds - min
		let expected: CharStatError = CsInvalidValue::Nan( "v_min".to_string() ).into();
		let bad_min = Bounds::new_const( f64::NAN, 5.0, );
		assert_eq!( bad_min, Err( expected.clone() ) );
		let bad_min = Bounds::new_mut( f64::NAN, 5.0, );
		assert_eq!( bad_min, Err( expected.clone() ) );
		let bad_min = Bounds::new( f64::NAN, 5.0, true, false, );
		assert_eq!( bad_min, Err( expected.clone() ) );
		
		// Bounds - max
		let expected: CharStatError = CsInvalidValue::Nan( "v_max".to_string() ).into();
		let bad_max = Bounds::new_const( 5.0, f64::NAN, );
		assert_eq!( bad_max, Err( expected.clone() ) );
		let bad_max = Bounds::new_mut( 5.0, f64::NAN, );
		assert_eq!( bad_max, Err( expected.clone() ) );
		let bad_max = Bounds::new( 5.0, f64::NAN, true, false, );
		assert_eq!( bad_max, Err( expected ) );
	}
	
	#[test]
	fn mutability() {
		let expected: CharStatError = CsLogicIssue::FieldIsConst.into();
		let mut bounds_1 = Bounds::new_const( 1.0, 10.0, ).expect( "..." );
		
		assert_eq!( bounds_1.set_min( 5.0 ), Err( expected.clone() ) );
		assert_eq!( bounds_1.set_max( 5.0 ), Err( expected.clone() ) );
		
		let mut bounds_2 = Bounds::new_mut( 1.0, 10.0, ).expect( "..." );
		
		assert_eq!( bounds_2.set_min( 5.0 ), Ok(()) );
		assert_eq!( bounds_2.set_max( 5.0 ), Ok(()) );
		
		bounds_2.set_min_const();
		bounds_2.set_max_const();
		
		assert_eq!( bounds_2.set_min( 1.0 ), Err( expected.clone() ) );
		assert_eq!( bounds_2.set_max( 10.0 ), Err( expected ) );
	}
}

// --Tests
//------------------------------------------------------------------------------
