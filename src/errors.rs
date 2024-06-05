use std::{ error::Error, fmt::Display };

use crate::{ ModCalcStageEnum, ModCalcModeEnum };

#[derive( Debug, Clone, PartialEq, Eq )]
pub enum CharStatError {
	LogicIssue( CsLogicIssue ),
	InvalidValue( CsInvalidValue ),
	MissingComponent( CsMissingComponent ),
	Other( String ),
}// CharStatError

impl From< CsLogicIssue > for CharStatError {
	#[inline]
	fn from( value: CsLogicIssue ) -> Self {
		CharStatError::LogicIssue( value )
	}
}// from CsLogicIssue

impl From< CsInvalidValue > for CharStatError {
	#[inline]
	fn from( value: CsInvalidValue ) -> Self {
		CharStatError::InvalidValue( value )
	}
}// from CsInvalidValue

impl From< CsMissingComponent > for CharStatError {
	#[inline]
	fn from( value: CsMissingComponent ) -> Self {
		CharStatError::MissingComponent( value )
	}
}// from CsMissingObject

impl Default for CharStatError {
	#[inline]
	fn default() -> Self {
		CharStatError::Other( "undefined error".to_string() )
	}
}// CharStatError - Default

impl Display for CharStatError {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::LogicIssue( tmp ) => tmp.to_string(),
			Self::InvalidValue( tmp ) => tmp.to_string(),
			Self::MissingComponent( tmp ) => tmp.to_string(),
			Self::Other( tmp ) => tmp.to_owned(),
		}.fmt(f)
	}
}// CharStatError - Display

impl Error for CharStatError {}

//----------------------------------------------------
#[derive( Debug, Clone, PartialEq, Eq )]
pub enum CsLogicIssue {
	InvalidModifierStage( ModCalcStageEnum ),
	InvalidModifierMode( ModCalcModeEnum ),
	MinGreaterThanMax,
	FieldIsConst,
	TimeTravel,
}// CsLogicIssue

impl Display for CsLogicIssue {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidModifierStage( bad ) => write!( f, "invalid modifier calculation stage: {bad}", ),
			Self::InvalidModifierMode( bad ) => write!( f, "invalid modifier calculation mode: {bad}", ),
			Self::MinGreaterThanMax => "invalid bounds: min cannot be greater than max".fmt(f),
			Self::FieldIsConst => "cannot mutate a const property".fmt(f),
			Self::TimeTravel => "invalid timestamp - cannot move back in time".fmt(f),
		}
	}
}// CsLogicIssue - Display

impl Error for CsLogicIssue {}

//----------------------------------------------------
#[derive( Debug, Clone, PartialEq, Eq )]
pub enum CsInvalidValue {
	BelowMinimum( String ),
	AboveMaximum( String ),
	Nan( String ),
}// CsInvalidValue

impl Display for CsInvalidValue {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut tmp = match self {
			Self::BelowMinimum( name ) | Self::AboveMaximum( name ) | Self::Nan( name ) => name,
		}.clone();
		
		tmp.push_str( " cannot be " );
		
		tmp.push_str( match self {
			Self::BelowMinimum( _ ) => "lower than min",
			Self::AboveMaximum( _ ) => "greater than max",
			Self::Nan( _ ) => "NAN",
		} );
		
		tmp.fmt(f)
	}
}// CsInvalidValue - Display

impl Error for CsInvalidValue {}

//----------------------------------------------------
#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum CsMissingComponent {
	BaseMult,
	Upgrade,
	ModOfBase,
	ModOfUpgrade,
	ModOfBasePlusUpgrade,
	ModMult,
}// CsMissingObject

impl Display for CsMissingComponent {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut tmp = String::from( "missing object: " );
		
		tmp.push_str( match self {
			Self::BaseMult => "BaseMult",
			Self::Upgrade => "Upgrade",
			Self::ModOfBase => "ModOfBase",
			Self::ModOfUpgrade => "ModOfUpgrade",
			Self::ModOfBasePlusUpgrade => "ModOfBasePlusUpgrade",
			Self::ModMult => "ModMult",
		} );
		
		tmp.fmt(f)
	}
}// CsMissingObject - Display

impl Error for CsMissingComponent {}
