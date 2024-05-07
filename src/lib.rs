//#![allow(unused_variables)]
//#![allow(dead_code)]

//use num;

//pub mod builder;
//use builder::*;

pub mod bounds;
use bounds::*;

pub mod modifier;
use modifier::*;

mod mod_mgr;
use mod_mgr::*;

pub mod base_conf;
use base_conf::*;

pub mod base_mult_conf;
use base_mult_conf::*;

pub mod upgrade_conf;
use upgrade_conf::*;

pub mod mod_mult;
use mod_mult::*;

#[derive(Clone, PartialEq, Debug)]
pub struct CharStat {
	current_value: f64,
	time_stamp: u64,
	
	val_base: f64,
	val_base_mod: f64,
	val_upgrade: f64,
	val_upgrade_mod: f64,
	val_base_plus_upgrade_mod: f64,
	val_mod_mult: f64,
	
	base:											BaseConf,
	upgrade:									Option< UpgradeConf >,
	//mod_of_base_bounds:				Option< ModConf >,
	mod_of_base:							Option< ModConf >,
	mod_of_upgrade:						Option< ModConf >,
	mod_of_base_plus_upgrade:	Option< ModConf >,
	mod_mult:									Option< ModMultConf >,
}// CharStat

impl CharStat {
	pub fn new (
		base: BaseConf, 
		upgrade: Option< UpgradeConf >, 
		mod_of_base: Option< ModConf >, 
		mut mod_of_upgrade: Option< ModConf>, 
		mut mod_of_base_plus_upgrade: Option< ModConf >, 
		mut mod_mult: Option< ModMultConf >, 
	) -> Self {
		// modifiers
		let mut tmp = 0;
		
		if let ( None, Some(_) ) = ( &upgrade, &mod_of_upgrade ) {
			mod_of_upgrade = None;
		} else {
			tmp += 1;
		}
		
		if let ( None, Some(_) ) = ( &upgrade, &mod_of_base_plus_upgrade ) {
			mod_of_base_plus_upgrade = None;
		} else {
			tmp += 1;
		}
		
		if let Some(_) = &mod_of_base {
			tmp += 1;
		}
		
		if tmp == 0 {
			mod_mult = None;
		}
		// end modifiers
		
		let mut out = CharStat {
			current_value: 0.0,
			time_stamp: 0,
			
			val_base: 0.0,
			val_base_mod: 0.0,
			val_upgrade: 0.0,
			val_upgrade_mod: 0.0,
			val_base_plus_upgrade_mod: 0.0,
			val_mod_mult: 1.0,
			
			base,
			upgrade,
			mod_of_base,
			mod_of_upgrade,
			mod_of_base_plus_upgrade,
			mod_mult,
		};
		
		out.update_base();
		out.update_upgrade();
		out.update_current_value();
		
		return out
	}// new
	
	pub fn new_minimal ( base: BaseConf ) -> Self {
		CharStat {
			current_value: 0.0,
			time_stamp: 0,
			
			val_base: 0.0,
			val_base_mod: 0.0,
			val_upgrade: 0.0,
			val_upgrade_mod: 0.0,
			val_base_plus_upgrade_mod: 0.0,
			val_mod_mult: 1.0,
			
			base,
			upgrade: None,
			mod_of_base: None,
			mod_of_upgrade: None,
			mod_of_base_plus_upgrade: None,
			mod_mult: None,
		}
	}// new_minimal
	
	pub fn new_no_mod ( base: BaseConf, upgrade: Option< UpgradeConf > ) -> Self {
		CharStat {
			current_value: 0.0,
			time_stamp: 0,
			
			val_base: 0.0,
			val_base_mod: 0.0,
			val_upgrade: 0.0,
			val_upgrade_mod: 0.0,
			val_base_plus_upgrade_mod: 0.0,
			val_mod_mult: 1.0,
			
			base,
			upgrade,
			mod_of_base: None,
			mod_of_upgrade: None,
			mod_of_base_plus_upgrade: None,
			mod_mult: None,
		}
	}// new_minimal
	
	pub fn value ( &self ) -> f64 {
		self.current_value
	}// value
	
