# V10 JavaScript/TypeScript Runtime
**Please mind that this is currently in development! It is not fast like bun, widely supported like node, im still working on making it better!**
 

***TODO: Add support for imports es5 and es6***

## Table of Contents
- [Description](#description)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Example](#example)
- [Installation](#installation)
- [Contributing](#contributing)
- [License](#license)

#### Description
V10 is a Rust-based engine designed to process and execute JavaScript and TypeScript files. It uses the Boa JavaScript engine for script execution and provides a simple interface for running scripts and displaying information about the engine.

#### Features
- **TypeScript to JavaScript Conversion**: Strips TypeScript types and converts `.ts` files to `.js` files.
- **JavaScript Execution**: Executes JavaScript files using the Boa engine.
- **Temporary File Management**: Creates and deletes temporary JavaScript files generated from TypeScript files.
- **Engine Information**: Displays information about the V10 engine.
- **Debugger**: Provides a simple debugger for debugging JavaScript files. Just add the word `debug` to the end of your `V10` command!
```shell
V10 examples/test.ts debug
```

#### Prerequisites
- Rust (latest stable version)
- Cargo (latest stable version)

#### Usage
1. Run a JavaScript file:
   ```sh
   cargo run path/to/yourfile.js
   ```

2. Run a TypeScript file:
   ```sh
   cargo run path/to/yourfile.ts
   ```

3. Display engine information:
   ```sh
   cargo run version
   ```

#### Project Structure
- `src/main.rs`: Entry point of the application.
- `src/engine/engine.rs`: Contains the `Engine` struct and methods for running and interpreting JavaScript.
- `src/utils/typescript.rs`: Functions for processing TypeScript files.
- `src/utils/helper.rs`: Helper functions for utils and more.
- `src/utils/logutil.rs`: Log util for debugging and logging on V10. (Not the JS/TS being executed)


#### Example
```sh
cargo run examples/test.ts
```

### Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/Thoq-jar/V10.git
   cd V10
    ```
2. Build the project:
3. ```sh
   cargo build --release
   ```
4. Add the executable generated in target/release to your path and reload terminal!

5. Run `V10 version` or `V10 version` to confim its installed!

**Linux/macOS only install:**
```sh
rm -rf V10
git clone https://github.com/Thoq-jar/V10.git
cd V10
cargo build --release
sudo mv target/release/V12 /usr/local/bin/v10
cd ..
rm -rf V10
```

#### Contributing
1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.

#### License
This project is licensed under the MIT License. See the `LICENSE` file for details.
