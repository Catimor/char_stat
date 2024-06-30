//! # CharStat
//! 
//! Game dev library for handling character statistics.
//! 
//! ## Roadmap
//! 
//! - [x] Main components: Base, Upgrade <br>
//! - [x] Simple Modifiers <br>
//! - [x] Stackable Modifiers <br>
//! - [x] Custom error types <br>
//! - [ ] Documentation ( partially done ) <br>
//! 
//! # Usage
//!
//! base with upgrade
//!  ```rust
//! use char_stat::{ CharStat, BaseConf, UpgradeConf, Bounds, RoundingHelper };
//! 
//! // new_const( min, max ) -> Result< Bounds, CharStatError >
//! let base_bounds = Bounds::new_const( 4.0, 20.0 ).unwrap();
//! 
//! // new( value, is_mut, bounds, rounding, multiplier ) -> Result
//! let base = BaseConf::new( 10.0, true, base_bounds, RoundingHelper::new_none(), None ).unwrap();
//! 
//! let upgrade_bounds = Bounds::new_const( 0.0, 50.0 ).unwrap();
//! 
//! // new( value, bounds, rounding_fn ) -> Result
//! let upgrade = UpgradeConf::new( 2.0, upgrade_bounds, RoundingHelper::new_none() ).unwrap();
//! 
//! let example = CharStat::new_no_mod( base, Some( upgrade ) );
//! 
//! assert_eq!( example.value(), 12.0 )
//! ```
//! 
//! Base with multiplier <br>
//! Could be used to calculate exp points required for next level. Here exponent is the characters' current level.
//! ```rust
//! use char_stat::{ CharStat, BaseConf, BaseMultConf, Bounds, RoundingHelper, RoundingFnEnum };
//! 
//! let b_base = Bounds::new_const( 0.0, 20_000.0 ).unwrap();
//! let b_mult_base = Bounds::new_const( 1.4, 1.4 ).unwrap();
//! let b_mult_exp = Bounds::new_const( 0.0, 10.0 ).unwrap();
//! 
//! let rh_base = RoundingHelper::new( RoundingFnEnum::Floor, None );
//! let rh_mult = RoundingHelper::new_none();
//! 
//! let mult = BaseMultConf::new( 1.4, 0.0, b_mult_base, b_mult_exp, rh_mult ).unwrap();
//! let base = BaseConf::new( 500.0, true, b_base, rh_base, Some( mult ) ).unwrap();
//! 
//! let mut example = CharStat::new_minimal( base );
//! 
//! assert_eq!( example.value(), 500.0 );
//! 
//! let _ = example.set_mult_exponent( 4.0 );
//! 
//! // 1.4_f64.powf( 4.0 ) = approx 3.8416
//! // ( 500.0 * 3.8416 ).floor() = 1920.0
//! assert_eq!( example.value(), 1920.0 );
//! ```
//! 
//! Base with modifiers
//! ```rust
//! use char_stat::{ CharStat, BaseConf, ModConf, Modifier, ModCommon, ModCalcMode, ModCalcStage, Bounds, RoundingHelper };
//! 
//! let base_bounds = Bounds::new_const( 4.0, 20.0 ).unwrap();
//! let base = BaseConf::new( 10.0, true, base_bounds, RoundingHelper::new_none(), None ).unwrap();
//! 
//! // this modifier will add half of base value to the total
//! let common = ModCommon::new( 0.50, ModCalcMode::Mul, ModCalcStage::Base ).unwrap();
//! let mod_1 = Modifier::new_persistent( common );
//! 
//! let mod_of_base_bounds = Bounds::new_const( -0.50, 0.50 ).unwrap();
//! 
//! // new( stage, bounds, rounding_fn, is_min_percent, is_max_percent ) -> Self
//! // min/max interpreted as a percent of the base value => here, the sum of all modifiers cannot exceed +/- 50% of the base value
//! let mod_of_base = ModConf::new( ModCalcStage::Base, mod_of_base_bounds, RoundingHelper::new_none(), true, true );
//! 
//! let mut cs = CharStat::new( base, None, Some( mod_of_base ), None, None, None );
//! 
//! cs.append_modifier( mod_1 ).unwrap();
//! 
//! assert_eq!( 15.0, cs.value() );
//! ```
//! 

//------------------------------------------------------------------------------
// --Lints