	pub fn set_ts( &mut self, new_val: u64 ) -> Result<(),()> {
		if new_val < self.time_stamp {
			return Err(())
		}
		
		self.time_stamp = new_val;
		
		self.remove_expired_modifiers();
		
		Ok(())
	}
	
	pub fn set_ts_unchecked( &mut self, new_val: u64 ) {
		self.time_stamp = new_val;
		
		self.remove_expired_modifiers();
	}
	
	pub fn new_modifier( &mut self, modifier: Modifier ) -> Result<(),()> {
		match &modifier.stage() {
			ModCalcStageEnum::Base => self.append_base_mod( modifier ),//self.mod_of_base_minmax.push( modifier ),
			//ModCalcStageEnum::BaseMin => todo!(),//self.base_min.push( modifier ),
			//ModCalcStageEnum::BaseMax => todo!(),//self.base_max.push( modifier ),
			ModCalcStageEnum::Upgrade => self.append_upgrade_mod( modifier ),
			ModCalcStageEnum::BasePlusUpgrade => self.append_base_plus_upgrade_mod( modifier ),//self.base_plus_upgrade.push( modifier ),
			ModCalcStageEnum::ModMult => self.append_modmult( modifier ),//self.mod_mult.push( modifier ),
		}?;
		
		self.update_current_value();
		Ok(())
	}
	
	pub fn base ( &self ) -> f64 {
		if let Some(_) = &self.mod_of_base {
			return self.val_base + self.val_base_mod
		}
		self.val_base
	}
	
	pub fn base_raw ( &self ) -> f64 {
		self.val_base
	}
	
	pub fn upgrade ( &self ) -> Option< f64 > {
		match ( &self.upgrade, &self.mod_of_upgrade ) {
			( Some(_), Some(_) ) => Some( self.val_upgrade + self.val_upgrade_mod ),
			( Some(_), None ) => Some( self.val_upgrade ),
			( _, _ ) => None,
		}
	}
	
	pub fn upgrade_raw ( &self ) -> Option< f64 > {
		if let Some(_) = &self.upgrade {
			return Some( self.val_upgrade )
		}
		
		None
	}
	
}// CharStat - pub

// priv
impl CharStat {
	fn update_current_value ( &mut self ) {
		/*
		let base = self.base.value();
		let mut upgrade = 0.0;
		let mut base_mod = 0.0;
		let mut upgrade_mod = 0.0;
		let mut base_plus_upgrade_mod = 0.0;
		let mut mod_mlt = 1.0;
		
		if let Some( mod_mult ) = &mut self.mod_mult {
			mod_mlt = mod_mult.get();
		}
		
		if let Some( mod_base ) = &mut self.mod_of_base {
			base_mod = mod_base.get() * mod_mlt;
		}
		
		if let Some( up ) = &self.upgrade {
			upgrade = up.value();
			
			if let Some( mod_up ) = &mut self.mod_of_upgrade {
				upgrade_mod = mod_up.get() * mod_mlt;
			}
			
			if let Some( mod_base_plus_up ) = &mut self.mod_of_base_plus_upgrade {
				mod_base_plus_up.update( base + upgrade );
				base_plus_upgrade_mod = mod_base_plus_up.get() * mod_mlt;
			}
		}
		
		self.current_value = base + upgrade + base_mod + upgrade_mod + base_plus_upgrade_mod;
		*/
		
		self.current_value = self.val_base + self.val_base_mod + self.val_upgrade + self.val_upgrade_mod + self.val_base_plus_upgrade_mod;
	}// calc_impl
	
	fn append_base_mod( &mut self, modifier: Modifier ) -> Result<(),()> {
		if let Some( mod_of_base ) = &mut self.mod_of_base {
			mod_of_base.append_mod_unchecked( self.base.value(), modifier );
			self.update_base_mod();
			
			return Ok(())
		}
		
		Err(())
	}
	
	fn append_upgrade_mod( &mut self, modifier: Modifier ) -> Result<(),()> {
		if let ( Some( mod_of_upgrade ), Some( upgrade ) ) = ( &mut self.mod_of_upgrade, &mut self.upgrade ) {
			mod_of_upgrade.append_mod_unchecked( upgrade.value(), modifier );
			self.update_upgrade_mod();
			
			return Ok(())
		}
		
		Err(())
	}
	
