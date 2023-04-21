# colinterp

```
colinterp v0.1.0
Nuno David <email@ndavd.com>

colinterp linearly interpolates 2 colors to generate a palette.


USAGE:
    colinterp [COLOR] [COLOR] [N]

ARGS:
    [COLOR] A color in hexadecimal format (#rrggbb)
    [N]     Number of colors in the generated palette
```

## Building from source

```
git clone "https://github.com/ndavd/colinterp"
cd colinterp
cargo build --release
./target/release/colinterp
```
