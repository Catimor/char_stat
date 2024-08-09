#![allow( clippy::missing_panics_doc )] // manually ensured safety of build methods.

// --Lints
//------------------------------------------------------------------------------
// --Imports

use std::marker::PhantomData;

// --Imports
//------------------------------------------------------------------------------
// --Modules

use super::{ CharStat, BaseConf, UpgradeConf, ModConf, ModMultConf };

// --Modules
//------------------------------------------------------------------------------
// struct - CharStatBuilder

// derive: 3, constructors: 2, priv impl: 1, setters: 6, build: 3

/// A typestate builder for `CharStat`
/// 
/// The rules that determine whether builder is in valid state to call `.build()`
/// - `BaseConf` must always be set
/// - `mod_of_upgrade` and `mod_of_base_plus_upgrade` require `UpgradeConf`
/// - `mod_mult` requires at least one of the `mod_of_*` to be set
/// 
/// To make invalid state unreachable, a type-state pattern has been implemented.
/// 
/// Generics used:
/// - B: is `base` set `FldSet` or not `FldEmpty`
/// - Up: is `upgrade` set
/// - Upc: is setter for `upgrade` allowed `FldAllow` or not `FldDeny`
/// - MoB: is `mod_of_base` set
/// - MoBc: is `mod_of_base` allowed
/// - MoU: is `mod_of_upgrade` set
/// - MoUc: are `mod_of_upgrade` and `mod_of_base_plus_upgrade` allowed
/// - MoBpU: is `mod_of_base_plus_upgrade` set
/// - Mmc: is `mod_mult` allowed
/// 
/// There was no need to track whether mod_mult is set
#[derive( Debug, Clone, PartialEq,  )]
pub struct CharStatBuilder< B, Up, Upc, MoB, MoBc, MoU, MoUc, MoBpU, Mmc >
where
	B: FldState,
	Up: FldState,
	Upc: FldCtrl,
	MoB: FldState,
	MoBc: FldCtrl,
	MoU: FldState,
	MoUc: FldCtrl,
	MoBpU: FldState,
	Mmc: FldCtrl,
{
	base:											Option< BaseConf >,
	upgrade:									Option< UpgradeConf >,
	mod_of_base:							Option< ModConf >,
	mod_of_upgrade:						Option< ModConf >,
	mod_of_base_plus_upgrade:	Option< ModConf >,
	mod_mult:									Option< ModMultConf >,
	
	b: PhantomData< B >,
	up: PhantomData< Up >,
	up_ctrl: PhantomData< Upc >,
	mod_b: PhantomData< MoB >,
	mod_b_ctrl: PhantomData< MoBc >,
	mod_u: PhantomData< MoU >,
	mod_u_ctrl: PhantomData< MoUc >,
	mod_b_u: PhantomData< MoBpU >,
	mod_m_ctrl: PhantomData< Mmc >,
}

impl CharStatBuilder< FldEmpty, FldEmpty, FldDeny, FldEmpty, FldDeny, FldEmpty, FldDeny, FldEmpty, FldDeny > {
	#[inline]
	pub fn new () -> CharStatBuilder< FldEmpty, FldEmpty, FldDeny, FldEmpty, FldDeny, FldEmpty, FldDeny, FldEmpty, FldDeny > {
		CharStatBuilder {
			base:											None,
			upgrade:									None,
			mod_of_base:							None,
			mod_of_upgrade:						None,
			mod_of_base_plus_upgrade:	None,
			mod_mult:									None,
			
			b: PhantomData::< FldEmpty >,
			up: PhantomData::< FldEmpty >,
			up_ctrl: PhantomData::< FldDeny >,
			mod_b: PhantomData::< FldEmpty >,
			mod_b_ctrl: PhantomData::< FldDeny >,
			mod_u: PhantomData::< FldEmpty >,
			mod_u_ctrl: PhantomData::< FldDeny >,
			mod_b_u: PhantomData::< FldEmpty >,
			mod_m_ctrl: PhantomData::< FldDeny >,
		}
	}
}