//#![allow(unused_variables)]
//#![allow(dead_code)]
#![forbid(unsafe_code)]
#![warn(clippy::all)]
// pedantic
#![warn(
	clippy::cast_lossless, 
	clippy::checked_conversions, 
	clippy::default_trait_access, 
	clippy::float_cmp, 
	clippy::fn_params_excessive_bools, 
	clippy::if_not_else,
	clippy::ignored_unit_patterns,
	clippy::implicit_clone,
	clippy::inconsistent_struct_constructor,
	clippy::index_refutable_slice,
	clippy::inefficient_to_string,
	clippy::items_after_statements,
	clippy::large_types_passed_by_value,
	clippy::manual_assert,
	clippy::manual_let_else,
	clippy::manual_string_new,
	clippy::match_on_vec_items,
	clippy::match_same_arms,
	clippy::match_wild_err_arm,
	clippy::match_wildcard_for_single_variants,
	clippy::mismatching_type_param_order,
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	//clippy::module_name_repetitions,
	clippy::mut_mut,
	clippy::needless_continue,
	clippy::needless_for_each,
	clippy::needless_pass_by_value,
	clippy::option_option,
	clippy::redundant_closure_for_method_calls,
	clippy::redundant_else,
	clippy::ref_option_ref,
	clippy::return_self_not_must_use,
	clippy::same_functions_in_if_condition,
	clippy::semicolon_if_nothing_returned,
	clippy::should_panic_without_expect,
	clippy::similar_names,
	clippy::single_match_else,
	clippy::stable_sort_primitive,
	clippy::str_split_at_newline,
	clippy::string_add_assign,
	clippy::struct_excessive_bools,
	clippy::struct_field_names,
	clippy::too_many_lines,
	clippy::trivially_copy_pass_by_ref,
	clippy::unicode_not_nfc,
	clippy::uninlined_format_args,
	clippy::unnecessary_wraps,
	clippy::unnested_or_patterns,
	clippy::unused_self,
	clippy::missing_inline_in_public_items,
)]
// style
#![warn(
	clippy::assertions_on_constants,
	clippy::assign_op_pattern,
	clippy::blocks_in_conditions,
	clippy::bool_assert_comparison,
	clippy::collapsible_else_if,
	clippy::collapsible_if,
	clippy::collapsible_match,
	clippy::comparison_chain,
	clippy::comparison_to_empty,
	clippy::enum_variant_names,
	clippy::field_reassign_with_default,
	clippy::get_first,
	clippy::implicit_saturating_add,
	clippy::implicit_saturating_sub,
	clippy::infallible_destructuring_match,
	clippy::inherent_to_string,
	clippy::is_digit_ascii_radix,
	clippy::iter_nth,
	clippy::iter_nth_zero,
	clippy::len_zero,
	clippy::let_and_return,
	clippy::manual_is_ascii_check,
	clippy::manual_map,
	clippy::manual_range_contains,
	clippy::manual_while_let_some,
	clippy::match_overlapping_arm,
	clippy::match_ref_pats,
	clippy::match_result_ok,
	clippy::needless_borrow,
	clippy::needless_range_loop,
	clippy::new_without_default,
	clippy::op_ref,
	clippy::question_mark,
	clippy::redundant_closure,
	clippy::redundant_field_names,
	clippy::redundant_pattern,
	clippy::redundant_pattern_matching,
	clippy::redundant_static_lifetimes,
	clippy::same_item_push,
	clippy::self_named_constructors,
	clippy::should_implement_trait,
	clippy::single_char_add_str,
	clippy::single_match,
	clippy::to_digit_is_some,
	clippy::trim_split_whitespace,
	clippy::unnecessary_fallible_conversions,
	clippy::unnecessary_fold,
	clippy::unnecessary_lazy_evaluations,
	clippy::unnecessary_mut_passed,
	clippy::unnecessary_owned_empty_strings,
	clippy::while_let_on_iterator,
	clippy::write_literal,
	clippy::wrong_self_convention,
)]

// --Lints
//------------------------------------------------------------------------------
// --Imports

#[cfg(feature = "serde")]
use serde::{ Serialize, Deserialize };

// --Imports
//------------------------------------------------------------------------------
// --Modules

mod bounds;
pub use bounds::*;

mod errors;
pub use errors::*;

mod modifier;
pub use modifier::*;

mod base;
pub use base::*;

mod base_mult;
pub use base_mult::*;

mod upgrade;
pub use upgrade::*;

mod mod_mgr;
pub use mod_mgr::*;

mod mod_mult;
pub use mod_mult::*;

//pub mod builder;
//use builder::*;

// --Modules
//------------------------------------------------------------------------------
// struct - CharStat

#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq )]
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
	upgrade:									Option< Box< UpgradeConf > >,
	mod_of_base:							Option< Box< ModConf > >,
	mod_of_upgrade:						Option< Box< ModConf > >,
	mod_of_base_plus_upgrade:	Option< Box< ModConf > >,
	mod_mult:									Option< Box< ModMultConf > >,
}

