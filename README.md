# Unit Converter CLI Tool

## Description

This is a simple CLI tool that converts units of length, weight, and volume. Written in Rust.

### Supported units:

- Length: meters, feet, inches, yards, miles, kilometers, centimeters, millimeters, decimeters, dekameters, nautical miles
- Weight: grams, kilograms, ounces, pounds, and more ...
- Volume: milliliters, liters, cups, pints, quarts, gallons, and more ...
- Temperature: celsius, fahrenheit, kelvin

## Usage

### Installation

1. Download the latest release from the [releases page](123).
2. Unzip the file.
3. open a terminal and navigate to the directory where you unzipped the file.
4. Run the following command: `./unit-converter`

### Examples

#### Convert 1 meter to feet

```bash
./unit-converter meter 1 feet

1 meter is 3.28084 feet
```

#### Convert 30 Celsius to Kelvin and

```bash
./unit-converter celsius 30 kelvin
```

#### Output

```bash
30 celsius is 303.15 kelvin
```

#### Convert 30 Celsius to Fahrenheit

```bash
./unit-converter celsius 30 fahrenheit
```

#### Output

```bash
30 celsius is 86 fahrenheit
```

### Help

```bash
./unit-converter --help
```

#### or

```bash
./unit-converter -h
```

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Author

[Chloe Pratas](https://github.com/Chloe199719?)
