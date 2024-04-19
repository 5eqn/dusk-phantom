# Dusk Phantom

## Building

After installing [Rust](https://rustup.rs/), you can install Dusk Phantom (on Linux) as follows:

```shell
make vst3
```

For more info, check `Makefile`.

## Syntax

```dp
let lp: Float -> Float = (i: Float) => if i < 300 then 1 else 0 in
(f: Float -> Float) => (i: Float) => f(i) * lp(i)
```

```dp
// Pitch shift
(f: Freq -> Comp) => (i: Freq) => f(i * 2)

// LP Filter
let lp: (i: Freq) -> Float = (i: Freq) => if i < 1600 Hz then 1 else 0
(f: Freq -> Comp) => (i: Freq) => f(i) * lp(i)

// Spacer
(f: Freq -> Comp) => (i: Freq) => let c = f(i) in comp(c.r, c.t * 2)
```
