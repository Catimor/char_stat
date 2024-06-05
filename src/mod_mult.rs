use super::{ Bounds, ModCalcStageEnum, ModCalcModeEnum, Modifier, CharStatError, CsLogicIssue };

#[derive(Clone, PartialEq, Debug)]
pub struct ModMultConf {
	value: f64,
	bounds: Bounds,
	mod_vec: Vec< Modifier >,
}// ModConf

impl ModMultConf {
	#[inline]
	pub fn new ( bounds: Bounds ) -> Self {
		ModMultConf {
			value: 0.0,
			bounds,
			mod_vec: Vec::new(),
		}
	}// new
	
	#[inline]
	pub fn append_mod ( &mut self, modifier: Modifier ) -> Result<(), CharStatError> {
		let stage = modifier.stage();
		let mode = modifier.calc_mode();
		
		if stage != &ModCalcStageEnum::ModMult {
			return Err( CsLogicIssue::InvalidModifierStage( *stage ) )?
		}
		
		if let ModCalcModeEnum::Mul | ModCalcModeEnum::Div = mode {
			return Err( CsLogicIssue::InvalidModifierMode( *mode ) )?
		}
		
		self.mod_vec.push( modifier );
		self.update();
		
		Ok(())
	}// append_mod
	
	#[inline]
	pub(crate) fn append_mod_unchecked ( &mut self, modifier: Modifier ) {
		self.mod_vec.push( modifier );
		self.update();
	}
	
	#[inline]
	pub(crate) fn update ( &mut self ) {
		let mut tmp = 0.0;
		
		for el in &self.mod_vec {
			if let ModCalcModeEnum::Add = el.calc_mode() {
				tmp += el.value();
			} else if let ModCalcModeEnum::Sub = el.calc_mode() {
				tmp -= el.value();
			}
		}// for
		
		self.value = tmp.clamp( self.bounds.min(), self.bounds.max() );
		self.value += 1.0;// +1 cause it's a mult, here due to bounds check
	}// update
	
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
}// ModUpgradeConf

// bounds
impl ModMultConf {
	#[inline]
	pub fn set_bounds_min ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.bounds.set_min( new_val )?;
		
		Ok(())
	}// set_min
	
	#[inline]
	pub fn set_bounds_max ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.bounds.set_max( new_val )?;
		
		Ok(())
	}// set_max
	
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
}// ModMultConf.bounds: Bounds

