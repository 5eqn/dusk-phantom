# Dusk Phantom

## Building

After installing [Rust](https://rustup.rs/), you can install Dusk Phantom (on Linux) as follows:

```shell
make vst3
```

For more info, check `Makefile`.

## Syntax

```dp
// Pitch shift
(f: [i: freq] -> complex) => [i: freq] => f[i * 2]

// LP Filter
let lp: (i: freq) -> float = (i: freq) => if i < 1600 Hz then 1 else 0
(f: [i: freq] -> complex) => [i: freq] => f[i] * lp(i)

// Spacer
(f: [i: freq] -> complex) => [i: freq] => rt(f[i].r, f[i].t * 2)
```
