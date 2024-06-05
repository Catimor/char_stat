use std::fmt::Display;
use crate::{ CharStatError, CsLogicIssue, CsInvalidValue };

#[derive( Debug, Clone, PartialEq )]
pub struct Modifier {
	value: f64,
	exp_ts: Option< u64 >,
	
	mode: ModCalcModeEnum,
	stage: ModCalcStageEnum,
}// Modifier

impl Modifier {
	#[inline]
	pub fn new ( value: f64, exp_ts: Option< u64 >, mode: ModCalcModeEnum, stage: ModCalcStageEnum, ) -> Result< Self, CharStatError > {
		if value.is_nan() {
			
			return Err( CsInvalidValue::Nan( "value".to_string() ) )?
		}
		
		if ModCalcStageEnum::ModMult == stage {
			if let ModCalcModeEnum::Mul | ModCalcModeEnum::Div = mode {
				
				return Err( CsLogicIssue::InvalidModifierMode( mode ) )?
			}
		}
		
		Ok( Modifier {
			value,
			exp_ts,
			
			mode,
			stage,
		})
	}// new
	
	#[inline]
	pub fn has_expired ( &self, ts: u64 ) -> bool {
		if let Some( exp_ts ) = self.exp_ts {
			return ts >= exp_ts
		}
		
		false
	}
	
	#[inline]
	pub fn value ( &self ) -> f64 {
		self.value
	}
	
	#[inline]
	pub fn expiration_ts ( &self ) -> Option< u64 > {
		self.exp_ts
	}
	
	#[inline]
	pub fn calc_mode ( &self ) -> &ModCalcModeEnum {
		&self.mode
	}
	
	#[inline]
	pub fn stage ( &self ) -> &ModCalcStageEnum {
		&self.stage
	}
}// Modifier

//----------------------------------------------------
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum ModCalcModeEnum {
	Add,
	Sub,
	Mul,
	Div,
}// ModCalcModeEnum

impl Display for ModCalcModeEnum {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut tmp = String::from( "missing object: " );
		
		tmp.push_str( match self {
			Self::Add => "Add",
			Self::Sub => "Sub",
			Self::Mul => "Mul",
			Self::Div => "Div",
		} );
		
		tmp.fmt(f)
	}
}// ModCalcStageEnum - Display

//----------------------------------------------------
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum ModCalcStageEnum {
	Base,
	Upgrade,
	BasePlusUpgrade,
	ModMult,
}// ModCalcStageEnum

impl Display for ModCalcStageEnum {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut tmp = String::from( "missing object: " );
		
		tmp.push_str( match self {
			Self::Base => "Base",
			Self::Upgrade => "Upgrade",
			Self::BasePlusUpgrade => "BasePlusUpgrade",
			Self::ModMult => "ModMult",
		} );
		
		tmp.fmt(f)
	}
}// ModCalcStageEnum - Display

//----------------------------------------------------
#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn expired_modifiers() {
		// Has Expiration
		let mod_1 = Modifier::new( 1.5, Some( 50 ), ModCalcModeEnum::Mul, ModCalcStageEnum::Base ).unwrap();
		assert!( !mod_1.has_expired( 49 ) );
		assert!( mod_1.has_expired( 50 ) );
		assert!( mod_1.has_expired( 51 ) );
		
		// Never Expires
		let mod_2 = Modifier::new( 1.5, None, ModCalcModeEnum::Mul, ModCalcStageEnum::Base ).unwrap();
		assert!( !mod_2.has_expired( u64::MAX ) );
	}// expired_modifiers
	
	#[test]
	fn nan_handling() {
		let modif = Modifier::new( f64::NAN, None, ModCalcModeEnum::Mul, ModCalcStageEnum::Base );
		let expected: CharStatError = CsInvalidValue::Nan( "value".to_string() ).into();
		assert_eq!( modif, Err( expected ) );
	}// nan_handling
}
