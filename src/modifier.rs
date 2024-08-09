use std::fmt::{ Display, Formatter };

#[cfg( feature = "serde" )]
use serde::{ Serialize, Deserialize };

// --Imports
//------------------------------------------------------------------------------
// --Modules

use crate::{ CharStatError, CsLogicIssue, CsInvalidValue };

// --Modules
//------------------------------------------------------------------------------
// struct - Modifier

/// An instance of a modifier.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq,  )]
pub struct Modifier {
	common: ModCommon,
	mod_type: ModType,
}

impl Modifier {
	#[inline]
	pub fn new_expiring ( common: ModCommon, exp_ts: u64 ) -> Modifier {
		let v_data = ModType::Expiring { exp_ts };
		
		Self{ common, mod_type: v_data }
	}
	
	#[inline]
	pub fn new_persistent ( common: ModCommon ) -> Modifier {
		Self{ common, mod_type: ModType::Persistent }
	}
	
	#[inline]
	pub fn new_stacked ( common: ModCommon, conf: ModStackConf ) -> Modifier {
		let v_data = ModType::Stacked { conf: Box::new( conf ) };
		
		Self{ common, mod_type: v_data }
	}
	
	#[inline]
	pub fn has_expired ( &self, ts: u64 ) -> bool {
		if let ModType::Expiring { exp_ts } = self.mod_type {
			return ts >= exp_ts
		}
		
		false 
	}
	
	#[inline]
	pub fn expiration_ts ( &self ) -> Option< u64 > {
		if let ModType::Expiring { exp_ts } = self.mod_type {
			return Some( exp_ts )
		}
		
		None
	}
	
	#[inline]
	pub fn value ( &self ) -> f64 {
		self.common.value
	}
	
	#[inline]
	pub fn calc_mode ( &self ) -> ModCalcMode {
		self.common.mode
	}
	
	#[inline]
	pub fn calc_stage ( &self ) -> ModCalcStage {
		self.common.stage
	}
	
	/// # Errors
	/// CsLogicIssue::InvalidModifierType( ModType ) when `self.mod_type` is not `::Stackable`
	#[inline]
	pub fn stack ( &self ) -> Result< u32, CharStatError > {
		if let ModType::Stacked{ conf } = &self.mod_type {
			return Ok( conf.stack_value )
		}
		
		Err( CsLogicIssue::InvalidModifierType( self.mod_type.clone(), "Stacked".to_string() ).into() )
	}
	
	/// # Errors
	/// CsInvalidValue::AboveMaximum( ... ) when `conf.stack_value` >= `conf.stack_max`
	/// CsLogicIssue::InvalidModifierType( ModType ) when `self.mod_type` is not `::Stackable`
	#[inline]
	pub fn stack_inc ( &mut self ) -> Result< (), CharStatError > {
		if let ModType::Stacked{ ref mut conf } = self.mod_type {
			if conf.stack_value >= conf.stack_max {
				
				return Err( CsInvalidValue::AboveMaximum( "stack_value".to_string() ).into() )
			}
			
			conf.stack_value += 1;
			
			return Ok(())
		}
		
		Err( CsLogicIssue::InvalidModifierType( self.mod_type.clone(), "Stacked".to_string() ).into() )
	}
	
	/// # Errors
	#[inline]
	pub fn stack_dec ( &mut self ) -> Result< (), CharStatError > {
		if let ModType::Stacked{ ref mut conf } = self.mod_type {
			if conf.stack_value == 0 {
				
				return Err( CsInvalidValue::BelowMinimum( "stack_value".to_string() ).into() )
			}
			
			conf.stack_value -= 1;
			
			return Ok(())
		}
		
		Err( CsLogicIssue::InvalidModifierType( self.mod_type.clone(), "Stacked".to_string() ).into() )
	}
	