impl CharStat {
	/// # Panics
	/// - when upgrade is None, but mod_of_upgrade or mod_of_base_plus_upgrade is Some
	/// - when mod_mult is Some, but every ModConf is None
	#[inline]
	pub fn new (
		base: BaseConf, 
		upgrade: Option< UpgradeConf >, 
		mod_of_base: Option< ModConf >, 
		mod_of_upgrade: Option< ModConf>, 
		mod_of_base_plus_upgrade: Option< ModConf >, 
		mod_mult: Option< ModMultConf >, 
	) -> Self {
		// modifiers
		if let ( None, Some(_) ) = ( &upgrade, &mod_of_upgrade ) {
			panic!()
		}
		
		if let ( None, Some(_) ) = ( &upgrade, &mod_of_base_plus_upgrade ) {
			panic!()
		}
		
		let tmp = mod_of_base.is_none() || mod_of_upgrade.is_none() || mod_of_base_plus_upgrade.is_none();
		assert!( !( tmp && mod_mult.is_some() ));
		
		let val_mod_mult = if let Some( tmp ) = &mod_mult {
			tmp.value()
		} else {
			1.0
		};
		// end modifiers
		
		// pointers
		let upgrade = upgrade.map( Box::new );
		let mod_of_base = mod_of_base.map( Box::new );
		let mod_of_upgrade = mod_of_upgrade.map( Box::new );
		let mod_of_base_plus_upgrade = mod_of_base_plus_upgrade.map( Box::new );
		let mod_mult = mod_mult.map( Box::new );
		// end pointers
		
		let mut out = CharStat {
			current_value: 0.0,
			time_stamp: 0,
			
			val_base: 0.0,
			val_base_mod: 0.0,
			val_upgrade: 0.0,
			val_upgrade_mod: 0.0,
			val_base_plus_upgrade_mod: 0.0,
			val_mod_mult,
			
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
		
		out
	}
	
	#[inline]
	pub fn new_minimal ( base: BaseConf ) -> Self {
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
			upgrade: None,
			mod_of_base: None,
			mod_of_upgrade: None,
			mod_of_base_plus_upgrade: None,
			mod_mult: None,
		};
		
		out.update_base();
		out.update_current_value();
		
		out
	}
	
	#[inline]
	pub fn new_no_mod ( base: BaseConf, upgrade: Option< UpgradeConf > ) -> Self {
		let upgrade = upgrade.map( Box::new );
		
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
			mod_of_base: None,
			mod_of_upgrade: None,
			mod_of_base_plus_upgrade: None,
			mod_mult: None,
		};
		
		out.update_base();
		out.update_upgrade();
		out.update_current_value();
		
