# Currently just a prototype!

A game dev library for handling character statistics.


#### Example

```
let base_bounds = Bounds::new_const( 4.0, 20.0 ).expect("hardcoded");
let base = BaseConf::new( 10.0, true, base_bounds, RoundingHelper::new_none(), None ).expect("hardcoded");

let upgrade_bounds = Bounds::new_const( 0.0, 50.0 ).expect("hardcoded");
let upgrade = UpgradeConf::new( 0.0, upgrade_bounds );

let example = CharStat::new_no_mod( base, upgrade );
```

#### License

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
