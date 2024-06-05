<p align="center">
	<a href="#description">Description</a> •
	<a href="#usage">Usage</a> •
	<a href="#main-components">Main Components</a> •
	<a href="#error-handling">Error Handling</a> •
	<a href="#calculation-stages">Calculation Stages</a> •
	<a href="#rounding-precision">Rounding precision</a> •
	<a href="#versioning">Versioning</a> •
	<a href="#msrv-policy">MSRV policy</a> •
	<a href="#license">License</a> •
</p>

# CharStat ![Static Badge](https://img.shields.io/badge/CharStat_MSRV-1.77-purple) ![Static Badge](https://img.shields.io/badge/Version-0.1.2-purple)

#### **This project is personal/experimental, so use at your own risk!**<br><br>


## Description

A modular game dev library for handling character statistics, inspired by DnD. Allows configuring min/max bounds, mutability and modifiers.<br>
Base value can be modified in 4 ways:
- directly via `set_value` or `set_value_clamping` if not used `.set_value_const()`
- with upgrade - both values are summed
- with multiplier - base *= ( mult.base ^ mult.exp )
- with modifiers


## Usage

```rust
use char_stat::{ CharStat, BaseConf, UpgradeConf, Bounds, RoundingHelper };

// new_const( min, max ) -> Result< Bounds, CharStatError >
let base_bounds = Bounds::new_const( 4.0, 20.0 ).expect( "hardcoded" );

// new( value, is_mut, bounds, rounding, multiplier ) -> Result
let base = BaseConf::new( 10.0, true, base_bounds, RoundingHelper::new_none(), None ).expect( "hardcoded" );

let upgrade_bounds = Bounds::new_const( 0.0, 50.0 ).expect( "hardcoded" );

// new( value, bounds ) -> Result
let upgrade = UpgradeConf::new( 2.0, upgrade_bounds ).unwrap();

let example = CharStat::new_no_mod( base, Some( upgrade ) );

assert_eq!( example.value(), 12.0 )
```

## Main Components

- `CharStat` holds all of the components
	- all constructors require `BaseConf` passed by value
	- all other components are passed as `Option< T >`, which are then turned into `Option< Box< T >>`. Enums allocate the same amount of memory on stack for every variant.
	- 
- `BaseConf` holds the base value
	- value: `f64`
	- adjustable mutability
	- bounds
	- rounding
	- optional multiplier
	- implements `Default`: { value: 0.0, is_mut: true, bounds: Bounds::default(), rounding_fn: RoundingHelper::default() mult: None }
- `BaseMultConf` automatically multiplies base. The idea is to select a static base and then adjust exponent, for example exp points required to level up could use the character level as exponent
	- mult base
	- exponent
	- bounds for base
	- bounds for exponent
	- rounding
	- multiplier - calculated automatically
- `UpgradeConf` simply adds upgrade value to base, always mutable
	- value: `f64`
	- bounds
- `ModConf` configuration for modifiers of a specific calculation stage
	- value - calculated automatically
	- stage: `ModCalcStageEnum`
	- bounds
	- rounding
	- min/max can be interpreted as a percent of the modified value
	- vector of modifiers
- `ModMultConf` modifier multiplier, affects value of all modifiers
	- value - calculated automatically
	- bounds
	- vector of modifiers
- `Modifier` object describing modifiers
	- value: `f64`
	- expiration timestamp: `Option< u64 >`
	- mode: `ModCalcModeEnum`
	- stage: `ModCalcStageEnum`

Other Components

- `ModCalcStageEnum` variants: Base, Upgrade, BasePlusUpgrade, ModMult
- `ModCalcModeEnum`
	- Add - value of `Modifier` is added to the total
	- Sub - value of mod is subtracted from the total
	- Mul - the total value is increased by base value multiplied by mod
	- Div - the total value is increased by base value divided by mod
- `Bounds` holds min/max values and whether they are mutable. Once disabled mutability cannot be re-enabled.
	- implements `Default`: { mut min: 0.0, mut max: 1.0 }
- `RoundingHelper` function is chosen by enum, precision of N rounds to multiples of N
	- function: `RoundingFunctionEnum`
	- precision: `Option< f64 >`
	- implements `Default`: { function: RoundingFunctionEnum::None, precision: None }
- `RoundingFunctionEnum` variants: Round, RoundTiesEven, Floor, Ceil, Trunk, None


## Error Handling

CharStat uses custom enums which implement `std:error:Error` trait.
- `CharStatError` - public facing type, wrapper for other types
- `CsLogicIssue`: InvalidModifierStage, InvalidModifierMode, MinGreaterThanMax, FieldIsConst, TimeTravel
- `CsInvalidValue`: BelowMinimum, AboveMaximum, Nan
- `CsMissingObject`: BaseMult, Upgrade, ModOfBase, ModOfUpgrade, ModOfBasePlusUpgrade, ModMult,


## Calculation Stages

1. Modifier Multiplier
2. Base value
3. Base Multiplier ( unlike modifiers it changes the effective value used during calculations )
4. Modifier of Base
5. Upgrade
6. Modifier of Upgrade
7. Modifier of Base + Upgrade


## Rounding precision

Default = 1.0<br>
Algoritm: round_fn( value / precision ) * precision<br>

Example:<br>
`let value = 1.55;`<br>
`let precision = 0.1;`<br>
1. `1.55 / 0.1 = 15.5`
2. `round( 15.5 ) = 16`
3. `16 * 0.1 = 1.6`


## Versioning

This project uses <a href="https://semver.org">SemVer 2.0.0</a>


## MSRV policy

During development MSRV may be changed at any time. It will increase the minor version.
Upon reaching 1.0.0, increasing MSRV will be considered a breaking change, and will increase a major version.


## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
