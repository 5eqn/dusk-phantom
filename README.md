# Dusk Phantom

## Building

After installing [Rust](https://rustup.rs/), you can install Dusk Phantom (on Linux) as follows:

```shell
make vst3
```

For more info, check `Makefile`.

## Usage

1. Change environment variable `DUSK_PHANTOM_PATH` to your code directory
  - REMOVE TRAILING `/`
2. Write `{profile}.dft`
  - `{profile}` should be replaced to the actual profile you use, for example `1`
3. Reload code manually
  - You can see compilation results in the GUI

## Syntax

Low pass:

```dp
let lp: Float -> Float -> Float = (l: Float) => (i: Float) => 
  if i < l then 1 else 0 in
(f: Float -> Float) => (i: Float) => f(i) * lp(10)(i)
```

Band pass:

```dp
let bp: Float -> Float -> Float -> Float = 
  (l: Float) => 
  (r: Float) =>
  (i: Float) => 
  if i < l then 0 else 
  if i > r then 0 else 1 in
(f: Float -> Float) => (i: Float) => f(i) * bp(25)(50)(i)
```

Pitcher:

```dp
(f: Float -> Float) => (i: Float) => f(i * 2)
```

Noise:

```dp
(f: Float -> Float) => (i: Float) => 20 / (i + 50)
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