		out
	}
	
	/// returns the current ( total ) value
	#[inline]
	pub fn value ( &self ) -> f64 {
		self.current_value
	}
	
	/// Sets the timestamp and checks for expired modifiers.
	/// 
	/// # Examples
	/// ```rust
	/// use char_stat::{ CharStat, BaseConf, Bounds, RoundingHelper, CharStatError, CsLogicIssue };
	/// 
	/// // example uses `new_minimal` for the sake of brevity
	/// // `new_minimal` and `new_no_mod` does not allow any modifiers which makes setting TS pointless
	/// let bounds = Bounds::new_const( 0.0, 1.0 ).unwrap();
	/// let base = BaseConf::new( 0.0, true, bounds, RoundingHelper::new_none(), None ).unwrap();
	/// let mut cs = CharStat::new_minimal( base );
	/// assert_eq!( cs.set_ts( 100 ), Ok(()) );
	/// 
	/// // check for "time travel"
	/// let err = Err( CsLogicIssue::TimeTravel.into() );
	/// assert_eq!( cs.set_ts( 0 ), err );
	/// ```
	/// 
	/// # Errors
	/// `CsLogicIssue::TimeTravel` when `new_val` > `self.time_stamp` <br>
	#[inline]
	pub fn set_ts( &mut self, new_val: u64 ) -> Result<(), CharStatError > {
		if new_val < self.time_stamp {
			return Err( CsLogicIssue::TimeTravel.into() )
		}
		
		self.time_stamp = new_val;
		
		self.remove_expired_modifiers();
		
		Ok(())
	}
	
	/// Sets the timestamp and checks for expired modifiers.
	/// Performs no checks against new TS being lower (earlier)
	/// 
	/// # Examples
	/// ```rust
	/// use char_stat::{ CharStat, BaseConf, Bounds, RoundingHelper };
	/// 
	/// // example uses `new_minimal` for the sake of brevity
	/// // `new_minimal` and `new_no_mod` does not allow any modifiers which makes setting TS pointless
	/// let bounds = Bounds::new_const( 0.0, 1.0 ).unwrap();
	/// let base = BaseConf::new( 0.0, true, bounds, RoundingHelper::new_none(), None ).unwrap();
	/// let mut cs = CharStat::new_minimal( base );
	/// cs.set_ts( 100 );
	/// cs.set_ts( 0 );
	/// ```
	#[inline]
	pub fn set_ts_unchecked( &mut self, new_val: u64 ) {
		self.time_stamp = new_val;
		
		self.remove_expired_modifiers();
	}
	
	/// Appends modifier to list of active modifiers. Each ModConf maintains its own list (Vec) of active modifiers.
	/// This method will dispatch modifiers based on output from `modifier.stage()`.
	/// 
	/// # Examples
	/// ```rust
	/// use char_stat::{ CharStat, BaseConf, ModConf, Modifier, ModCommon, ModCalcMode, ModCalcStage, Bounds, RoundingHelper };
	/// 
	/// let bounds_base = Bounds::new_const( 0.0, 1.0 ).unwrap();
	/// let bounds_mod_base = Bounds::new_const( 0.0, 1.0 ).unwrap();
	///
	/// let base = BaseConf::new( 0.5, false, bounds_base, RoundingHelper::new_none(), None ).unwrap();
	/// let mod_conf = Some( ModConf::new( ModCalcStage::Base, bounds_mod_base, RoundingHelper::new_none(), false, false ) );
	/// // mods' value of 1.0 --> + 100%
	/// let common = ModCommon::new( 1.0, ModCalcMode::Mul, ModCalcStage::Base ).unwrap();
	/// let mod_base = Modifier::new_expiring( common, 24 );
	///
	/// let mut cs = CharStat::new( base, None, mod_conf, None, None, None );
	/// assert_eq!( cs.append_modifier( mod_base ), Ok(()) );
	/// assert_eq!( cs.value(), 1.0 );
	/// ```
	/// 
	/// # Errors
	/// `CsMissingComponent::*` when associated `ModConf` or `ModMult` is missing <br>
	#[inline]
	pub fn append_modifier( &mut self, modifier: Modifier ) -> Result<(), CharStatError > {
		match &modifier.calc_stage() {
			ModCalcStage::Base => self.append_base_mod( modifier ),
			ModCalcStage::Upgrade => self.append_upgrade_mod( modifier ),
			ModCalcStage::BasePlusUpgrade => self.append_base_plus_upgrade_mod( modifier ),
			ModCalcStage::ModMult => self.append_modmult( modifier ),
		}?;
		
		self.update_current_value();
		Ok(())
	}
	
	/// returns the value of base with multiplier and modifiers applied
	#[inline]
	pub fn base ( &self ) -> f64 {
		if self.mod_of_base.is_some() {
			return self.val_base + self.val_base_mod
		}
		self.val_base
	}
	
	/// returns the "raw" value of base with multiplier but not modifiers
	#[inline]
	pub fn base_raw ( &self ) -> f64 {
		self.val_base
	}
	
	/// returns the value of upgrade with modifiers applied.
	/// 
	/// # Errors
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn upgrade ( &self ) -> Result< f64, CharStatError > {
		match ( &self.upgrade, &self.mod_of_upgrade ) {
			( Some(_), Some(_) ) => Ok( self.val_upgrade + self.val_upgrade_mod ),
			( Some(_), None ) => Ok( self.val_upgrade ),
			( _, _ ) => Err( CsMissingComponent::Upgrade.into() ),
		}
	}
	
	/// returns the "raw" value of upgrade without modifiers.
	/// 
	/// # Errors
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn upgrade_raw ( &self ) -> Result< f64, CharStatError > {
		if self.upgrade.is_some() {
			return Ok( self.val_upgrade )
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
}

// priv
impl CharStat {
	#[inline]
	#[doc(hidden)]
	fn update_current_value ( &mut self ) {
		self.current_value = self.val_base + self.val_base_mod + self.val_upgrade + self.val_upgrade_mod + self.val_base_plus_upgrade_mod;
	}
	
	#[inline]
	#[doc(hidden)]
	fn append_base_mod( &mut self, modifier: Modifier ) -> Result<(), CharStatError > {
		if let Some( mod_of_base ) = &mut self.mod_of_base {
			mod_of_base.append_mod_unchecked( self.base.value(), modifier );
			self.update_base_mod();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::ModOfBase.into() )
	}
	
	#[inline]
	#[doc(hidden)]
	fn append_upgrade_mod( &mut self, modifier: Modifier ) -> Result<(), CharStatError > {
		if let ( Some( mod_of_upgrade ), Some( upgrade ) ) = ( &mut self.mod_of_upgrade, &mut self.upgrade ) {
			mod_of_upgrade.append_mod_unchecked( upgrade.value(), modifier );
			self.update_upgrade_mod();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::ModOfUpgrade.into() )
	}
	
	#[inline]
	#[doc(hidden)]
	fn append_base_plus_upgrade_mod( &mut self, modifier: Modifier ) -> Result<(), CharStatError > {
		if let Some( tmp ) = &mut self.mod_of_base_plus_upgrade {
			let val = self.val_base + self.val_upgrade;
			tmp.append_mod_unchecked( val, modifier );
			self.update_base_plus_upgrade_mod();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::ModOfBasePlusUpgrade.into() )
	}
	
	#[inline]
	#[doc(hidden)]
	fn append_modmult( &mut self, modifier: Modifier ) -> Result<(), CharStatError > {
		if let Some( mod_mult ) = &mut self.mod_mult {
			mod_mult.append_mod_unchecked( modifier );
			
			self.val_mod_mult = mod_mult.value();
			
			if self.mod_of_base.is_some() {
				self.update_base_mod();
			}
			
			if self.mod_of_upgrade.is_some() {
				self.update_upgrade_mod();
			}
			
			if self.mod_of_base_plus_upgrade.is_some() {
				self.update_base_plus_upgrade_mod();
			}
			
			return Ok(())
		}
		
		Err( CsMissingComponent::ModMult.into() )
	}
	
	#[inline]
	#[doc(hidden)]
	fn remove_expired_modifiers( &mut self ) {
		if let Some( tmp ) = &mut self.mod_mult {
			tmp.remove_expired( self.time_stamp );
			self.val_mod_mult = tmp.value();
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
	}
	
	#[inline]
	#[doc(hidden)]
	fn update_base( &mut self ) {
		self.val_base = self.base.value();
		
		if self.mod_of_base.is_some() {
			self.update_base_mod();
		}
	}
	
	#[inline]
	#[doc(hidden)]
	fn update_base_mod( &mut self ) {
		if let Some( mod_mgr ) = &mut self.mod_of_base {
			mod_mgr.update( self.val_base );
			self.val_base_mod = mod_mgr.value() * self.val_mod_mult;
		}
	}
	
	#[inline]
	#[doc(hidden)]
	fn update_upgrade( &mut self ) {
		if let Some( upgrade ) = &self.upgrade {
			self.val_upgrade = upgrade.value();
			
			if self.mod_of_upgrade.is_some() {
				self.update_upgrade_mod();
			}
			
			if self.mod_of_base_plus_upgrade.is_some() {
				self.update_base_plus_upgrade_mod();
			}
		}
	}
	
	#[inline]
	#[doc(hidden)]
	fn update_upgrade_mod( &mut self ) {
		if let Some( mod_mgr ) = &mut self.mod_of_upgrade {
			mod_mgr.update( self.val_upgrade );
			self.val_upgrade_mod = mod_mgr.value() * self.val_mod_mult;
		}
	}
	
	#[inline]
	#[doc(hidden)]
	fn update_base_plus_upgrade_mod( &mut self ) {
		if let Some( mod_mgr ) = &mut self.mod_of_base_plus_upgrade {
			mod_mgr.update( self.val_base + self.val_upgrade );
			self.val_base_plus_upgrade_mod = mod_mgr.value() * self.val_mod_mult;
		}
	}
}// priv

// base
/// Methods for manipulation of BaseConf
impl CharStat {
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `value` is not within `self.bounds` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.is_mut` is false <br>
	#[inline]
	pub fn set_base_value ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		self.base.set_value( value )?;
		self.update_base();
		self.update_current_value();
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.is_mut` is false <br>
	#[inline]
	pub fn set_base_value_clamping ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		self.base.set_value_clamping( value )?;
		self.update_base();
		self.update_current_value();
		
		Ok(())
	}
	
	#[inline]
	pub fn set_base_value_const ( &mut self ) {
		self.base.set_value_const();
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `new_val` > `self.v_max` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.bounds.is_min_mut` is false <br>
	#[inline]
	pub fn set_base_bounds_min ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.base.set_bounds_min( new_val )?;
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `self.v_min` > `new_val` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.bounds.is_max_mut` is false <br>
	#[inline]
	pub fn set_base_bounds_max ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.base.set_bounds_max( new_val )?;
		
		Ok(())
	}
	
	#[inline]
	pub fn set_base_bounds_min_const ( &mut self ) {
		self.base.set_bounds_min_const();
	}
	
	#[inline]
	pub fn set_base_bounds_max_const ( &mut self ) {
		self.base.set_bounds_max_const();
	}
	
	#[inline]
	pub fn base_bounds_min ( &self ) -> f64 {
		self.base.bounds_min()
	}
	
	#[inline]
	pub fn base_bounds_max ( &self ) -> f64 {
		self.base.bounds_max()
	}
}// base

// base mult
impl CharStat {
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `new_val` is not within `self.bounds` <br>
	/// `CsMissingComponent::BaseMult` when `BaseMultConf` is missing <br>
	#[inline]
	pub fn set_mult_base ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.base.set_mult_base( new_val )?;
		self.update_base();
		self.update_current_value();
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `new_val` is not within `self.bounds` <br>
	/// `CsMissingComponent::BaseMult` when `BaseMultConf` is missing <br>
	#[inline]
	pub fn set_mult_exponent ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.base.set_mult_exponent( new_val )?;
		self.update_base();
		self.update_current_value();
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsMissingComponent::BaseMult` when `BaseMultConf` is missing <br>
	#[inline]
	pub fn set_mult_base_clamping ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.base.set_mult_base_clamping( new_val )?;
		self.update_base();
		self.update_current_value();
		
		Ok(())
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsMissingComponent::BaseMult` when `BaseMultConf` is missing <br>
	#[inline]
	pub fn set_mult_exponent_clamping ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		self.base.set_mult_exponent_clamping( new_val )?;
		self.update_base();
		self.update_current_value();
		
		Ok(())
	}
}// base mult