	fn append_base_plus_upgrade_mod( &mut self, modifier: Modifier ) -> Result<(),()> {
		if let Some( tmp ) = &mut self.mod_of_base_plus_upgrade {
			let val = self.val_base + self.val_upgrade;
			tmp.append_mod_unchecked( val, modifier );
			self.update_base_plus_upgrade_mod();
			
			return Ok(())
		}
		
		Err(())
	}
	
	fn append_modmult( &mut self, modifier: Modifier ) -> Result<(),()> {
		if let Some( mod_mult ) = &mut self.mod_mult {
			mod_mult.append_mod_unchecked( modifier );
			
			self.val_mod_mult = mod_mult.get();
			
			if let Some(_) = self.mod_of_base {
				self.update_base_mod();
			}
			
			if let Some(_) = self.mod_of_upgrade {
				self.update_upgrade_mod();
			}
			
			if let Some(_) = self.mod_of_base_plus_upgrade {
				self.update_base_plus_upgrade_mod();
			}
			
			return Ok(())
		}
		
		Err(())
	}// append_modmult
	
	fn remove_expired_modifiers( &mut self ) {
		if let Some( tmp ) = &mut self.mod_mult {
			tmp.remove_expired( self.time_stamp );
			self.val_mod_mult = tmp.get();
		}
		
		if let Some( tmp ) = &mut self.mod_of_base {
			tmp.remove_expired( self.time_stamp );
			self.update_base_mod();
		}
		
		if let Some( tmp ) = &mut self.mod_of_upgrade {
			tmp.remove_expired( self.time_stamp );
			self.update_upgrade_mod();
		}
		
		if let Some( tmp ) = &mut self.mod_of_base_plus_upgrade {
			tmp.remove_expired( self.time_stamp );
			self.update_base_plus_upgrade_mod();
		}
		
		self.update_current_value();
	}// remove_expired_modifiers
	
	fn update_base( &mut self ) {
		self.val_base = self.base.value();
		
		if let Some(_) = self.mod_of_base {
			self.update_base_mod();
		}
	}
	
	fn update_base_mod( &mut self ) {
		if let Some( mod_mgr ) = &self.mod_of_base {
			self.val_base_mod = mod_mgr.get() * self.val_mod_mult;
		}
	}
	
	fn update_upgrade( &mut self ) {
		if let Some( upgrade ) = &self.upgrade {
			self.val_upgrade = upgrade.value();
			
			if let Some(_) = self.mod_of_upgrade {
				self.update_upgrade_mod();
			}
			
			if let Some(_) = self.mod_of_base_plus_upgrade {
				self.update_base_plus_upgrade_mod();
			}
		}
	}// update_upgrade
	
	fn update_upgrade_mod( &mut self ) {
		if let Some( mod_mgr ) = &self.mod_of_upgrade {
			self.val_upgrade_mod = mod_mgr.get() * self.val_mod_mult;
		}
	}
	
	fn update_base_plus_upgrade_mod( &mut self ) {
		if let Some( mod_mgr ) = &self.mod_of_base_plus_upgrade {
			self.val_base_plus_upgrade_mod = mod_mgr.get() * self.val_mod_mult;
		}
	}
}// CharStat - priv

impl CharStat {
	pub fn set_base_value ( &mut self, value: f64 ) -> Result<(), ()> {
		self.base.set_value( value )?;
		self.update_base();
		self.update_current_value();
		
		Ok(())
	}
	
	pub fn set_base_value_clamping ( &mut self, value: f64 ) -> Result<(), ()> {
		self.base.set_value_clamping( value )?;
		self.update_base();
		self.update_current_value();
		
		Ok(())
	}
	
	pub fn set_base_value_const ( &mut self ) {
		self.base.set_value_const();
	}
	
	pub fn set_base_bounds_min ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.base.set_bounds_min( new_val )?;
		
		Ok(())
	}
	
	pub fn set_base_bounds_max ( &mut self, new_val: f64 ) -> Result<(),()> {
		self.base.set_bounds_max( new_val )?;
		
		Ok(())
	}
	
	pub fn set_base_bounds_min_const ( &mut self ) {
		self.base.set_bounds_min_const();
	}
	
	pub fn set_base_bounds_max_const ( &mut self ) {
		self.base.set_bounds_max_const();
	}
	
	pub fn base_bounds_min ( &self ) -> f64 {
		self.base.bounds_min()
	}
	
	pub fn base_bounds_max ( &self ) -> f64 {
		self.base.bounds_max()
	}
}// CharStat - BaseConf

