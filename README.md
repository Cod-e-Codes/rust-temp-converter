# Rust Temperature Converter

A simple command-line application written in Rust to convert temperatures between Celsius and Fahrenheit. This beginner-friendly project demonstrates basic input/output operations and temperature conversion formulas.

## Features
- **Celsius to Fahrenheit**: Converts a temperature value from Celsius to Fahrenheit.
- **Fahrenheit to Celsius**: Converts a temperature value from Fahrenheit to Celsius.
- **Quit Option**: Allows the user to exit the program gracefully.

## Requirements
- [Rust](https://www.rust-lang.org/) (version 1.65.0 or later)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Cod-e-Codes/rust-temp-converter.git
   cd rust-temp-converter
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## Usage
When you run the application, you will be presented with a menu:

```
Temperature Converter
Choose an option:
1. Celsius to Fahrenheit
2. Fahrenheit to Celsius
3. Quit
```

### Convert Celsius to Fahrenheit
1. Select `1` from the menu.
2. Enter the temperature in Celsius.
3. The program will display the equivalent temperature in Fahrenheit.

### Convert Fahrenheit to Celsius
1. Select `2` from the menu.
2. Enter the temperature in Fahrenheit.
3. The program will display the equivalent temperature in Celsius.

### Quit
1. Select `3` from the menu to exit the application.

## Example
```
Temperature Converter
Choose an option:
1. Celsius to Fahrenheit
2. Fahrenheit to Celsius
3. Quit
Choose an option: 1
Enter temperature in Celsius: 25
25°C is equal to 77.00°F

Choose an option: 2
Enter temperature in Fahrenheit: 77
77°F is equal to 25.00°C

Choose an option: 3
Exiting...
```

## File Structure
- **`main.rs`**: The main program logic.

## How It Works
- **Conversion Formulas**:
  - Celsius to Fahrenheit: `(C × 9/5) + 32`
  - Fahrenheit to Celsius: `(F - 32) × 5/9`
- **Error Handling**:
  - Handles invalid input and prompts the user to enter a valid number.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request.

