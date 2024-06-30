#[cfg(feature = "serde")]
use serde::{ Serialize, Deserialize };

// --Imports
//------------------------------------------------------------------------------
// --Modules

use crate::{ Bounds, RoundingHelper, ModCalcMode, ModCalcStage, Modifier, CharStatError, CsLogicIssue, CsInvalidValue };

// --Modules
//------------------------------------------------------------------------------
// struct - ModConf

#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq,  )]
pub struct ModConf {
	value: f64,
	stage: ModCalcStage,
	bounds: Bounds,
	rounding_fn: RoundingHelper,
	mod_vec: Vec< Modifier >,
	is_min_percent: bool,
	is_max_percent: bool,
}

impl ModConf {
	#[inline]
	pub fn new ( stage: ModCalcStage, bounds: Bounds, rounding_fn: RoundingHelper, is_min_percent: bool, is_max_percent: bool, ) -> Self {
		ModConf {
			value: 0.0,
			stage,
			bounds,
			rounding_fn,
			mod_vec: Vec::new(),
			is_min_percent,
			is_max_percent,
		}
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsLogicIssue::InvalidModifierMode` when `modifier.calc_stage` is different from `self.stage` <br>
	#[inline]
	pub fn append_mod ( &mut self, value: f64, modifier: Modifier ) -> Result<(), CharStatError > {
		let stage = modifier.calc_stage();
		
		if stage != self.stage {
			return Err( CsLogicIssue::InvalidModifierStage( stage, self.stage ).into() )
		}
		
		if value.is_nan() {
			return Err( CsInvalidValue::Nan( "value".to_string() ).into() )
		}
		
		self.mod_vec.push( modifier );
		self.update( value );
		
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
	}
	
	#[inline]
	pub fn set_rounding ( &mut self, new_val: RoundingHelper ) {
		self.rounding_fn = new_val;
	}
}

// pub-crate
impl ModConf {
	#[inline]
	#[doc(hidden)]
	pub(crate) fn append_mod_unchecked ( &mut self, value: f64, modifier: Modifier ) {
		self.mod_vec.push( modifier );
		self.update( value );
	}
	
	#[inline]
	#[doc(hidden)]
	pub(crate) fn update ( &mut self, value: f64 ) {
		let mut tmp = 0.0;
		
		for el in &self.mod_vec {
			match el.calc_mode() {
				ModCalcMode::Add => tmp += el.value(),
				ModCalcMode::Sub => tmp -= el.value(),
				ModCalcMode::Mul => tmp += el.value() * value,
				ModCalcMode::Div => tmp += value / el.value(),
			}
		}// for
		
		let mut eff_min = self.bounds.min();
		let mut eff_max = self.bounds.max();
		
		if self.is_min_percent {
			eff_min *= value;
		}
		
		if self.is_max_percent {
			eff_max *= value;
		}
		
		tmp = self.rounding_fn.do_rounding( tmp );
		
		self.value = tmp.clamp( eff_min, eff_max );
	}
}// pub-crate

// bounds
impl ModConf {
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

// struct - ModConf
//------------------------------------------------------------------------------
// --Tests

#[cfg(test)]
mod tests {
	use crate::{ ModConf, Modifier, ModCommon, ModCalcStage, ModCalcMode, Bounds, RoundingHelper, CsInvalidValue,  };
	
	#[test]
	fn basic_functional() {
		// bounds are applied to the sum of modifiers' values, therefore the minimum bound must be <= 0.0, bacause the modifier list will start empty.
		let bounds_mgr = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let mut mod_mgr = ModConf::new( ModCalcStage::Base, bounds_mgr, RoundingHelper::new_none(), false, false );
		
		let common = ModCommon::new( 1.0, ModCalcMode::Add, ModCalcStage::Base ).unwrap();
		let modifier = Modifier::new_persistent( common );
		let base_value = 69.0;
		assert_eq!( mod_mgr.value(), 0.0 );
		
		mod_mgr.append_mod( base_value, modifier ).unwrap();
		assert_eq!( mod_mgr.value(), 1.0 );
	}
	
	#[test]
	fn nan_handling() {
		let bounds_mgr = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let mut mod_mgr = ModConf::new( ModCalcStage::Base, bounds_mgr, RoundingHelper::new_none(), false, false );
		
		let common = ModCommon::new( 1.0, ModCalcMode::Add, ModCalcStage::Base ).unwrap();
		let modifier = Modifier::new_persistent( common );
		
		let base_value = f64::NAN;
		let expected = CsInvalidValue::Nan( "value".to_string() ).into();
		
		assert_eq!( mod_mgr.append_mod( base_value, modifier ), Err( expected ) );
	}
	
	#[test]
	fn expired_modifiers() {
		let stage = ModCalcStage::Base;
		let mode = ModCalcMode::Add;
		let bounds = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let rounding = RoundingHelper::new_none();
		let mut mgr = ModConf::new( stage, bounds, rounding, false, false );
		
		let common = ModCommon::new( 1.0, mode, stage).unwrap();
		let common_2 = ModCommon::new( 2.0, mode, stage).unwrap();
		
		let mod_1 = Modifier::new_expiring( common, 69 );
		let mod_2 = Modifier::new_expiring( common_2, 100 );
		
		mgr.append_mod_unchecked( 0.0, mod_1 );
		mgr.append_mod_unchecked( 0.0, mod_2 );
		
		mgr.remove_expired( 50 );
		mgr.update( 0.0 );
		assert_eq!( mgr.value(), 3.0 );
		
		mgr.remove_expired( 75 );
		mgr.update( 0.0 );
		assert_eq!( mgr.value(), 2.0 );
		
		mgr.remove_expired( 100 );
		mgr.update( 0.0 );
		assert_eq!( mgr.value(), 0.0 );
		
	}
}

// --Tests
//------------------------------------------------------------------------------
