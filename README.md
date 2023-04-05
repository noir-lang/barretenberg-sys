# barretenberg-sys

FFI bindings to Barretenberg

## Dependencies

To leverage the `barretenberg-sys` crate, you'll need to install some global packages:

1. `libomp`

    Usually installable via `brew install libomp` or `apt install libomp-dev`.

2. `pkg-config`

    Usually installable via `brew install pkg-config` or `apt install pkg-config`.

3. `lld`

    Linker provided by Clang, but might need to be installed via `apt install lld`.

4. `barretenberg` (preferably at commit `4a069923f4a869f8c2315e6d3f738db6e66dcdfa`)

    Needs to be built and installed following the instructions [in the README](https://github.com/AztecProtocol/barretenberg#getting-started). Note that barretenberg has its own [dependencies](https://github.com/AztecProtocol/barretenberg#dependencies) that will need to be installed, such as `cmake` and `ninja`.

## Usage

```rust
pub fn pedersen() {
    let input = vec![0; 64];
    barretenberg_sys::blake2s::hash_to_field(&input);

    let f_zero = [0_u8; 32];
    let mut f_one = [0_u8; 32];
    f_one[31] = 1;
    let got = barretenberg_sys::pedersen::compress_native(&f_zero, &f_one);
    assert_eq!(
        "229fb88be21cec523e9223a21324f2e305aea8bff9cdbcb3d0c6bba384666ea1",
        hex::encode(got)
    );
}
```

## Nix

We provide a [Nix Flake](./flake.nix) that shows you how to configure an environment if you are build your Rust code inside Nix.
