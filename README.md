# Cipher_analyzer

This tool can help you decipher a text where letters have been replaced by another letter.

To begin you can define the mappings that you already know in [cipher_coding.txt](input/cipher_coding.txt).

You can then run the programm to see the text with the correct mappings. This can help you to get new mappings.

The formatting in the `cipher_coding.txt` file is as followed:

```
a|b
```
Where `a` is the cipher that maps to `b`.

Example for a complete `cipher_coding.txt` file would be:
```
n|a
o|b
p|c
q|d
r|e
s|f
t|g
u|h
v|i
x|k
y|l
z|m
a|n
b|o
c|p
e|r
f|s
g|t
h|u
i|v
j|w
l|y
m|z
ä|ä
ö|ö
ü|ü
```