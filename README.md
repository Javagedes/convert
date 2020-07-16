# convert
A Simple command line executable for converting between common numbering systems one byte at a time

## Inputs
* -i, -in: Numbering system of input data
* -o, -out: Numbering system to be converted to
* -f, -file: A File to read for data
* -d, -delimiter: A way to separate the values. Only used for non-ascii numbering systems

## Numbering systems
* ascii
* bin, binary
* oct, octal
* dec, decimal
* hex, hexadecimal

## Examples
* convert -i hex -o ascii "48 65 6c 6c 6f"
* convert -i hex -o ascii -d : "48:65:6c:6c:6f"
* convert "Hello" -i ascii -o hex
* convert -f text.txt -o hex
