# Counting Macros

This library adds macro to get compile time counters.
It uses procedural macros to implement state **between** macro invocations.

There are a few places this may be useful.
Perhaps if you were defining a series of constant variables that you needed to
increment for each variable.
Of course in that situation it may make more sense to just build those values
at runtime or using some kind of build script.

## Example
```rust
use counting_macros::*;

counter_create!(counter);
let nums = [counter_incr!(counter), counter_incr!(counter),
    counter_incr!(counter)];
assert_eq!(nums, [0, 1, 2]);

counter_set!(counter, -5);
let nums = [counter_incr!(counter), counter_peek!(counter),
    counter_incr!(counter)];
assert_eq!(nums, [-5, -4, -4]);

counter_next!(counter);
assert_eq!(counter_peek!(counter), -2);
```

## Warning
I'm not certain about the stability or safety of this, so I would not
recomend this for use in serious projects.

Additionally there is currently there is no error handling beyond unwraps.

<br>

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
