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

# CharStat ![Static Badge](https://img.shields.io/badge/CharStat_MSRV-1.77-purple) ![Static Badge](https://img.shields.io/badge/Version-0.1.3-purple)

### **This project is personal/experimental and NOT production ready. Use at your own risk!**


## Description

CharStat is a flexible game dev library for handling character statistics, inspired by DnD. <br>
It allows configuring min/max bounds, modifiers, rounding and whether setter methods are allowed. <br>
Aside from `BaseConf` every other component is optional, allowing easy composability.


## Usage

```rust
use char_stat::{ CharStat, BaseConf, UpgradeConf, Bounds, RoundingHelper };

let base_val = 10.0;
let up_val = 2.0;

// new_const( min, max ) -> Result< Bounds, CharStatError >
let base_bounds = Bounds::new_const( 4.0, 20.0 ).unwrap();

// new( value, is_mut, bounds, rounding, multiplier ) -> Result
let base = BaseConf::new( base_val, true, base_bounds, RoundingHelper::new_none(), None ).unwrap();

let upgrade_bounds = Bounds::new_const( 0.0, 50.0 ).unwrap();

// new( value, bounds ) -> Result
let upgrade = UpgradeConf::new( up_val, upgrade_bounds ).unwrap();

let example = CharStat::new_no_mod( base, Some( upgrade ) );

assert_eq!( example.value(), base_val + up_val )
```

#### Example explained

`Bounds` is used to restrics parent objects' minimum and maximum value. Mutability of each bound can be disabled, <br>
`RoundingHelper` is used to configure ( or disable ) the rounding of parent objects' value, <br>
`BaseConf` holds the base value, whether it is mutable and a few helper structs, <br>
`UpgradeConf` is an optional component, value is added to the total, <br>
`CharStatError` is a container for more specific error types, <br>

## Main Components

- `CharStat` holds all of the components
	- all constructors require `BaseConf` passed by value
	- all other components are passed as `Option< T >`, which are then turned into `Option< Box< T >>`. Box is used because enums allocate the same amount of memory on stack for every variant.
	- 
- `BaseConf` holds the base value
	- value: `f64`
	- adjustable mutability
	- bounds
	- rounding
	- optional multiplier
	- implements `Default`: { value: 0.0, is_mut: true, bounds: bounds.clone(), rounding_fn: RoundingHelper::default() mult: None }
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
	- stage: `ModCalcStage`
	- bounds
	- rounding
	- min/max can be interpreted as a percent of the modified value
	- vector of modifiers
- `ModMultConf` modifier multiplier, affects value of all modifiers
	- value - calculated automatically
	- bounds
	- vector of modifiers

#### Other Components

- `Modifier` object describing modifiers
	- common: `ModCommon`
	- mod_type: `ModType`
- `ModCommon` common fields
	- value: `f64`
	- mode: `ModCalcMode`
	- stage: `ModCalcStage`
- `ModType` determines the functionality
	- Expiring
		- exp_ts: `u64`
	- Persistent
	- Stacked
		- conf: `ModStackConf`
- `ModStackConf` handles provides functionality for Stacked variant
- `ModCalcStage` variants: Base, Upgrade, BasePlusUpgrade, ModMult
- `ModCalcMode`
	- Add - value of `Modifier` is added to the total
	- Sub - value of mod is subtracted from the total
	- Mul - the total value is increased by base value multiplied by mods' value
	- Div - the total value is increased by base value divided by mods' value
- `Bounds` holds min/max values and whether they are mutable. Once disabled mutability cannot be re-enabled.
	- implements `Default`: { mut min: 0.0, mut max: 1.0 }
- `RoundingHelper` function is chosen by enum, precision of N rounds to multiples of N
	- function: `RoundingFnEnum`
	- precision: `Option< f64 >`
	- implements `Default`: { function: RoundingFnEnum::None, precision: None }
- `RoundingFnEnum` variants: Round, RoundTiesEven, Floor, Ceil, Trunk, None


## Error Handling

CharStat uses custom enums which implement `std:error:Error` trait.
- `CharStatError` - public facing type, wrapper for other types
- `CsLogicIssue`: InvalidModifierStage, InvalidModifierMode, InvalidModifierType, MinGreaterThanMax, FieldIsConst, TimeTravel
	> here, "Time travel" refers to a situation where new timestamp is lower ( refert to earlier point in time ) than the one already stored
- `CsInvalidValue`: BelowMinimum, AboveMaximum, Nan
- `CsMissingObject`: BaseMult, Upgrade, ModOfBase, ModOfUpgrade, ModOfBasePlusUpgrade, ModMult,


## Calculation Stages

1. Modifier Multiplier
2. Base value
3. Base Multiplier ( unlike modifiers it changes the effective value used during later stages )
4. Upgrade
5. Modifier of Base
6. Modifier of Upgrade
7. Modifier of Base + Upgrade


## Rounding precision

Default (None) = 1.0<br>
Algoritm: round_fn( value / precision ) * precision<br>

Example:<br>
`let value = 1.56;`<br>
`let precision = 0.1;`<br>
1. `1.56 / 0.1 = 15.6`
2. `round( 15.6 ) = 16`
3. `16 * 0.1 = 1.6`


## Versioning

This project uses <a href="https://semver.org">SemVer 2.0.0</a>


## MSRV policy

During development MSRV may be changed at any time. It will increase the minor version.<br>
Upon reaching 1.0.0, increasing MSRV will be considered a breaking change, and will increase the major version.


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
