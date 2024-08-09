#[cfg( feature = "serde" )]
use serde::{ Serialize, Deserialize };

// --Imports
//------------------------------------------------------------------------------
// --Modules

use super::{ Bounds, ModCalcStage, ModCalcMode, Modifier, CharStatError, CsLogicIssue };

// --Modules
//------------------------------------------------------------------------------
// struct - ModMultConf

/// Component handling modifier multiplier.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq,  )]
pub struct ModMultConf {
	value: f64,
	bounds: Bounds,
	mod_vec: Vec< Modifier >,
}

impl ModMultConf {
	#[inline]
	pub fn new ( bounds: Bounds ) -> Self {
		ModMultConf {
			value: 1.0,
			bounds,
			mod_vec: Vec::new(),
		}
	}
	
	/// # Errors
	/// `CsLogicIssue::InvalidModifierMode` when `stage` is not `ModMult` or `mode` is `Mul` or `Div` <br>
	#[inline]
	pub fn append_mod ( &mut self, modifier: Modifier ) -> Result<(), CharStatError> {
		let stage = modifier.calc_stage();
		let mode = modifier.calc_mode();
		
		if stage != ModCalcStage::ModMult {
			return Err( CsLogicIssue::InvalidModifierStage( stage, ModCalcStage::ModMult ).into() )
		}
		
		if let ModCalcMode::Mul | ModCalcMode::Div = mode {
			return Err( CsLogicIssue::InvalidModifierMode( mode, vec![ ModCalcMode::Add, ModCalcMode::Sub ] ).into() )
		}
		
		self.append_mod_unchecked( modifier );
		
		Ok(())
	}
	
	#[inline]
	pub fn value ( &self ) -> f64 {
		self.value
	}
	
	#[inline]
	pub fn remove_expired ( &mut self, ts: u64 ) {
		for i in ( 0..self.mod_vec.len() ).rev() {
			let tmp = self.mod_vec.get( i );
			
			if let Some( element ) = tmp {
				if element.has_expired( ts ) {
					self.mod_vec.remove( i );
				}
			}
		}// for
		
		self.update();
	}
}

// pub-crate
impl ModMultConf {
	#[inline]
	pub( crate ) fn append_mod_unchecked ( &mut self, modifier: Modifier ) {
		self.mod_vec.push( modifier );
		self.update();
	}
	
	#[inline]
	pub( crate ) fn update ( &mut self ) {
		let mut tmp = 0.0;
		
		for el in &self.mod_vec {
			if let ModCalcMode::Add = el.calc_mode() {
				tmp += el.value();
			} else {// if let ModCalcMode::Sub = el.calc_mode() {
				tmp -= el.value();
			}
		}// for
		
		self.value = tmp.clamp( self.bounds.min(), self.bounds.max() );
		self.value += 1.0;// +1 cause it's a mult, here due to bounds check
	}
}// pub-crate

// bounds
impl ModMultConf {
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

// struct - ModMultConf
//------------------------------------------------------------------------------
// --Tests

#[cfg( test )]
mod tests {
	use crate::{ ModMultConf, Bounds, Modifier, ModCommon, ModCalcMode, ModCalcStage };
	
	#[test]
	fn basic_functional() {
		// bounds are applied to the sum of modifiers' values, therefore the minimum bound must be <= 0.0, bacause the modifier list will start empty.
		let bounds_mlt = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let mut mod_mult = ModMultConf::new( bounds_mlt );
		
		let common = ModCommon::new( 1.0, ModCalcMode::Add, ModCalcStage::ModMult ).unwrap();
		let modifier = Modifier::new_expiring( common, 9 );
		
		// the returned value is incremented by 1.0 to turn it into multiplier
		assert_eq!( mod_mult.value(), 1.0 );
		
		mod_mult.append_mod( modifier ).unwrap();
		assert_eq!( mod_mult.value(), 2.0 );
		
		mod_mult.remove_expired( 100 );
		assert_eq!( mod_mult.value(), 1.0 );
	}
}

// --Tests
//------------------------------------------------------------------------------
