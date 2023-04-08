# Lako

[![Build Status](https://travis-ci.com/glella/lako.svg?branch=main)](https://travis-ci.com/glella/lako)

Small & Easy Programming Language created with Rust based on the awesome book [Crafting Interpreters](https://craftinginterpreters.com/)

Lako means Easy in Croatian/Bosnian.

Follows closely the book to produce an Interpreted and a Bytecode Compiled version as an exercise to learn about Compilers in a practical way.

## Interpreted version

Status:

* Finished & tested scanner and tokenizer.
* Finished Expressions Syntax Tree and tested AST printer.
* Finished & tested parser (chap 6).
April 2023 update:
* Resuming after a loooong period of inactivity. Forgot half of what I wrote.
* Will restart from scratch evaluating and updating every piece of code - specially the visitor pattern looking for ways to implement simmilar functionality in more idiomatic Rust form.

For now it just prints the parsed expression back handling precedence and associativity correctly.

###### Usage

As a repl. Just launch Lako. To exit Ctrl-c.

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

For now it mirrors the basic capabilities of [Lox](https://craftinginterpreters.com/the-lox-language.html)

* High level: dynamic typing, automatic memory management.
* Data types: booleans, numbers, strings, nil (ugh).
* Expressions: arithmetic, comparison / equality, logical operators, precedence / grouping.
* Syntax, Statements & Control Flow: follows C based languages for familiarity.
* Functions are first class.
* OOP: Classes & Inheritance.
* Minuscule Standard Library: starting off with just print & clock
* Created in Rust