impl< Up, Upc, MoB, MoBc, MoU, MoUc, MoBpU, Mmc > CharStatBuilder< FldSet, Up, Upc, MoB, MoBc, MoU, MoUc, MoBpU, Mmc >
where
	Up: FldState,
	Upc: FldCtrl,
	MoB: FldState,
	MoBc: FldCtrl,
	MoU: FldState,
	MoUc: FldCtrl,
	MoBpU: FldState,
	Mmc: FldCtrl,
{
	fn build_impl ( &self ) -> CharStat {
		let base = self.base.clone().unwrap();
		let upgrade = self.upgrade.clone();
		let mod_of_base = self.mod_of_base.clone();
		let mod_of_upgrade = self.mod_of_upgrade.clone();
		let mod_of_base_plus_upgrade = self.mod_of_base_plus_upgrade.clone();
		let mod_mult = self.mod_mult.clone();
		
		CharStat::new( base, upgrade, mod_of_base, mod_of_upgrade, mod_of_base_plus_upgrade, mod_mult )
	}
}

impl Default for CharStatBuilder< FldEmpty, FldEmpty, FldDeny, FldEmpty, FldDeny, FldEmpty, FldDeny, FldEmpty, FldDeny > {
	#[inline]
	fn default() -> Self { CharStatBuilder::new() }
}

// new & default
//------------------------------------------------------------------------------
// setters

impl< B, Up, Upc, MoB, MoBc, MoU, MoUc, MoBpU, Mmc > CharStatBuilder< B, Up, Upc, MoB, MoBc, MoU, MoUc, MoBpU, Mmc >
where
	B: FldState,
	Up: FldState,
	Upc: FldCtrl,
	MoB: FldState,
	MoBc: FldCtrl,
	MoU: FldState,
	MoUc: FldCtrl,
	MoBpU: FldState,
	Mmc: FldCtrl,
{
	#[inline]
	pub fn base ( self, value: BaseConf ) -> CharStatBuilder< FldSet, Up, FldAllow, MoB, FldAllow, MoU, MoUc, MoBpU, Mmc > {
		CharStatBuilder {
			base:											Some( value ),
			upgrade:									self.upgrade,
			mod_of_base:							self.mod_of_base,
			mod_of_upgrade:						self.mod_of_upgrade,
			mod_of_base_plus_upgrade:	self.mod_of_base_plus_upgrade,
			mod_mult:									self.mod_mult,
			
			b: PhantomData::< FldSet >,
			up: PhantomData::< Up >,
			up_ctrl: PhantomData::< FldAllow >,
			mod_b: PhantomData::< MoB >,
			mod_b_ctrl: PhantomData::< FldAllow >,
			mod_u: PhantomData::< MoU >,
			mod_u_ctrl: PhantomData::< MoUc >,
			mod_b_u: PhantomData::< MoBpU >,
			mod_m_ctrl: PhantomData::< Mmc >,
		}
	}
}

impl< Up, MoB, MoU, MoUc, MoBpU, Mmc > CharStatBuilder< FldSet, Up, FldAllow, MoB, FldAllow, MoU, MoUc, MoBpU, Mmc >
where
	Up: FldState,
	MoB: FldState,
	MoU: FldState,
	MoUc: FldCtrl,
	MoBpU: FldState,
	Mmc: FldCtrl,
{
	#[inline]
	pub fn upgrade ( self, value: UpgradeConf ) -> CharStatBuilder< FldSet, FldSet, FldAllow, MoB, FldAllow, MoU, FldAllow, MoBpU, Mmc > {
		CharStatBuilder {
			base:											self.base,
			upgrade:									Some( value ),
			mod_of_base:							self.mod_of_base,
			mod_of_upgrade:						self.mod_of_upgrade,
			mod_of_base_plus_upgrade:	self.mod_of_base_plus_upgrade,
			mod_mult:									self.mod_mult,
			
			b: PhantomData::< FldSet >,
			up: PhantomData::< FldSet >,
			up_ctrl: PhantomData::< FldAllow >,
			mod_b: PhantomData::< MoB >,
			mod_b_ctrl: PhantomData::< FldAllow >,
			mod_u: PhantomData::< MoU >,
			mod_u_ctrl: PhantomData::< FldAllow >,
			mod_b_u: PhantomData::< MoBpU >,
			mod_m_ctrl: PhantomData::< Mmc >,
		}
	}
}