// upgrade
/// Methods for manipulation of UpgradeConf
impl CharStat {
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsInvalidValue::BelowMinimum` or `CsInvalidValue::AboveMaximum` when `value` is not within `self.bounds` <br>
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn set_upgrade_value ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_value( value )?;
			self.update_upgrade();
			self.update_current_value();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `value` is `f64::NAN` <br>
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn set_upgrade_value_clamping ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_value_clamping( value )?;
			self.update_upgrade();
			self.update_current_value();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
	
	/// Attempts to increment `self.upgrade` by supplied `value`
	/// 
	/// # Errors
	/// `CsInvalidValue::AboveMaximum` when new value is not within `self.bounds` <br>
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn inc_upgrade_value ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_value( upgrade.value() + value )?;
			self.update_upgrade();
			self.update_current_value();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
	
	/// Attempts to decrement `self.upgrade` by supplied `value`
	/// 
	/// # Errors
	/// `CsInvalidValue::BelowMinimum` when new value is not within `self.bounds` <br>
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn dec_upgrade_value ( &mut self, value: f64 ) -> Result<(), CharStatError > {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_value( upgrade.value() - value )?;
			self.update_upgrade();
			self.update_current_value();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `new_val` > `self.v_max` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.bounds.is_min_mut` is false <br>
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn set_upgrade_bounds_min ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_bounds_min( new_val )?;
			
