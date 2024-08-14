### V12 Engine

V12 is a Rust-based engine designed to process and execute JavaScript and TypeScript files. It uses the Boa JavaScript engine for script execution and provides a simple interface for running scripts and displaying information about the engine.

#### Features
- **TypeScript to JavaScript Conversion**: Strips TypeScript types and converts `.ts` files to `.js` files.
- **JavaScript Execution**: Executes JavaScript files using the Boa engine.
- **Temporary File Management**: Creates and deletes temporary JavaScript files generated from TypeScript files.
- **Engine Information**: Displays information about the V12 engine.

#### Prerequisites
- Rust (latest stable version)
- Cargo (latest stable version)
- 
#### Installation
1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/V12.git
   cd V12
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

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
- `src/utils/helper.rs`: Helper functions for logging.
- `src/js/V12.js`: JavaScript file containing the `about` function.

#### Example
```sh
cargo run examples/hello_world.ts
```

### Installation
1. Clone the repository:
   ```sh
   git clone
    cd V12
    ```
2. Build the project:
3. ```sh
   cargo build --release
   ```
4. Add the executable generated in target/release to your path and reload terminal!

#### Contributing
1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.

#### License
This project is licensed under the MIT License. See the `LICENSE` file for details.