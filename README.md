# barretenberg-sys

FFI bindings to Barretenberg

## Dependencies

This is a [`*-sys` package](https://doc.rust-lang.org/cargo/reference/build-scripts.html#-sys-packages), which means it will
probe the system for Barretenberg, link to it, and provide higher-level bindings that can be consumed by other Rust projects.

We provide a comprehensive [Nix Flake](./flake.nix) for consuming Barretenberg within Rust projects, but if you instead want
to install everything globally, you'll need:

1. `libomp`

    Usually installable via `brew install libomp` or `apt install libomp-dev`.

2. `pkg-config`

    Usually installable via `brew install pkg-config` or `apt install pkg-config`.

3. `lld`

    Linker provided by Clang, but might need to be installed via `apt install lld`.

4. `barretenberg` (preferably at commit `87aeb375d7b434e0faf47abb79f97753ab760987`)

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

## Working on this project

Due to the large number of native dependencies, this project uses [Nix](https://nixos.org/) and [direnv](https://direnv.net/) to streamline the development experience.

### Setting up your environment

For the best experience, please follow these instructions to setup your environment:
1. Install Nix following [their guide](https://nixos.org/download.html) for your operating system
2. Create the file `~/.config/nix/nix.conf` with the contents:
```ini
experimental-features = nix-command
extra-experimental-features = flakes
```
3. Install direnv into your Nix profile by running:
```sh
nix profile install nixpkgs#direnv
```
4. Add direnv to your shell following [their guide](https://direnv.net/docs/hook.html)
5. Restart your shell

### Shell & editor experience

Now that your environment is set up, you can get to work on the project.

1. Clone the repository, such as:
```sh
git clone git@github.com:noir-lang/barretenberg-sys
```
2. Navigate to the directory:
```sh
cd barretenberg-sys
```
3. You should see a __direnv error__ because projects aren't allowed by default. Make sure you trust our `.envrc` file, then you need to run:
```sh
direnv allow
```
4. Now, wait awhile for all the native dependencies to be built. This will take some time and direnv will warn you that it is taking a long time, but we just need to let it run.
5. Once you are presented with your prompt again, you can start your editor within the project directory (we recommend [VSCode](https://code.visualstudio.com/)):
```sh
code .
```
6. (Recommended) When launching VSCode for the first time, you should be prompted to install our recommended plugins. We highly recommend installing these for the best development experience.

### Building and testing

Assuming you are using `direnv` to populate your environment, building and testing the project can be done
with the typical `cargo build`, `cargo test`, and `cargo clippy` commands. You'll notice that the `cargo` version matches the version we specify in [flake.nix](./flake.nix), which is 1.66.0 at the time of this writing.

If you want to build the entire project in an isolated sandbox, you can use Nix commands:
1. `nix build .` (or `nix build . -L` for verbose output) to build the project in a Nix sandbox
2. `nix flake check` (or `nix flake check -L` for verbose output) to run clippy and tests in a Nix sandbox

### Building against a different local/remote version of Barretenberg

If you are working on this crate, it is likely that you want to incorporate changes from some other version of Barretenberg
instead of the version this project is pinned against.

To reference a different version of Barretenberg, you want to replace the lockfile version with your version. This can be done
by running:

```sh
nix flake lock --override-input barretenberg /absolute/path/to/your/barretenberg
```

You can also point at a fork and/or branch on GitHub using:

```sh
nix flake lock --override-input barretenberg github:phated/barretenberg/mybranch
```

__Note:__ You don't want to commit the updated lockfile, as it will fail in CI!

### Without direnv

If you have hesitations with using `direnv`, you can launch a subshell with `nix develop` and then launch your editor
from within the subshell. However, if VSCode was already launched in the project directory, the environment won't be updated.

__Advanced:__ If you aren't using `direnv` nor launching your editor within the subshell, you can try to install barretenberg and other global dependencies the package needs. This is an advanced workflow and likely won't receive support!
