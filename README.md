# Overview

An assembler developed to translate programs written in the Hack assembly language into the binary code understood by the Hack hardware platform.  The translation specification is as described in the book 'The Elements of Computing Systems' by Noam Nisan and Shimon Schocken.

# Installation

A linux binary is available under 'Releases'.

# Operation

The assembler is intended to be run from the command line and takes two required arguments.

> $ assembler <input_file> <output_file>

## 'input_file' argument

The first required argument is the path to the file containing a program written in the Hack assembly language which is intended for translation.

The file must have the extension '.asm', otherwise the process will exit with an error.

## 'output_file' argument

The second required argument is the file path where the resultant translated binary code file should be saved.  If a file already exists with this path then it will be overwritten, else it will be created.

The file path must have the extension '.hack', otherwise the process will exit with an error.

## Debugging

Optional debug logging can be activated by setting the below environment variable.

> $ RUST_LOG=debug

# Context

Building this assembler is part of the 'Nand to Tetris' project that runs throughout the previously mentioned book, 'The Elements of Computing Systems'.  It is the task given at the end of chapter 6.

The 'Nand to Tetris' project as a whole, of which this assembler is one element, seeks to build a simple computer system starting from nothing more than a Nand logic gate.

In order to simplify the design process, one of the conditions given as part of the task outline was to assume that any input file was error-free.  Therefore the assembler that has been developed includes very limited syntax validation capabilities for provided input files.  Generally you will only see syntax checking in place when its implementation fell naturally out of the chosen design path and therefore didn't incur any significant design effort in and of itself.

A number of known correct input files and output files are contained in the 'testfiles' directory of this repository and are used to implement the integration testing.

# Language

For this task I took the opportunity to learn and then use the Rust programming language.  Firstly, I was interested in exploring a lower level language.  Moreover, having read about Rust's recent growth, I was motivated to understand the benefits it potentially confers for memory safety and error handling.

# Takeaways

As this was the first program I had written from scratch in Rust, there was somewhat of a learning curve involved.  However, I quickly came to appreciate some of the features of Rust that distinguish it from its peers.  The borrow-checker was highly useful for flagging possible memory safety bugs at compile time, whilst the 'Result' and 'Option' ensured robust error and edge case handling in a way that felt natural and integral to the design process.
