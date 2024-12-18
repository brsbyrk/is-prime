# Is Prime - WebAssembly Module

This project provides a WebAssembly (WASM) module for prime number operations, including checking if a number is prime and retrieving the nth prime number. The project leverages the Rust `primal` crate for efficient prime number computations and exports functions through WebAssembly for use in web and other environments.

## Features

- **`is_prime(n: u64) -> bool`:** Check if a given number is prime.
- **`nth_prime(n: u64) -> u64`:** Retrieve the nth prime number.

## Getting Started

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install).
- Install the `wasm-pack` tool for building the WASM package:

  ```bash
  cargo install wasm-pack
  ```

### Installation

- Clone the repository:

```bash
git clone https://github.com/yourusername/is-prime.git
cd is-prime
```

- Build the WebAssembly package using wasm-pack:

```bash
wasm-pack build --target web
```

- Use the generated WASM module in your project. The generated files will be available in the pkg directory.

### Usage

> **Warning:** Ensure to use `BigInt` for representing `u64` values in JavaScript. Regular numbers in JavaScript may not accurately represent large `u64` values, leading to incorrect results.

```js
import init, { is_prime, nth_prime } from "./pkg/is_prime.js";

(async () => {
  await init();

  console.log(is_prime(BigInt(7)); // true
  console.log(is_prime(BigInt(10))); // false
  console.log(nth_prime(BigInt(5))); // 11
})();
```

> **Note:** To run this html code, you need to serve it from a web server. Opening the file directly in the browser may not work due to security restrictions on loading WebAssembly modules. You can use a simple HTTP server like `python -m http.server` to serve the file safely.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>Is Prime</title>
    <script type="module">
      import init, { is_prime, nth_prime } from "./pkg/is_prime.js";

      (async () => {
        await init();

        console.log(is_prime(7)); // true
        console.log(is_prime(10)); // false
        console.log(nth_prime(5)); // 11
      })();
    </script>
  </head>
  <body>
    <h1>Is Prime</h1>
  </body>
</html>
```
