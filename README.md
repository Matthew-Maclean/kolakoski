# Kolakoski sequence iterator

Information about the Kolakoski sequence can be found at the OEIS page for it: https://oeis.org/A000002

In `test.rs`, the first 100 number of the sequence are used for testing. These numbers come from https://oeis.org/A000002.

Warning! Attempting to get too many numbers from the iterator may cause a stack overflow. If the test fails for that reason, lower the number of items take to test like so:

    // Too many
    for (i, n) in Kolakoski::new().take(10_000_000)

    // A better amount
    for (i, n) in Kolakoski::new().take(20)

More that 100 will also fail, because the test only includes the first 100 number for comparison.

## Features:

### `default`
The `default` feature makes `Kolakoski` iterate over `u8`, this feature is `no_std` also.

### `num-traits`
The `num-traits` feature makes `Kolakoski` iterate generically over any type that implements `num::traits::Num`. This includes a dependency of `num v1.4`, and makes the crate no longer `no_std`.