impl CharStat {
	pub fn set_upgrade_value ( &mut self, value: f64 ) -> Result<(), ()> {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_value( value )?;
			self.update_upgrade();
			self.update_current_value();
			
			return Ok(())
		}
		
		Err(())
	}
	
	pub fn set_upgrade_value_clamping ( &mut self, value: f64 ) -> Result<(), ()> {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_value_clamping( value )?;
			self.update_upgrade();
			self.update_current_value();
			
			return Ok(())
		}
		
		Err(())
	}
	
	pub fn set_upgrade_bounds_min ( &mut self, new_val: f64 ) -> Result<(),()> {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_bounds_min( new_val )?;
			
			return Ok(())
		}
		
		Err(())
	}
	
	pub fn set_upgrade_bounds_max ( &mut self, new_val: f64 ) -> Result<(),()> {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_bounds_max( new_val )?;
			
			return Ok(())
		}
		
		Err(())
	}
	
	pub fn set_upgrade_bounds_min_const ( &mut self ) -> Result<(),()> {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_bounds_min_const();
			
			return Ok(())
		}
		
		Err(())
	}
	
	pub fn set_upgrade_bounds_max_const ( &mut self ) -> Result<(),()> {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_bounds_max_const();
			
			return Ok(())
		}
		
		Err(())
	}
	
	pub fn upgrade_bounds_min ( &self ) -> Option< f64 > {
		if let Some( upgrade ) = &self.upgrade {
			return Some( upgrade.bounds_min() )
		}
		
		None
	}
	
	pub fn upgrade_bounds_max ( &self ) -> Option< f64 > {
		if let Some( upgrade ) = &self.upgrade {
			return Some( upgrade.bounds_max() )
		}
		
		None
	}
}// CharStat - UpgradeConf

impl Default for CharStat {
	fn default() -> Self {
		CharStat::new( BaseConf::default(), None, None, None, None, None )
	}
}

/*
#[derive(Clone, Eq, PartialEq)]
pub enum CharStatError {
	LogicIssue( CharStatLogicIssue ),
	InvalidValue( CharStatInvalidValue ),
	MissingObject( CharStatMissingObject ),
	Other,
}// CharStatError

impl From< CharStatLogicIssue > for CharStatError {
	fn from( value: CharStatLogicIssue ) -> Self {
		CharStatError::LogicIssue( value )
	}
}// from LogicIssue

impl From< CharStatInvalidValue > for CharStatError {
	fn from( value: CharStatInvalidValue ) -> Self {
		CharStatError::InvalidValue( value )
	}
}// from InvalidValue

impl From< CharStatMissingObject > for CharStatError {
	fn from( value: CharStatMissingObject ) -> Self {
		CharStatError::MissingObject( value )
	}
}// from MissingObject

impl Default for CharStatError {
	fn default() -> Self {
		CharStatError::Other
	}
}// from MissingObject

#[derive(Clone, Eq, PartialEq)]
pub enum CharStatLogicIssue {
	MinGreaterThanMax,
	ValueIsImmutable,
}// CharStatLogicIssue

#[derive(Clone, Eq, PartialEq)]
pub enum CharStatInvalidValue {
	ValueBelowMinimum,
	ValueAboveMaximum,
	ValueIsNan,
}// CharStatInvalidValue

#[derive(Clone, Eq, PartialEq)]
pub enum CharStatMissingObject {
	Upgrade,
	ModOfBase,
	ModOfUpgrade,
	ModOfBasePlusUpgrade,
	ModMult,
}// CharStatMissingObject
*/

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RoundingFunctionEnum {
	Round,
	RoundTiesEven,
	Floor,
	Ceil,
	Trunk,
	None,
}// RoundingFunction

