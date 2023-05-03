# Test Data Generator

**Test Data Generator** is a helper for generating test data for coding problems

![build](https://img.shields.io/github/actions/workflow/status/revival0728/test-data-generator/rust.yml?branch=master)
![language](https://img.shields.io/github/languages/top/revival0728/test-data-generator)
![codesize](https://img.shields.io/github/languages/code-size/revival0728/test-data-generator)
![downcount](https://img.shields.io/github/downloads/revival0728/test-data-generator/total)
![issue](https://img.shields.io/github/issues/revival0728/test-data-generator)
![license](https://img.shields.io/github/license/revival0728/test-data-generator)
![version](https://img.shields.io/github/v/release/revival0728/test-data-generator)
![upstatus](https://img.shields.io/github/last-commit/revival0728/test-data-generator)
---

## Features

- **Easy to use**: use simple custom script to generate test data
- **Save time**: no longer to write source for test data in other complex language
- **Fast**: compile the script to TD assambly (test-data assembly) which helps generating multiple file
- **Convenient**: create answer file and input file at the same time


## Installation

### Download Binary
There is no need to install anything on your computer. Just download the binary file from the release

### Build from Source
```zsh
git clone https://github.com/revival0728/test-data-generator.git
cd test-data-generator
cargo build --release
```

## Usage

### Compile Script
```
tdg -c [File Path]
```

### Run Compiled Script
```
tdg -e [Compiled File Path] [-n [Generate File Count:default=1]] 
[--filename-format [Format:default=origin_file_name]] 
[--create-answer [Answer Execution Command:default=""]] 
[--id-base [Generate ID Starting Number:default=0]]
[--output-dir [Output Directory for Test Data:default="."]]
```

### Argument Notice
- `-c`: `[File Path]` should end with `.tds`
- `-e`: `[Compiled File Path]` should end with `.tdc`
- `-n`: to set this argument, you also have to set argument `--filename-format`
- `--filename-format`: needs to contain `*` in parameter. represents the length of file ID  e.g. "test_\*\*"


## Script Grammer

### Variable
Each "random statement" in the script is `Variable`

A `Variable` can be define as `( Material ; Type ; Quantity ; End Char )`

A `Material` can be any character or `macro` provided

A `Type` can be `int`, `float`, `string` and `=`, which `=` means `auto` in other language

A `Quantity` can be a positive integer, which means the quantity of test data that this `Variable` will generate

A `End Char` can be any character, which will add between each generated test data in this `Variable`

`Material`, `Type` and `Quantity` are necessary attributes in one `Variable`

Notice that `Material` and `End Char` cannot be " ", "\\", "(", "), ";", "\n" if you want to add these charater please use the `macro` provided

### Material
`BEGIN` and `END` are special `Material` which doesn't belong to any type and cannot add to the `Variable` that contains other `Material`

`BEGIN` doesn't required `Type` and `END` doesn't required `Type` and `Quantity`

The program will keep repeating between `BEGIN` and `END` `Quantity`(in `BEGIN`) times

All the other `Variable` must between at least one `BEGIN` and `END`

A `int` `Material` can be define as `[L]..[R]`([L, R])

A `float` `Material ` can be define as `[L]..[R]:.[P].f`([L, R] with precision [P])

If you want to remove the `Material` except for adding it, add "\\" before the `Material`

### Macro
`macro` can be put in `Material`, below is the list of all `macro`

- `UPC`: upper cases
- `LOC`: lower cases
- `SML`: symbols
- `ALC`: all characters (`UPC` + `LOC` + `SML`)
- `SPACE`: " "
- `BSL`: "\\"
- `LSB`: "("
- `RSB`: ")"
- `SEMI`: ";"

### Example
Each example file is in `test_file` folder

### test1.tds
script
```
(BEGIN;; 1)(1..10 ; int ; 10)(END)
```
generate
```
25364241089
```

### test2.tds
script
```
(BEGIN;; 3)

abc
 
(1..10 3..19; = =; 3) (UPC SPACE LOC; = = =; 10)
(END)
```
generate
```


abc
 
6714 VUJvsu Esl


abc
 
1958 GAtHjXlRqt


abc
 
745 vLYUkbAZdm

```

### test3.tds
script
```
(BEGIN;; 1)(ALC \abcdefg ; = string ; 8 ; *)
(1..100 \25..75 ; = int ; 11; SPACE)
(1..100:.4f \25..75:.4f ; = float ; 15; SPACE)(END)
```
generate
```
]*N*A*W*x*N*T*h
80 80 16 7 3 98 6 88 20 82 90
2.6160 99.2313 97.4529 2.0649 14.9469 12.8090 18.9847 99.3761 95.6014 87.7863 80.9590 9.0313 96.2728 5.1795 14.8349
```
