# Lox-rs [![Build Status](https://travis-ci.com/jeschkies/lox-rs.svg?branch=master)](https://travis-ci.com/jeschkies/lox-rs)

A [Lox](http://craftinginterpreters.com/the-lox-language.html) Interpreter in Rust based on the
[Crafting Interpreters](http://craftinginterpreters.com) book.

Run the interpreter with `cargo run -p interpreter -- examples/class.lox`. The bytecode VM is run with `cargo run -p bytcode -- examples/class.lox`.

Each commit corresponds to one chapter in the book:

## Part II: A Tree-Walk Interpreter
  * [Chapter 4: Scanning](https://github.com/jeschkies/lox-rs/commit/9fef15e73fdf57a3e428bb074059c7e144e257f7)
  * [Chapter 5: Representing Code](https://github.com/jeschkies/lox-rs/commit/0156a95b4bf448dbff9cb4341a2339b741a163ca)
  * [Chapter 6: Parsing Expressions](https://github.com/jeschkies/lox-rs/commit/9508c9d887a88540597d314520ae6aa045256e7d)
  * [Chapter 7: Evaluating Expressions](https://github.com/jeschkies/lox-rs/commit/fd90ef985c88832c9af6f193e0614e41dd13ef28)
  * [Chapter 8: Statements and State](https://github.com/jeschkies/lox-rs/commit/941cbba900acb5876dbe6031b24ef31ff81ca99e)
  * [Chapter 9: Control Flow](https://github.com/jeschkies/lox-rs/commit/d1f8d67f65fa4d66e24e654fec7bd8d1529b124d)
  * [Chapter 10: Functions](https://github.com/jeschkies/lox-rs/commit/0e10d13944a6cd77d37f9cdf393ed81ba9573172)
  * [Chapter 11: Resolving and Binding](https://github.com/jeschkies/lox-rs/commit/bd2952230567df568d77855f730540462f350a45)
  * [Chapter 12: Classes](https://github.com/jeschkies/lox-rs/commit/337896b3dae4087ad889dca2f3cca32ed025134b)
  * [Chapter 13: Inheritance](https://github.com/jeschkies/lox-rs/commit/0207ecc8fca1af20667c69cefb4fa5f277330ca3)
  
  ## Part III: A Bytecode Virtual Machine 
  * [Chapter 14: Chunks of Bytecode](https://github.com/jeschkies/lox-rs/commit/bcec748d59b568c3b6ce93d6d07b40b14f44caa0)
  * [Chapter 15: A Virtual Machone](https://github.com/jeschkies/lox-rs/commit/5c528b63f0ea4a5cfce3757b6c0a5323cba1abf6)