impl RoundingFunctionEnum {
	fn do_rounding( &self, val: f64 ) -> f64 {
		match self {
			RoundingFunctionEnum::Round => val.round(),
			RoundingFunctionEnum::RoundTiesEven => val.round_ties_even(),
			RoundingFunctionEnum::Floor => val.floor(),
			RoundingFunctionEnum::Ceil => val.ceil(),
			RoundingFunctionEnum::Trunk => val.trunc(),
			RoundingFunctionEnum::None => val,
		}
	}// do_rounding
}// RoundingFunctionEnum

#[derive(Clone, PartialEq, Debug)]
pub struct RoundingHelper {
	function: RoundingFunctionEnum,
	precision: Option< f64 >,
}

impl RoundingHelper {
	pub fn new ( function: RoundingFunctionEnum, mut precision: Option< f64 >, ) -> Self {
		if let Some( val ) = &precision {
			if val.is_nan() {
				precision = None;
			}
		}
		
		RoundingHelper {
			function,
			precision,
		}
	}// new
	
	pub fn new_none() -> Self {
		RoundingHelper {
			function: RoundingFunctionEnum::None,
			precision: None,
		}
	}
	
	pub(crate) fn do_rounding( &self, mut value: f64 ) -> f64 {
		if let RoundingFunctionEnum::None = self.function {
			return value;
		}
		
		if let Some( prec ) = self.precision {
			value /= prec;
			value = self.function.do_rounding( value );
			
			return value * prec
		}
		
		self.function.do_rounding( value )
	}// do_rounding
}// RoundingHelper

