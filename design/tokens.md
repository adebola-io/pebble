# Lexer Tokens

This document contains all token types that will be scanned by the lexer, as well as the properties it contains. Each token has a `location` property indicating where the token starts and ends in the file.

- `Number` is a numeric value, e.g. 1, 10, 2.99, -94, 0x0dF, etc.
  - `Number.content` The raw string of the number.
- `String` is a sequence of characters. e.g. "Sefunmi", "model", "One does not simply walk into Mordor", etc.
  - `String.tokens` A string can have nested expressions, so the tokens property describes how the string itself is formatted.
- `Character` is a single character stored within single quotes. e.g. 'i', '\n', '\u000f', etc.
  - `Character.content` The raw value of the character.
- `Bracket` any of the following: {, } ,[ , ], (, ).
  - `Bracket.content` The raw value of the bracket.
- `Keyword` a word from the list of reserved words in Pebble. e.g. fn, for, if.
  - `Keyword.content` The raw value of the keyword.
- `Identifier` a word that does not belong to the list of keywords defined in the language.
  - `Identifier.content` The raw value of the identifier.
- `Injunction` a word starting with an @ symbol that is used for declarations in the language.
- - `Injunction.content` The raw value of the injunction.
- `Terminator` a semicolon or comma, used to end a statement, element or property declaration.
  - `Terminator.content` The raw content of the terminator.
- `Operator` a character or group of characters that performs an operation on another value or values. e. g. +, -, ++, !.
  - `Operator.content` The raw value of the operator.