			return Ok(())
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
	
	/// # Errors
	/// `CsInvalidValue::Nan` when `new_val` is `f64::NAN` <br>
	/// `CsLogicIssue::MinGreaterThanMax` when `self.v_min` > `new_val` <br>
	/// `CsLogicIssue::FieldIsConst` when `self.bounds.is_max_mut` is false <br>
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn set_upgrade_bounds_max ( &mut self, new_val: f64 ) -> Result<(), CharStatError > {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_bounds_max( new_val )?;
			
			return Ok(())
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
	
	/// # Errors
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn set_upgrade_bounds_min_const ( &mut self ) -> Result<(), CharStatError > {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_bounds_min_const();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
	
	/// # Errors
	/// `CsMissingComponent::Upgrade` when `UpgradeConf` is missing <br>
	#[inline]
	pub fn set_upgrade_bounds_max_const ( &mut self ) -> Result<(), CharStatError > {
		if let Some( upgrade ) = &mut self.upgrade {
			upgrade.set_bounds_max_const();
			
			return Ok(())
		}
		
		Err( CsMissingComponent::Upgrade.into() )
	}
	
	#[inline]
	pub fn upgrade_bounds_min ( &self ) -> Option< f64 > {
		if let Some( upgrade ) = &self.upgrade {
			return Some( upgrade.bounds_min() )
		}
		
		None
	}
	
	#[inline]
	pub fn upgrade_bounds_max ( &self ) -> Option< f64 > {
		if let Some( upgrade ) = &self.upgrade {
			return Some( upgrade.bounds_max() )
		}
		
		None
	}
}// upgrade

// struct - CharStat
//------------------------------------------------------------------------------
// enum - RoundingFnEnum

#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, Copy, PartialEq )]
pub enum RoundingFnEnum {
	Round,
	RoundTiesEven,
	Floor,
	Ceil,
	Trunk,
	None,
}

impl RoundingFnEnum {
	fn do_rounding( self, val: f64 ) -> f64 {
		match self {
			RoundingFnEnum::Round => val.round(),
			RoundingFnEnum::RoundTiesEven => val.round_ties_even(),
			RoundingFnEnum::Floor => val.floor(),
			RoundingFnEnum::Ceil => val.ceil(),
			RoundingFnEnum::Trunk => val.trunc(),
			RoundingFnEnum::None => val,
		}
	}
}

// enum - RoundingFnEnum
//------------------------------------------------------------------------------
// struct - RoundingHelper

#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Debug, Clone, PartialEq )]
pub struct RoundingHelper {
	function: RoundingFnEnum,
	precision: Option< f64 >,
}

impl RoundingHelper {
	#[inline]
	pub fn new ( function: RoundingFnEnum, mut precision: Option< f64 >, ) -> Self {
		if let Some( val ) = &precision {
			if val.is_nan() {
				precision = None;
			}
		}
		
		RoundingHelper {
			function,
			precision,
		}
	}
	
	#[inline]
	pub fn new_none() -> Self {
		RoundingHelper {
			function: RoundingFnEnum::None,
			precision: None,
		}
	}
	
	pub(crate) fn do_rounding( &self, mut value: f64 ) -> f64 {
		if let RoundingFnEnum::None = self.function {
			return value;
		}
		
		if let Some( prec ) = self.precision {
			value /= prec;
			value = self.function.do_rounding( value );
			
			return value * prec
		}
		
		self.function.do_rounding( value )
	}
}

impl Default for RoundingHelper {
	#[inline]
	fn default() -> Self {
		RoundingHelper {
			function: RoundingFnEnum::None,
			precision: None,
		}
	}
}

// struct - RoundingHelper
//------------------------------------------------------------------------------
// --Tests

#[cfg(test)]
mod char_stat_tests {
	use super::*;
	
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
		let ronud_helper = RoundingHelper::new( RoundingFnEnum::Round, Some( 0.1 ) );
		let mult = BaseMultConf::new( v_mult_b, v_mult_e, bounds_mult_b, bounds_mult_e, ronud_helper ).unwrap();
		
		let bounds_base = Bounds::new_const( 4.0, 20.0 ).unwrap();
		let base = BaseConf::new( v_base, true, bounds_base, RoundingHelper::default(), Some( mult ) ).unwrap();
		
		let bounds_up_mod = Bounds::new_const( -0.5, 1.5 ).unwrap();
		let rounding = RoundingHelper::new( RoundingFnEnum::None, None );
		let up_mod = ModConf::new( ModCalcStage::Upgrade, bounds_up_mod, rounding, true, true );
		
		let bounds_upgr = Bounds::new_const( 0.0, 50.0 ).unwrap();
		let rounding_fn = RoundingHelper::new_none();
		let mut upgrade = UpgradeConf::new( 0.0, bounds_upgr, rounding_fn ).unwrap();
		
		assert_eq!( base.value(), v_base_plus_mlt );
		
		let expected: CharStatError = CsInvalidValue::AboveMaximum( "value".to_string() ).into();
		assert_eq!( upgrade.set_value( 69.0 ), Err( expected ) );
		assert_eq!( upgrade.value(), 0.0 );
		
		assert_eq!( upgrade.set_value( v_upgrade ), Ok(()) );
		
		let mut cs = CharStat::new( base, Some( upgrade ), None, Some( up_mod ), None, None );
		assert_eq!( cs.value(), v_final );
		
		assert_eq!( cs.set_upgrade_value( v_upgrade + 2.0 ), Ok(()) );
		assert_eq!( cs.upgrade_raw(), Ok( v_upgrade + 2.0 ) );
		
		assert_eq!( cs.value(), v_final + 2.0 );
		
		let modifier = Modifier::new_persistent( ModCommon::new( 0.5, ModCalcMode::Mul, ModCalcStage::Upgrade ).unwrap() );
		assert_eq!( cs.append_modifier( modifier ), Ok(()) );
		assert_eq!( cs.value(), v_final + 4.0 );
	}
	
	#[test]
	fn modifiers() {
		let v_base: f64 = 10.0;
		let v_upgrade: f64 = 2.0;
		
		let v_base_mod = 0.1;
		let v_upgrade_mod = 0.25;
		let v_base_and_up_mod = 0.05;
		let v_mod_mult = 1.0;
		
		let common_b = ModCommon::new( v_base_mod, ModCalcMode::Mul, ModCalcStage::Base ).unwrap();
		let common_u = ModCommon::new( v_upgrade_mod, ModCalcMode::Mul, ModCalcStage::Upgrade ).unwrap();
		let common_bau = ModCommon::new( v_base_and_up_mod, ModCalcMode::Mul, ModCalcStage::BasePlusUpgrade ).unwrap();
		let common_mm = ModCommon::new( v_mod_mult, ModCalcMode::Add, ModCalcStage::ModMult ).unwrap();
		
		let mod_base = Modifier::new_expiring( common_b, 24 );
		let mod_upgrade = Modifier::new_expiring( common_u, 49 );
		let mod_base_and_up = Modifier::new_expiring( common_bau, 74 );
		let mod_mod_mlt = Modifier::new_expiring( common_mm, 99 );
		
		let bounds_base = Bounds::new_const( v_base, v_base ).unwrap();
		let bounds_upgrade = Bounds::new_const( v_upgrade, v_upgrade ).unwrap();
		let bounds_mod_base = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let bounds_mod_upgrade = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let bounds_mod_base_plus_upgrade = Bounds::new_const( 0.0, 1.0 ).unwrap();
		let bounds_mod_mult = Bounds::new_const( 0.0, 1.0 ).unwrap();
		
		let rounding_fn = RoundingHelper::new_none();
		
		let base = BaseConf::new( v_base, false, bounds_base, rounding_fn.clone(), None ).unwrap();
		let upgrade = Some( UpgradeConf::new( v_upgrade, bounds_upgrade, rounding_fn.clone() ).unwrap() );
		let mod_of_base = Some( ModConf::new( ModCalcStage::Base, bounds_mod_base, rounding_fn.clone(), false, false ) );
		let mod_of_upgrade = Some( ModConf::new( ModCalcStage::Base, bounds_mod_upgrade, rounding_fn.clone(), false, false ) );
		let mod_of_base_plus_upgrade = Some( ModConf::new( ModCalcStage::Base, bounds_mod_base_plus_upgrade, rounding_fn.clone(), false, false ) );
		let mod_mult = Some( ModMultConf::new( bounds_mod_mult ) );
		
		let mut cs = CharStat::new( base, upgrade, mod_of_base, mod_of_upgrade, mod_of_base_plus_upgrade, mod_mult );
		assert_eq!( cs.append_modifier( mod_base ), Ok(()) );
		assert_eq!( cs.append_modifier( mod_upgrade ), Ok(()) );
		assert_eq!( cs.append_modifier( mod_base_and_up ), Ok(()) );
		assert_eq!( cs.append_modifier( mod_mod_mlt ), Ok(()) );
		
		let mod_mult = 1.0 + v_mod_mult;
		
		let res_base_mod = v_base * ( v_base_mod * mod_mult );
		let res_up_mod = v_upgrade * ( v_upgrade_mod * mod_mult );
		let res_base_and_up_mod = (v_base + v_upgrade) * ( v_base_and_up_mod * mod_mult );
		
		assert_eq!( cs.base(), v_base + res_base_mod );
		assert_eq!( cs.upgrade().unwrap(), v_upgrade + res_up_mod );
		assert_eq!( cs.value(), v_base + v_upgrade + res_base_mod + res_up_mod + res_base_and_up_mod );
		
		assert_eq!( cs.set_ts( 25 ), Ok(()) );
		assert_eq!( cs.base(), v_base );
		assert_eq!( cs.value(), v_base + v_upgrade + res_up_mod + res_base_and_up_mod );
		
		assert_eq!( cs.set_ts( 50 ), Ok(()) );
		assert_eq!( cs.value(), v_base + v_upgrade + res_base_and_up_mod );
		
		assert_eq!( cs.set_ts( 75 ), Ok(()) );
		assert_eq!( cs.value(), v_base + v_upgrade );
	}
	
	#[test]
	fn nan_handling() {
		//let expected: CharStatError = CsInvalidValue::Nan( "value".to_string() ).into();
		
		// RoundingHelper
		let bad_rh = RoundingHelper::new( RoundingFnEnum::Round, Some( f64::NAN ) );
		assert_eq!( bad_rh.precision, None );
		let ronuding_helper = RoundingHelper::new( RoundingFnEnum::Round, Some( 0.1 ) );
		assert_eq!( ronuding_helper.precision, Some( 0.1 ) );
	}
	
	#[cfg( feature = "serde" )]
	#[test]
	fn test_serde() {
		let bounds_base = Bounds::new_const( 4.0, 20.0 ).unwrap();
		let base = BaseConf::new( 10.0, true, bounds_base, RoundingHelper::new_none(), None ).unwrap();
		let cs = CharStat::new_minimal( base );
		
		let serialized = serde_json::to_string( &cs ).unwrap();
		let deserialized: CharStat = serde_json::from_str( &serialized ).unwrap();
		
		assert_eq!( cs, deserialized );
	}
}

// --Tests
//------------------------------------------------------------------------------
