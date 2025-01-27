# Integrate Rust code into Typescript project.

## Step 1:
Write and Compile Rust Code to WebAssembly:
Use Rust and tools like wasm-pack to compile your Rust code to WebAssembly.
Install `wasm-pack` if you haven't already:
```bash
cargo install wasm-pack
```


## Step 2:
Create a Rust Library Project with name `rust_lib`
```bash
cargo new --lib rust_lib
cd rust_lib
```

## Step 3:
Add the following to your `Cargo.toml` file:
```toml
[package]
name = "rust_lib"
version = "0.1.0"
edition = "2021"

[dependencies]
wasm-bindgen = "0.2"

[lib]
crate-type = ["cdylib"]
```

## Step 4:
Write Rust Code:
Create a Rust file (e.g., `src/lib.rs`) with your Rust code.
For example:
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    let result:i32 = a + b;
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(3, 5), 8);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-3, -5), -8);
    }

    #[test]
    fn test_add_mixed_numbers() {
        assert_eq!(add(-3, 5), 2);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_add_large_numbers() {
        assert_eq!(add(1_000_000, 2_000_000), 3_000_000);
    }
}
```

## Step 5:
Compile to WebAssembly with `nodejs` attribute:
If you were writing a package that should be used in Node.js (with CommonJS modules, e.g. require), you would run this in your terminal:
```bash
wasm-pack build --target nodejs
```
for ESM modules, run this in your terminal:
```bash
wasm-pack build --target web
```
This will generate a `pkg` directory containing the compiled WebAssembly code and JavaScript bindings.

## Step 6:
Create a Typescript Project.
```bash
mkdir typescript_exec
cd typescript_exec
npm init -y
npm install --save-dev typescript
npx tsc --init
```
Note: use current `package.json` and `tsconfig.json` file which provide in the repository.

## Step 7:
Copy the `pkg` directory from the Rust project to the Typescript project.
Create a `rust` folder in the Typescript project and copy the `pkg` directory from the Rust project into it. 
Note:`rust` folder will create in the root of the Typescript project.

## Step 8:
Create a `core/index.ts` file in the `src` folder of the Typescript project.
Import the WebAssembly module in your TypeScript code:
```typescript
import {add} from "../../rust/pkg/rust_lib";

export const main = async () => {

  const value1:number = 1;
  const value2:number = 2;

  const result:number=add(value1, value2);

  console.log(result);
};

main();
```

## Step 9:
Run the following command to run the Typescript code:
```bash
npm run debug
```
Note: go to `package.json` and add the following script:
```json
"scripts": {
    "debug": "ts-node-dev --inspect=4321 --pretty --transpile-only ./src/core/index.ts",
}

```