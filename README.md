# convert
A Simple command line executable for converting between common numbering systems.

## Inputs
* -i, -in: Numbering system of input data
* -o, -out: Numbering system to be converted to
* -f, -file: A File to read for data

## Numbering systems
* ascii
* bin, binary
* oct, octal
* dec, decimal
* hex, hexadecimal

## Examples
* convert -i hex -o ascii "48 65 6c 6c 6f"
* convert "Hello" -i ascii -o hex
* convert -f text.txt -o hex
