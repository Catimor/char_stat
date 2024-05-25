
# CharStat ![Static Badge](https://img.shields.io/badge/CharStat_MSRV-1.72-purple) ![Static Badge](https://img.shields.io/badge/Version-0.1.1-purple)



## Status

This project is personal/experimental, so use at your own risk.
Currently uses unit err `Result<_, ()>` instead of custom error types.


## Description

A modular game dev library for handling character statistics, inspired by DnD. Allows configuring min/max bounds, mutability and modifiers.


## Usage

```rust
// new_const( min, max ) -> Option< Bounds >
let base_bounds = Bounds::new_const( 4.0, 20.0 ).expect( "hardcoded" );

// new( value, is_mut, bounds, rounding, multiplier ) -> Option
let base = BaseConf::new( 10.0, true, base_bounds, RoundingHelper::new_none(), None ).expect( "hardcoded" );

let upgrade_bounds = Bounds::new_const( 0.0, 50.0 ).expect( "hardcoded" );
let upgrade = UpgradeConf::new( 2.0, upgrade_bounds );

let example = CharStat::new_no_mod( base, upgrade );

assert_eq!( example.value(), 12.0 )
```

## Components
- `BaseConf` holds the base value
	- value: `f64`
	- adjustable mutability
	- bounds
	- optional rounding
	- optional multiplier
- `BaseMultConf` automatically multiplies base. The idea is to select a static base and then adjust exponent, for example exp points required to level up could use the character level as exponent
	- base value
	- exponent
	- bounds for base
	- bounds for exponent
	- optional rounding
	- multiplier - calculated automatically
- `UpgradeConf` simply adds upgrade value to base, always mutable
	- value: `f64`
	- bounds
- `ModConf` configuration for modifiers of a specific calculation stage
	- value - calculated automatically
	- stage: `ModCalcStageEnum`
	- bounds
	- optional rounding
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
- `ModCalcStageEnum` variants: Base, Upgrade, BasePlusUpgrade, ModMult
- `ModCalcModeEnum`
	- Add - value of `Modifier` is added to the total
	- Sub - value of mod is subtracted from the total
	- Mul - the total value is increased by base value multiplied by mod
	- Div - the total value is increased by base value divided by mod
- `Bounds` holds min/max values and whether they are mutable. Once disabled mutability cannot be re-enabled.
- `RoundingHelper` function is chosen by enum, precision of N rounds to multiples of N
	- function: `RoundingFunctionEnum`
	- precision: `Option< f64 >`
- `RoundingFunctionEnum` variants: Round, RoundTiesEven, Floor, Ceil, Trunk, None

## Calculation Stages
1. Modifier Multiplier
2. Base value
3. Base Multiplier ( unlike modifiers it changes the effective value used during calculations )
4. Modifier of Base
5. Upgrade
6. Modifier of Upgrade
7. Modifier of Base + Upgrade


## Versioning
We use <a href="https://semver.org">SemVer</a> for versioning.


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