impl< Up, MoB, MoU, MoUc, MoBpU, Mmc > CharStatBuilder< FldSet, Up, FldAllow, MoB, FldAllow, MoU, MoUc, MoBpU, Mmc >
where
	Up: FldState,
	MoB: FldState,
	MoU: FldState,
	MoUc: FldCtrl,
	MoBpU: FldState,
	Mmc: FldCtrl,
{
	#[inline]
	pub fn mod_of_base ( self, value: ModConf ) -> CharStatBuilder< FldSet, Up, FldAllow, FldSet, FldAllow, MoU, MoUc, MoBpU, FldAllow > {
		CharStatBuilder {
			base:											self.base,
			upgrade:									self.upgrade,
			mod_of_base:							Some( value ),
			mod_of_upgrade:						self.mod_of_upgrade,
			mod_of_base_plus_upgrade:	self.mod_of_base_plus_upgrade,
			mod_mult:									self.mod_mult,
			
			b: PhantomData::< FldSet >,
			up: PhantomData::< Up >,
			up_ctrl: PhantomData::< FldAllow >,
			mod_b: PhantomData::< FldSet >,
			mod_b_ctrl: PhantomData::< FldAllow >,
			mod_u: PhantomData::< MoU >,
			mod_u_ctrl: PhantomData::< MoUc >,
			mod_b_u: PhantomData::< MoBpU >,
			mod_m_ctrl: PhantomData::< FldAllow >,
		}
	}
}

impl< MoB, MoU, MoBpU, Mmc > CharStatBuilder< FldSet, FldSet, FldAllow, MoB, FldAllow, MoU, FldAllow, MoBpU, Mmc >
where
	MoB: FldState,
	MoU: FldState,
	MoBpU: FldState,
	Mmc: FldCtrl,
{
	#[inline]
	pub fn mod_of_upgrade ( self, value: ModConf ) -> CharStatBuilder< FldSet, FldSet, FldAllow, MoB, FldAllow, FldSet, FldAllow, MoBpU, FldAllow > {
		CharStatBuilder {
			base:											self.base,
			upgrade:									self.upgrade,
			mod_of_base:							self.mod_of_base,
			mod_of_upgrade:						Some( value ),
			mod_of_base_plus_upgrade:	self.mod_of_base_plus_upgrade,
			mod_mult:									self.mod_mult,
			
			b: PhantomData::< FldSet >,
			up: PhantomData::< FldSet >,
			up_ctrl: PhantomData::< FldAllow >,
			mod_b: PhantomData::< MoB >,
			mod_b_ctrl: PhantomData::< FldAllow >,
			mod_u: PhantomData::< FldSet >,
			mod_u_ctrl: PhantomData::< FldAllow >,
			mod_b_u: PhantomData::< MoBpU >,
			mod_m_ctrl: PhantomData::< FldAllow >,
		}
	}
}

impl< MoB, MoU, MoBpU, Mmc > CharStatBuilder< FldSet, FldSet, FldAllow, MoB, FldAllow, MoU, FldAllow, MoBpU, Mmc >
where
	MoB: FldState,
	MoU: FldState,
	MoBpU: FldState,
	Mmc: FldCtrl,
{
	#[inline]
	pub fn mod_of_base_plus_upgrade ( self, value: ModConf ) -> CharStatBuilder< FldSet, FldSet, FldAllow, MoB, FldAllow, MoU, FldAllow, FldSet, FldAllow > {
		CharStatBuilder {
			base:											self.base,
			upgrade:									self.upgrade,
			mod_of_base:							self.mod_of_base,
			mod_of_upgrade:						self.mod_of_upgrade,
			mod_of_base_plus_upgrade:	Some( value ),
			mod_mult:									self.mod_mult,
			
			b: PhantomData::< FldSet >,
			up: PhantomData::< FldSet >,
			up_ctrl: PhantomData::< FldAllow >,
			mod_b: PhantomData::< MoB >,
			mod_b_ctrl: PhantomData::< FldAllow >,
			mod_u: PhantomData::< MoU >,
			mod_u_ctrl: PhantomData::< FldAllow >,
			mod_b_u: PhantomData::< FldSet >,
			mod_m_ctrl: PhantomData::< FldAllow >,
		}
	}
}

