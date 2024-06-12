# color converter

A small example at adding a `native func` in Redscript to calculate RGBA from a hex string.

## build

Both [Rust toolchain](https://www.rust-lang.org/tools/install) and [Justfile command runner](https://just.systems/man/en/chapter_4.html) must be pre-installed.

Build binary:

```Powershell
just build
```

Binary will be built into `red4ext\plugins\colorconverter\colorconverter.dll`.
Don't forget to also place `r6\scripts\ColorConverter\Native.reds` in your game folder :)

## usage

```swift
let color: Color = ColorHexToRgba("FF0000");
let color: Color = ColorHexToRgba("#ff00007f");
let color: Color = ColorHexToRgba("aliceblue");
```

It supports any input accepted by [csscolorparser](https://crates.io/crates/csscolorparser).