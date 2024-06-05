use super::{ Bounds, RoundingHelper, ModCalcModeEnum, ModCalcStageEnum, Modifier, CharStatError, CsLogicIssue };

#[derive(Clone, PartialEq, Debug)]
pub struct ModConf {
	value: f64,
	stage: ModCalcStageEnum,
	bounds: Bounds,
	rounding_fn: RoundingHelper,
	mod_vec: Vec< Modifier >,
	is_min_percent: bool,
	is_max_percent: bool,
}// ModConf

impl ModConf {
	#[inline]
	pub fn new ( stage: ModCalcStageEnum, bounds: Bounds, rounding_fn: RoundingHelper, is_min_percent: bool, is_max_percent: bool, ) -> Self {
		ModConf {
			value: 0.0,
			stage,
			bounds,
			rounding_fn,
			mod_vec: Vec::new(),
			is_min_percent,
			is_max_percent,
		}
	}// new
	
	#[inline]
	pub fn append_mod ( &mut self, value: f64, modifier: Modifier ) -> Result<(), CharStatError > {
		let stage = modifier.stage();
		if stage != &self.stage {
			return Err( CsLogicIssue::InvalidModifierStage( *stage ) )?
		}
		
		self.mod_vec.push( modifier );
		self.update( value );
		
		Ok(())
	}
	
	#[inline]
	pub fn get ( &self ) -> f64 {
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
	}// remove_expired
	
	#[inline]
	pub fn set_rounding ( &mut self, new_val: RoundingHelper ) {
		self.rounding_fn = new_val;
	}
}

// pub-crate
impl ModConf {
	#[inline]
	pub(crate) fn append_mod_unchecked ( &mut self, value: f64, modifier: Modifier ) {
		self.mod_vec.push( modifier );
		self.update( value );
	}
	
	#[inline]
	pub(crate) fn update ( &mut self, value: f64 ) {
		let mut tmp = 0.0;
		
		for el in &self.mod_vec {
			match el.calc_mode() {
				ModCalcModeEnum::Add => tmp += el.value(),
				ModCalcModeEnum::Sub => tmp -= el.value(),
				ModCalcModeEnum::Mul => tmp += el.value() * value,
				ModCalcModeEnum::Div => tmp += value / el.value(),
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
	}// update
}// ModConf


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn expired_modifiers() {
		let stage = ModCalcStageEnum::Base;
		let bounds = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let rounding = RoundingHelper::new_none();
		let mut mgr = ModConf::new( stage, bounds, rounding, false, false );
		
		let mod_1 = Modifier::new( 1.0, Some( 69 ), ModCalcModeEnum::Add, stage ).unwrap();
		let mod_2 = Modifier::new( 2.0, Some( 100 ), ModCalcModeEnum::Add, stage ).unwrap();
		
		mgr.append_mod_unchecked( 0.0, mod_1 );
		mgr.append_mod_unchecked( 0.0, mod_2 );
		
		mgr.remove_expired( 50 );
		mgr.update( 0.0 );
		assert_eq!( mgr.get(), 3.0 );
		
		mgr.remove_expired( 75 );
		mgr.update( 0.0 );
		assert_eq!( mgr.get(), 2.0 );
		
		mgr.remove_expired( 100 );
		mgr.update( 0.0 );
		assert_eq!( mgr.get(), 0.0 );
		
	}// expired_modifiers
}