	/// # Errors
	#[inline]
	pub fn update_stack_ts ( &mut self, ts: u64 ) -> Result< (), CharStatError > {
		if let ModType::Stacked { ref mut conf } = self.mod_type {
			if conf.last_ts > ts {
				
				return Err( CsLogicIssue::TimeTravel.into() )
			}
			
			if conf.last_ts == ts {
				
				return Ok(())
			}
			
			let diff = ts - conf.last_ts;
			let stacks_to_clear = u32::try_from( diff / conf.duration );
			
			let stacks_to_clear = if let Ok( value ) = stacks_to_clear {
				value
			} else {
				u32::MAX
			};
			
			conf.stack_value = conf.stack_value.saturating_sub( stacks_to_clear );
			
			return Ok(())
		}
		
		Err( CsLogicIssue::InvalidModifierType( self.mod_type.clone(), "Stacked".to_string() ).into() )
	}
}

// struct - Modifier
//------------------------------------------------------------------------------
// enum - ModType

#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq, Eq,  )]
pub enum ModType {
	Expiring{ exp_ts: u64 },
	Persistent,
	Stacked{ conf: Box< ModStackConf > },
}

impl Display for ModType {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let tmp = match self {
			Self::Expiring{ .. } => "Expiring",
			Self::Persistent => "Persistent",
			Self::Stacked{ .. } => "Stacked",
		};
		
		tmp.fmt(f)
	}
}

// enum - ModType
//------------------------------------------------------------------------------
// struct - ModCommon

#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, Copy, PartialEq,  )]
pub struct ModCommon {
	value: f64,
	
	mode: ModCalcMode,
	stage: ModCalcStage,
}

impl ModCommon {
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// CsLogicIssue::InvalidModifierMode( ... ) when `stage` is `::ModMult` and `mode` is `::Mul` or `::Div`
	#[inline]
	pub fn new ( value: f64, mode: ModCalcMode, stage: ModCalcStage, ) -> Result< Self, CharStatError > {
		if value.is_nan() {
			return Err( CsInvalidValue::Nan( "value".to_string() ).into() )
		}
		
		if ModCalcStage::ModMult == stage {
			if let ModCalcMode::Mul | ModCalcMode::Div = mode {
				
				return Err( CsLogicIssue::InvalidModifierMode( mode, vec![ ModCalcMode::Add, ModCalcMode::Sub ] ).into() )
			}
		}
		
		Ok( ModCommon {
			value,
			mode,
			stage,
		})
	}
}

// struct - ModCommon
//------------------------------------------------------------------------------
// struct - ModStackConf

/// Configguration for a stackable modifier.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub struct ModStackConf {
	last_ts: u64,
	duration: u64,
	
	stack_value: u32,
	stack_max: u32,
}

impl ModStackConf {
	/// # Errors
	/// `CsInvalidValue::CannotBeZero` when `duration == 0` <br>
	/// `CsInvalidValue::AboveMaximum` when `stack_value > stack_max` <br>
	#[inline]
	pub fn new ( last_ts: u64, duration: u64, stack_value: u32, stack_max: u32, ) -> Result< Self, CharStatError > {
		if duration == 0 {
			return Err( CsInvalidValue::CannotBeZero( "duration".to_string() ).into() )
		}
		
		if stack_value >= stack_max {
			return Err( CsInvalidValue::AboveMaximum( "stack_value".to_string() ).into() )
		}
		
		Ok( ModStackConf {
			last_ts,
			duration,
			
			stack_value,
			stack_max,
		} )
	}
}

// struct - ModStackConf
//------------------------------------------------------------------------------
// enum - ModCalcMode

/// Calculation Mode:
/// - Add | Sub => value of modifier is added / substracted from the total,
/// - Mul => adds to the total a result of multiplying base by modifier,
/// - Div => adds to the total a result of dividing base by modifier,
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum ModCalcMode {
	Add,
	Sub,
	Mul,
	Div,
}

