# Tompac

`tompac` is a small Rust utility that converts a simple, machine-readable
TOML description of your preferred pacman setup into a ready-to-drop
`pacman.conf`.

It is handy when you want to version-control your Arch Linux package
manager configuration or generate it during provisioning.

---

## Status

⚠️  Experimental — the public interface is **not** stable yet.

Current capabilities:

* parses `config.example.toml` (path hard-coded in `src/main.rs`)
* merges the values into `pacmanconf` data structures
* writes the resulting config to `ok.conf`

Planned improvements:

* CLI flags to choose input/output paths
* honour the `packages` list by installing them via libalpm
* tests and CI

---

## Quick start

```bash
# build binary
git clone <this-repo> && cd tompac
cargo build --release

# prepare config file
cp config.example.toml mypc.toml
$EDITOR mypc.toml

# generate pacman.conf
# (temporarily: the binary always reads config.example.toml)
./target/release/tompac
```

Inspect the generated `ok.conf` and, if you are happy with it, copy it
into place:

```bash
sudo install -m644 ok.conf /etc/pacman.conf
```

---

## Configuration file layout

See [`config.example.toml`](./config.example.toml) for the full schema.
Important keys:

| Key | Purpose |
| --- | ------- |
| `config.repositories` | Extra repositories to append. |
| `config.pacman` | Mirrors the `[options]` section of `pacman.conf`. |
| `config.packages` | (unused for now) packages to ensure are installed. |

Unset values are commented out in the generated file so you can clearly
see what is left at its default.

---

## Building / Installing

Rust 1.78+ (edition 2024) is required.

```bash
cargo install --path .
```

---

## Contributing

Bug reports and pull requests are welcome. Please open an issue first if
you plan to work on a bigger change.

---

## License

Licensed under the MIT License. See [LICENSE](./LICENSE) for details.
