# colinterp

![Screenshot](https://raw.githubusercontent.com/ndavd/colinterp/main/.github/bg.png)

```
colinterp v0.1.1
Nuno David <email@ndavd.com>

colinterp linearly interpolates 2 colors to generate a palette.


USAGE:
    colinterp [COLOR] [COLOR] [N]

ARGS:
    [COLOR] A color in hexadecimal format (#rrggbb)
    [N]     Number of colors in the generated palette
```

## Installing
```
cargo install colinterp
```

## Building from source

```
git clone "https://github.com/ndavd/colinterp"
cd colinterp
cargo build --release
./target/release/colinterp
```