impl Default for RoundingHelper {
	fn default() -> Self {
		RoundingHelper {
			function: RoundingFunctionEnum::None,
			precision: None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	// unwrap because hardcoded values have been checked manually.
	
	#[test]
	fn basic_functional() {
		let v_base: f64 = 10.0;
		let v_mult_b: f64 = 1.1;
		let v_mult_e: f64 = 1.0;
		let v_base_plus_mlt: f64 = v_base * ( v_mult_b.powf( v_mult_e ) );
		let v_upgrade: f64 = 2.0;
		let v_final = v_base_plus_mlt + v_upgrade;
		
		let bounds_mult_b = Bounds::new_const( 0.5, 1.5 ).unwrap();
		let bounds_mult_e = Bounds::new_mut( 1.0, 2.0 ).unwrap();
		let ronud_helper = RoundingHelper::new( RoundingFunctionEnum::Round, Some( 0.1 ) );
		let mult = BaseMultConf::new( v_mult_b, v_mult_e, bounds_mult_b, bounds_mult_e, ronud_helper );
		
		let bounds_base = Bounds::new_const( 4.0, 20.0 ).unwrap();
		let base = BaseConf::new( v_base, true, bounds_base, RoundingHelper::default(), mult ).unwrap();
		
		let bounds_up_mod = Bounds::new_const( -0.5, 1.5 ).unwrap();
		let rounding = RoundingHelper::new( RoundingFunctionEnum::None, None );
		let up_mod = ModConf::new( ModCalcStageEnum::Upgrade, bounds_up_mod, rounding, true, true );
		
		let bounds_upgr = Bounds::new_const( 0.0, 50.0 ).unwrap();
		let mut upgrade = UpgradeConf::new( 0.0, bounds_upgr ).unwrap();
		
		assert_eq!( base.value(), v_base_plus_mlt );
		assert_eq!( upgrade.set_value( 69.0 ), Err(()) );
		assert_eq!( upgrade.value(), 0.0 );
		
		assert_eq!( upgrade.set_value( v_upgrade ), Ok(()) );
		
		let mut cs = CharStat::new( base, Some( upgrade ), None, Some( up_mod ), None, None );
		assert_eq!( cs.value(), v_final );
		
		assert_eq!( cs.set_upgrade_value( v_upgrade + 2.0 ), Ok(()) );
		assert_eq!( cs.upgrade_raw(), Some( v_upgrade + 2.0 ) );
		
		assert_eq!( cs.value(), v_final + 2.0 );
		
		let modifier = Modifier::new( 0.5, None, ModCalcModeEnum::Mul, ModCalcStageEnum::Upgrade ).unwrap();
		assert_eq!( cs.new_modifier( modifier ), Ok(()) );
		assert_eq!( cs.value(), v_final + 4.0 );
		
	}// it_works
	
	#[test]
	fn modifiers() {
		let v_base: f64 = 10.0;
		let v_upgrade: f64 = 2.0;
		
		let v_base_mod = 0.1;
		let v_upgrade_mod = 0.25;
		let v_base_and_up_mod = 0.05;
		let v_mod_mult = 1.0;
		
		let mod_base = Modifier::new( v_base_mod, None, ModCalcModeEnum::Mul, ModCalcStageEnum::Base ).unwrap();
		let mod_upgrade = Modifier::new( v_upgrade_mod, None, ModCalcModeEnum::Mul, ModCalcStageEnum::Upgrade ).unwrap();
		let mod_base_and_up = Modifier::new( v_base_and_up_mod, None, ModCalcModeEnum::Mul, ModCalcStageEnum::BasePlusUpgrade ).unwrap();
		let mod_mod_mlt = Modifier::new( v_mod_mult, None, ModCalcModeEnum::Add, ModCalcStageEnum::ModMult ).unwrap();
		
		let bounds_base = Bounds::new_const( v_base, v_base ).unwrap();
		let bounds_upgrade = Bounds::new_const( v_upgrade, v_upgrade ).unwrap();
		let bounds_mod_base = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let bounds_mod_upgrade = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let bounds_mod_base_plus_upgrade = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let bounds_mod_mult = Bounds::new_const( 0.0, 1.0 ).unwrap();
		
		let base = BaseConf::new( v_base, false, bounds_base, RoundingHelper::new_none(), None ).unwrap();
		let upgrade = UpgradeConf::new( v_upgrade, bounds_upgrade );
		let mod_of_base = ModConf::new( ModCalcStageEnum::Base, bounds_mod_base, RoundingHelper::new_none(), false, false );
		let mod_of_upgrade = ModConf::new( ModCalcStageEnum::Base, bounds_mod_upgrade, RoundingHelper::new_none(), false, false );
		let mod_of_base_plus_upgrade = ModConf::new( ModCalcStageEnum::Base, bounds_mod_base_plus_upgrade, RoundingHelper::new_none(), false, false );
		let mod_mlt = ModMultConf::new( bounds_mod_mult );
		
		let mut cs = CharStat::new( base, upgrade, Some( mod_of_base ), Some( mod_of_upgrade ), Some( mod_of_base_plus_upgrade ), Some( mod_mlt ) );
		assert_eq!( cs.new_modifier( mod_base ), Ok(()) );
		assert_eq!( cs.new_modifier( mod_upgrade ), Ok(()) );
		assert_eq!( cs.new_modifier( mod_base_and_up ), Ok(()) );
		assert_eq!( cs.new_modifier( mod_mod_mlt ), Ok(()) );
		
		let mod_mult = 1.0 + v_mod_mult;
		
		let res_base_mod = v_base * ( v_base_mod * mod_mult );
		let res_up_mod = v_upgrade * ( v_upgrade_mod * mod_mult );
		let res_base_and_up_mod = (v_base + v_upgrade) * ( v_base_and_up_mod * mod_mult );
		
		assert_eq!( cs.base(), v_base + res_base_mod );
		assert_eq!( cs.upgrade().unwrap(), v_upgrade + res_up_mod );
		assert_eq!( cs.value(), v_base + res_base_mod + v_upgrade + res_up_mod + res_base_and_up_mod );
	}// modifiers
	
	#[test]
	fn nan_handling() {
		// Base
		let base = BaseConf::new( f64::NAN, true, Bounds::default(), RoundingHelper::default(), None );
		assert_eq!( base, None );
		let base = BaseConf::new_clamping( f64::NAN, true, Bounds::default(), RoundingHelper::default(), None );
		assert_eq!( base, None );
		
		// RoundingHelper
		let bad_rh = RoundingHelper::new( RoundingFunctionEnum::Round, Some( f64::NAN ) );
		assert_eq!( bad_rh.precision, None );
		let ronuding_helper = RoundingHelper::new( RoundingFunctionEnum::Round, Some( 0.1 ) );
		assert_eq!( ronuding_helper.precision, Some( 0.1 ) );
		
		// Modifier
		let modif = Modifier::new( f64::NAN, None, ModCalcModeEnum::Add, ModCalcStageEnum::Base );
		assert_eq!( modif, None );
	}// nan_handling
}