impl Display for ModCalcMode {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let tmp = match self {
			Self::Add => "Add",
			Self::Sub => "Sub",
			Self::Mul => "Mul",
			Self::Div => "Div",
		};
		
		tmp.fmt(f)
	}
}

// enum - ModCalcMode
//------------------------------------------------------------------------------
// enum - ModCalcStage

/// Calculation Stage.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum ModCalcStage {
	Base,
	Upgrade,
	BasePlusUpgrade,
	ModMult,
}

impl Display for ModCalcStage {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let tmp = match self {
			Self::Base => "Base",
			Self::Upgrade => "Upgrade",
			Self::BasePlusUpgrade => "BasePlusUpgrade",
			Self::ModMult => "ModMult",
		};
		
		tmp.fmt(f)
	}
}

// enum - ModCalcStage
//------------------------------------------------------------------------------
// --Tests

#[cfg( test )]
mod tests {
	use super::*;
	
	#[test]
	fn expired_modifiers() {
		let common = ModCommon::new( 1.5, ModCalcMode::Mul, ModCalcStage::Base ).unwrap();
		let mod_1 = Modifier::new_expiring( common, 50 );
		
		assert!( !mod_1.has_expired( 49 ) );
		assert!( mod_1.has_expired( 50 ) );
		assert!( mod_1.has_expired( 51 ) );
		
		let mod_2 = Modifier::new_persistent( common );
		
		let stack = ModStackConf::new( 0, 160, 0, 2 ).unwrap();
		let mod_3 = Modifier::new_stacked( common, stack );
		
		let _v = vec![ mod_1, mod_2, mod_3 ];
	}
	
	#[test]
	fn nan_handling() {
		let common = ModCommon::new( f64::NAN, ModCalcMode::Mul, ModCalcStage::Base );
		
		let expected: CharStatError = CsInvalidValue::Nan( "value".to_string() ).into();
		assert_eq!( common, Err( expected ) );
	}
	
	#[test]
	fn invalid_calc_for_modmult_stage() {
		let add = ModCalcMode::Add;
		let sub = ModCalcMode::Sub;
		let mul = ModCalcMode::Mul;
		let div = ModCalcMode::Div;
		
		let common = ModCommon::new( 1.5, mul, ModCalcStage::ModMult );
		let expected: CharStatError = CsLogicIssue::InvalidModifierMode( mul, vec![ add, sub ] ).into();
		assert_eq!( common, Err( expected ) );
		
		let common = ModCommon::new( 1.5, div, ModCalcStage::ModMult );
		let expected: CharStatError = CsLogicIssue::InvalidModifierMode( div, vec![ add, sub ] ).into();
		assert_eq!( common, Err( expected ) );
	}
	
	#[test]
	fn stacked_mod() {
		let common = ModCommon::new( 2.0, ModCalcMode::Add, ModCalcStage::Base ).unwrap();
		let conf = ModStackConf::new( 0, 16, 0, 2 ).unwrap();
		let mut modif = Modifier::new_stacked( common, conf );
		
		assert_eq!( modif.stack_inc(), Ok(()) );
		assert_eq!( modif.stack(), Ok( 1 ) );
		assert_eq!( modif.stack_inc(), Ok(()) );
		assert_eq!( modif.stack(), Ok( 2 ) );
		
		let expected: CharStatError = CsInvalidValue::AboveMaximum( "stack_value".to_string() ).into();
		assert_eq!( modif.stack_inc(), Err( expected)  );
		
		assert_eq!( modif.update_stack_ts( 20 ), Ok(()) );
		assert_eq!( modif.stack(), Ok( 1 ) );
		assert_eq!( modif.update_stack_ts( 500 ), Ok(()) );
		assert_eq!( modif.stack(), Ok( 0 ) );
		
		let expected: CharStatError = CsInvalidValue::BelowMinimum( "stack_value".to_string() ).into();
		assert_eq!( modif.stack_dec(), Err( expected ) );
	}
}

// --Tests
//------------------------------------------------------------------------------
