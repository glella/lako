# Lako
Small & Easy Programming Language created with Rust based on the awesome Crafting Interpreters book https://craftinginterpreters.com/

Lako means Easy in Croatian/Bosnian

Follows closely the book to produce an Interpreted and a Compiled version as an exercise to learn about Compilers in a practical way.

Second phase will add refinements and capabilities to make it a moderately useful tiny language.

## Interpreted version

Status: Finished & tested scanner and tokenizer

###### Usage

As a repl. Just launch Lako:

```
./lako
```

With source file. ie with "test.lak":

```
./lako test.lak
```

## Bytecode compiled version

Not started yet...

## Lako Language

For now it mirrors the basic capabilities of Lox [Crafting Interpreters](https://craftinginterpreters.com/the-lox-language.html)

* High level: dynamic typing, automatic memory management.
* Data types: booleans, numbers, strings, nil (ugh).
* Expressions: arithmetic, comparison / equality, logical operators, precedence / grouping.
* Syntax, Statements & Control Flow: Follows C based languages for familiarity.
* Functions are first class.
* OOP: Classes & Inheritance.
* Minuscule Standard Library: starting off with just print & clock
* Programmed in Rust
