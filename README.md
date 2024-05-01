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

The whole term evaluates to a `Float -> (Float, Float)`, representing FFT spectrogram.

Pitcher:

```dp
(i: Float) => fft(i * 2)
```

Noise:

```dp
(i: Float) => (20 / (i + 50), 0)
```

Low pass:

```dp
let lp: Float -> Float -> Float = (l: Float) => (i: Float) => 
  if i < l then 1 else 0 in
(i: Float) => fft(i) * lp(10)(i)
```

Band pass:

```dp
let bp: Float -> Float -> Float -> Float = 
  (l: Float) => 
  (r: Float) =>
  (i: Float) => 
  if i < l then 0 else 
  if i > r then 0 else 1 in
(i: Float) => fft(i) * bp(25)(50)(i)
```

## Library Function

- `fft(i)`: frequency and phase at band `i`
- `param(i)`: value of param "Mod i"
- `beat`: current beat count in float
- `sec`: current second in float