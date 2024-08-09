use std::{ error::Error, fmt::{ Display, Formatter } };

// --Imports
//------------------------------------------------------------------------------
// --Modules

use crate::{ ModCalcMode, ModCalcStage, ModType };

// --Modules
//------------------------------------------------------------------------------
// enum - CharStatError

#[derive( Debug, Clone, PartialEq )]
pub enum CharStatError {
	LogicIssue( CsLogicIssue ),
	InvalidValue( CsInvalidValue ),
	MissingComponent( CsMissingComponent ),
	Other( String ),
}

impl CharStatError {
	#[inline]
	pub fn custom< S: Into< String >> ( msg: S ) -> Self {
		Self::Other( msg.into() )
	}
}

impl From< CsLogicIssue > for CharStatError {
	#[inline]
	fn from( value: CsLogicIssue ) -> Self {
		CharStatError::LogicIssue( value )
	}
}

impl From< CsInvalidValue > for CharStatError {
	#[inline]
	fn from( value: CsInvalidValue ) -> Self {
		CharStatError::InvalidValue( value )
	}
}

impl From< CsMissingComponent > for CharStatError {
	#[inline]
	fn from( value: CsMissingComponent ) -> Self {
		CharStatError::MissingComponent( value )
	}
}

impl Default for CharStatError {
	#[inline]
	fn default() -> Self {
		CharStatError::Other( "undefined error".to_string() )
	}
}

impl Display for CharStatError {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::LogicIssue( tmp ) => tmp.to_string(),
			Self::InvalidValue( tmp ) => tmp.to_string(),
			Self::MissingComponent( tmp ) => tmp.to_string(),
			Self::Other( tmp ) => tmp.to_owned(),
		}.fmt(f)
	}
}

impl Error for CharStatError {}

// enum - CharStatError
//------------------------------------------------------------------------------
// enum - CsLogicIssue

#[derive( Debug, Clone, PartialEq )]
pub enum CsLogicIssue {
	InvalidModifierStage( ModCalcStage, ModCalcStage ),
	InvalidModifierMode( ModCalcMode, Vec< ModCalcMode > ),
	InvalidModifierType( ModType, String ),
	MinGreaterThanMax,
	FieldIsConst,
	TimeTravel,
}

impl Display for CsLogicIssue {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidModifierStage( bad, good ) => write!( f, "invalid modifier calculation stage - found: {bad}, expected: {good}", ),
			Self::InvalidModifierMode( bad, good ) => write!( f, "invalid modifier calculation mode - found: {bad}, expected: {}", vec_to_csv_string( good ) ),
			Self::InvalidModifierType( bad, good ) => write!( f, "invalid modifier type - found: {bad}, expected: {good}", ),
			Self::MinGreaterThanMax => "invalid bounds: min cannot be greater than max".fmt(f),
			Self::FieldIsConst => "cannot mutate a const property".fmt(f),
			Self::TimeTravel => "invalid timestamp - cannot move back in time".fmt(f),
		}
	}
}

#[inline]
fn vec_to_csv_string< T: ToString >( vec: &[ T ] ) -> String {
	let mut out = String::new();
	
	if let Some( tmp ) = vec.first() {
		out.push_str( &tmp.to_string() );
	} else {
		return out
	}
	
	for i in 1..vec.len() {
		if let Some( elem ) = vec.get( i ) {
			out.push_str( ", " );
			out.push_str( &elem.to_string() );
		}
	}
	
	out
}

impl Error for CsLogicIssue {}

// enum - CsLogicIssue
//------------------------------------------------------------------------------
// enum - CsInvalidValue

#[derive( Debug, Clone, PartialEq, Eq )]
pub enum CsInvalidValue {
	BelowMinimum( String ),
	AboveMaximum( String ),
	CannotBeZero( String ),
	Nan( String ),
}

impl Display for CsInvalidValue {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut tmp = match self {
			Self::BelowMinimum( name ) | Self::AboveMaximum( name ) | Self::CannotBeZero( name ) | Self::Nan( name ) => name,
		}.clone();
		
		tmp.push_str( " cannot be " );
		
		tmp.push_str( match self {
			Self::BelowMinimum( _ ) => "lower than min",
			Self::AboveMaximum( _ ) => "greater than max",
			Self::CannotBeZero( _ ) => "equal to zero",
			Self::Nan( _ ) => "NAN",
		} );
		
		tmp.fmt(f)
	}
}

impl Error for CsInvalidValue {}

// enum - CsInvalidValue
//------------------------------------------------------------------------------
// enum - CsMissingObject

#[derive( Debug, Clone, Copy, PartialEq, Eq )]
pub enum CsMissingComponent {
	BaseMult,
	Upgrade,
	ModOfBase,
	ModOfUpgrade,
	ModOfBasePlusUpgrade,
	ModMult,
}

impl Display for CsMissingComponent {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
}

impl Error for CsMissingComponent {}

// enum - CsMissingObject
//------------------------------------------------------------------------------