impl< Up, MoB, MoU, MoUc, MoBpU > CharStatBuilder< FldSet, Up, FldAllow, MoB, FldAllow, MoU, MoUc, MoBpU, FldAllow >
where
	Up: FldState,
	MoB: FldState,
	MoU: FldState,
	MoUc: FldCtrl,
	MoBpU: FldState,
{
	#[inline]
	#[must_use]
	pub fn mod_mult ( self, value: ModMultConf ) -> CharStatBuilder< FldSet, Up, FldAllow, MoB, FldAllow, MoU, MoUc, MoBpU, FldAllow > {
		CharStatBuilder {
			base:											self.base,
			upgrade:									self.upgrade,
			mod_of_base:							self.mod_of_base,
			mod_of_upgrade:						self.mod_of_upgrade,
			mod_of_base_plus_upgrade:	self.mod_of_base_plus_upgrade,
			mod_mult:									Some( value ),
			
			b: PhantomData::< FldSet >,
			up: PhantomData::< Up >,
			up_ctrl: PhantomData::< FldAllow >,
			mod_b: PhantomData::< MoB >,
			mod_b_ctrl: PhantomData::< FldAllow >,
			mod_u: PhantomData::< MoU >,
			mod_u_ctrl: PhantomData::< MoUc >,
			mod_b_u: PhantomData::< MoBpU >,
			mod_m_ctrl: PhantomData::< FldAllow >,
		}
	}
}

// setters
//------------------------------------------------------------------------------
// build

impl CharStatBuilder< FldSet, FldEmpty, FldAllow, FldEmpty, FldAllow, FldEmpty, FldDeny, FldEmpty, FldDeny > { // no up, no mod
	#[inline]
	pub fn build ( &self ) -> CharStat {
		let base = self.base.clone().unwrap();
		
		CharStat::new_minimal( base )
	}
}

impl CharStatBuilder< FldSet, FldSet, FldAllow, FldEmpty, FldAllow, FldEmpty, FldAllow, FldEmpty, FldDeny > { // up, no mod
	#[inline]
	pub fn build ( &self ) -> CharStat {
		let base = self.base.clone().unwrap();
		let upgrade = self.upgrade.clone();
		
		CharStat::new_no_mod( base, upgrade )
	}
}

impl< Up, MoB, MoU, MoUc, MoBpU > CharStatBuilder< FldSet, Up, FldAllow, MoB, FldAllow, MoU, MoUc, MoBpU, FldAllow >// any mod
where
	Up: FldState,
	MoB: FldState,
	MoU: FldState,
	MoUc: FldCtrl,
	MoBpU: FldState,
{
	#[inline]
	pub fn build ( &self ) -> CharStat {
		self.build_impl()
	}
}

// struct - CharStatBuilder
//------------------------------------------------------------------------------
// traits

#[doc( hidden )]
pub enum FldDeny {}

#[doc( hidden )]
pub enum FldAllow {}

#[doc( hidden )]
pub enum FldEmpty {}

#[doc( hidden )]
pub enum FldSet {}

#[doc( hidden )]
pub trait FldState: private::Sealed {}
impl FldState for FldEmpty {}
impl FldState for FldSet {}

#[doc( hidden )]
pub trait FldCtrl: private::Sealed {}
impl FldCtrl for FldDeny {}
impl FldCtrl for FldAllow {}

// traits
//------------------------------------------------------------------------------
// --Tests

#[cfg( test )]
mod tests {
	use crate::{ Bounds, ModCalcStage, RoundingHelper  };
	use super::*;
	
