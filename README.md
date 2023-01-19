# Rust btw

## Setting up

Run `rustup update` to get the latest version of `Rust`

After installing `Rust`, you can use Cargo to install trunk by running:

```sh
cargo install --locked trunk
```

We will also need to add the WASM build target by running:

```sh
rustup target add wasm32-unknown-unknown
```

### Setting up the project

First clone the project and get into it's folder

```sh
git@github.com:Sloth-Wizard/yew-test.git
cd yew-test
```

Then run

```sh
trunk serve
```

You will maybe see a warning displayed in the console telling us to convert the `document` static to `DOCUMENT` but this does not work for `wasm_bindgen`.
Keeping `document` in lowercase is important.
