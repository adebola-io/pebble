<h1 align=center>Roadmap</h1>
Below is the roadmap for the development of Pebble. Done items are marked.

## Scanning / Lexical Analysis

-   [x] Lex Injunction Starts
-   [x] Lex Block comments
-   [x] Lex Line comments
-   [x] Lex Doc comments
-   [x] Lex characters
-   [x] Lex Strings
-   [x] Lex Integers
-   [x] Lex Decimal numbers
-   [x] Lex Hexadecimal numbers
-   [x] Lex Octal numbers
-   [x] Lex Binary numbers
-   [x] Lex exponential numbers
-   [x] Lex keywords
-   [x] Lex operators
-   [x] Lex brackets
-   [x] Lex punctuation
-   [x] Lex Identifiers
-   [x] Lex literals

## Parsing/ Syntactic Analysis

-   [x] Parse strings
-   [x] Parse numbers
-   [x] Parse booleans
-   [x] Parse expression statement
-   [x] Parse identifiers
-   [x] Parse binary expression
-   [x] Parse call expression
-   [x] Parse member/dot expression
-   [x] Parse namespace expression
-   [x] Parse index expression
-   [x] Parse unary expression
-   [x] Parse range expression
-   [x] Parse logical expression
-   [x] Parse ternary expression
-   [x] Parse assignment expression
-   [x] Parse type labels
-   [x] Parse generic type
-   [ ] Parse call expression with type argument
-   [x] Parse functional expression
-   [x] Parse if statement
-   [x] Parse for statement
-   [x] Parse else block
-   [x] Parse while statement
-   [x] Parse loop statement
-   [x] Parse println statement
-   [x] Parse break statement
-   [x] Parse return statement
-   [x] Parse crash statement
-   [x] Parse try statement
-   [x] Parse recover statement
-   [x] Parse empty statement
-   [x] Parse let declaration
-   [x] Parse constant declaration
-   [x] Parse record declaration
-   [x] Parse function declaration
-   [x] Parse type declaration
-   [x] Parse class declaration
-   [x] Parse class property
-   [x] Parse implement statement
-   [x] Parse interface declaration
-   [ ] Parse enum declaration
-   [x] Parse use statement
-   [x] Parse prepend statement
-   [x] Parse public declaration
-   [x] Parse test block

## Semantic Analysis

-   [ ] Stray return statements
-   [ ] Stray break statements
-   [ ] Typeless function parameters
-   [ ] Infinite loops
-   [ ] Boolean conditions
-   [ ] Unreachable code
-   [ ] Control flow Analysis
-   [ ] Data values in interface.
-   [ ] Bundling and File Hashing
-   [ ] Prepend conflict resolution.
-   [ ] Type inference
-   [ ] Constant mutability blocking
-   [ ] Meaningless public modifiers.
-   [ ] Parameter mutability blocking
-   [ ] Circular interface implementation blocking.
-   [ ] Local test blocks blocking.
-   [ ] Record runtime value blocking.
-   [ ] Stray use imports.
-   [ ] Undefined class properties blocking.
-   [ ] Nil assignment blocking.
-   [ ] Function return type resolution.
-   [ ] Identifier resolution
-   [ ] Scope checking
-   [ ] Build symbol table

## Compiler Optimizations

-   [ ] Dead code elimination
-   [ ] Constant propagation
-   [ ] Strength reduction
-   [ ] Value numbering

## Virtual Machine

-   [ ] Short circuiting

## IDE Extensions

-   [ ] Syntax Highlighting
-   [ ] Hover Information
-   [ ] Support for @prepend
-   [ ] Support for @use
-   [ ] Code Completion

## Prepiler