	#[test]
	fn basic_functional() {
		let bounds = Bounds::new_const( 0.0, 10.0 ).unwrap();
		let rnd_hlp = RoundingHelper::new_none();
		
		let base = BaseConf::new( 2.0, true, bounds, rnd_hlp.clone(), None ).unwrap();
		let up_conf = UpgradeConf::new( 2.0, bounds, rnd_hlp.clone() ).unwrap();
		
		let builder = CharStatBuilder::new();// methods available: .base( ... )
		let builder = builder.base( base.clone() );
		// methods available: .mod_of_base(...) | .upgrade(...) | .build()
		
		let cs = builder.build();
		assert_eq!( cs, CharStat::new_minimal( base.clone() ) );
		
		let builder = builder.upgrade( up_conf.clone() );
		// methods available: .mod_of_base(...) | .mod_of_upgrade(...) | .mod_of_base_plus_upgrade(...) | .build()
		
		let cs_2 = builder.build();
		let cs_ref = CharStat::new_no_mod(
			base.clone(),
			Some( up_conf.clone() )
		);
		
		assert_eq!( cs_2, cs_ref );
		
		let up_mod_cfg = ModConf::new(
			ModCalcStage::Upgrade,
			bounds,
			rnd_hlp.clone(),
			false,
			false
		);
		let builder = builder.mod_of_upgrade( up_mod_cfg.clone() );
		// methods available: .mod_of_base(...) | .mod_of_base_plus_upgrade(...) | .mod_mult(...) | .build()
		
		let cs_3 = builder.build();
		let cs_ref = CharStat::new(
			base.clone(),
			Some( up_conf.clone() ),
			None,
			Some( up_mod_cfg.clone() ),
			None,
			None
		);
		
		assert_eq!( cs_3, cs_ref );
		
		let mod_mult = ModMultConf::new( bounds );
		let builder = builder.mod_mult( mod_mult.clone() );
		// methods available: .mod_of_base(...) | .mod_of_base_plus_upgrade(...) | .build()
		
		let cs_4 = builder.build();
		let cs_ref = CharStat::new(
			base.clone(),
			Some( up_conf.clone() ),
			None,
			Some( up_mod_cfg.clone() ),
			None,
			Some( mod_mult.clone() )
		);
		
		assert_eq!( cs_4, cs_ref );
		
		let mod_of_base = ModConf::new(
			ModCalcStage::Base,
			bounds,
			rnd_hlp.clone(),
			false,
			false
		);
		let builder = builder.mod_of_base( mod_of_base.clone() );
		// methods available: .mod_of_base_plus_upgrade(...) | .build()
		
		let cs_5 = builder.build();
		let cs_ref = CharStat::new(
			base.clone(),
			Some( up_conf.clone() ),
			Some( mod_of_base.clone() ),
			Some( up_mod_cfg.clone() ),
			None,
			Some( mod_mult.clone() )
		);
		
		assert_eq!( cs_5, cs_ref );
		
		let mod_of_base_plus_up = ModConf::new(
			ModCalcStage::BasePlusUpgrade,
			bounds,
			rnd_hlp.clone(),
			false,
			false
		);
		let builder = builder.mod_of_base_plus_upgrade( mod_of_base_plus_up.clone() );
		// methods available: .build()
		
		let cs_6 = builder.build();
		let cs_ref = CharStat::new(
			base.clone(),
			Some( up_conf.clone() ),
			Some( mod_of_base.clone() ),
			Some( up_mod_cfg.clone() ),
			Some( mod_of_base_plus_up.clone() ),
			Some( mod_mult.clone() )
		);
		
		assert_eq!( cs_6, cs_ref );
	}
}

// --Tests
//------------------------------------------------------------------------------
// sealed trait pattern

#[doc( hidden )]
mod private {
	use super::{ FldDeny, FldAllow, FldEmpty, FldSet };
	
	pub trait Sealed {}
	
	impl Sealed for FldDeny {}
	impl Sealed for FldAllow {}
	impl Sealed for FldEmpty {}
	impl Sealed for FldSet {}
}

// sealed trait pattern
//------------------------------------------------------------------------------

