#[derive(Clone, PartialEq, Debug)]
pub struct Modifier {
	value: f64,
	exp_ts: Option< u64 >,
	
	mode: ModCalcModeEnum,
	stage: ModCalcStageEnum,
}// Modifier

impl Modifier {
	pub fn new ( value: f64, exp_ts: Option< u64 >, mut mode: ModCalcModeEnum, stage: ModCalcStageEnum, ) -> Option< Self > {
		if value.is_nan() {
			
			return None
		}
		
		if let ModCalcStageEnum::ModMult = stage {
			mode = ModCalcModeEnum::Add;
		}
		
		Some( Modifier {
			value,
			exp_ts,
			
			mode,
			stage,
		})
	}// new
	
	pub fn has_expired ( &self, ts: u64 ) -> bool {
		if let Some( exp_ts ) = self.exp_ts {
			return ts >= exp_ts
		}
		
		false
	}
	
	pub fn value ( &self ) -> f64 {
		self.value
	}
	
	pub fn expiration_ts ( &self ) -> Option< u64 > {
		self.exp_ts
	}
	
	pub fn calc_mode ( &self ) -> &ModCalcModeEnum {
		&self.mode
	}
	
	pub fn stage ( &self ) -> &ModCalcStageEnum {
		&self.stage
	}
}// Modifier

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModCalcModeEnum {
	Add,
	Sub,
	Mul,
	Div,
}// ModCalcModeEnum


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModCalcStageEnum {
	Base,
	//BaseMin,
	//BaseMax,
	Upgrade,
	BasePlusUpgrade,
	ModMult,
}// ModCalcStageEnum
	
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
		assert_eq!( modif, None );
	}// nan_handling
}
