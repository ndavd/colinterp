# Colinterp

<div align="center">
    <div>
        <div>
            <img align="center" width="400px" src="https://user-images.githubusercontent.com/74260683/233796924-4cd19719-c990-4760-ba94-646a22cc281b.png" />
        </div>
        <div>
            <img align="center" width="400px" src="https://raw.githubusercontent.com/ndavd/colinterp/main/.github/bg.png" />
        </div>
    </div>
</div>

## Web

A web version of this CLI is available using Web Assembly (WASM). https://github.com/ndavd/colinterp-web

You can try it at: https://colinterp.vercel.app

## CLI

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

### Installing

```
cargo install colinterp
```

### Building from source

```
git clone "https://github.com/ndavd/colinterp"
cd colinterp
cargo build --release
./target/release/colinterp
```
