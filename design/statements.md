<h1 align=center>Statements in Pebble</h1>
The four types of statements in Pebble are:

1. Declarative Statements
2. Conditional Statements
3. Expressive Statements
4. Iterative Statements

- ## `Declarative Statements`

  Declarative statements, (otherwise known as injunctions) are statements that provide a new variable, type or function into the scope. A key feature of declarative statements in Pebbl is that they start with an `injunction`, i.e. a special keyword beginning with an `@` symbol.
  There are six types of declarative statements in Pebble.

  - ### `Variable Declarations`

    In Pebble, declaration of a new, changeable variable is done using the `@let` injunction. For example:

    ```pebble
    @let foo: String = "bar";
    ```

    This tells the compiler that throughout the scope, The word `foo` is a substitute for the string "bar".
    Variables may have type labels in their declarations, but the variable type can also be inferred automatically.

    ```pebble
    @let foo = "bar";
    ```

    The compiler can infer that the variable foo is a string, and it will be treated like a string throughout its lifetime.
    Variables can also be declared without a value. However, if there is no value to initialize the variable, then the type of the variable must be explicitly defined, like so:

    ```pebble
    @let foo: String;
    ```

  - ### `Constant Declarations`

    Unlike variable declarations, constant declarations cannot be mutated once they are defined. While variables can be declared and unintialized, constants must be initialized once they are declared, and their types must be clearly defined. e.g.

    ```pebble
    @const API_URL: String = "https://api.example.com";
    ```

    _`TIP`_ - The convention is to use `SCREAMING_SNAKE_CASE` for constant names, and regular `snake_case` for variable names.

  - ### `Function Declarations`

    Functions are bodies of code that perform an action and return a value. They are defined using the `@function` injunction, like so:

    ```pebble
    ## Adds two numbers together.
    @function addNum(m: Number, n: Number) -> Number {
      @let sum = m + n;
      return sum;
    }

    println addNum(2, 2); // Will print out 4.

    ## Prints a greeting.
    @function greet() {
      println "Hello, world!";
    }

    greet(); // Will print out "Hello, world!".
    ```

    A few things can be picked from the two functions in the code sample above:

    1. **A function can have any number of parameters**. Parameters are separated with a comma and any number of spaces.

    2. **All the parameters of a function must have type labels.** This tells the Pebble compiler how to treat the value within the code block.

    3. **If a function returns a value, the type of that value must be specified.** The `add` function returns `sum`, which is a value that is derived from the addition of numbers m and n. Since the addition of two numbers will also result in a number, when sum is returned, its type must match the return type specified for the function.

    4. **The return type _can_ be omitted from a function**, as we see in `greet`. When a function has no return type, the compiler infers that the function has no return value. i.e. It returns `nil`.

    5. **Functions are called with parenthesis.** To call a function, all you have to do is write out its name followed by the arguments passed in in parenthesis.

    Other things that may not be noticed from the example, but are equally important, are:

    1. **The arguments of a function, and other outer variables, are treated as constants.** When parameters are passed into a function, they cannot be mutated within the scope of that function. Therefore, assigning new values to a function argument is not allowed. If those values must be changed, consider saving the mutation into another variable. For example, the following code sample will fail to compile.

    ```pebble
        ## Subtracts two from a number.
        @function subtract(x: Number, y: Number) {
          x = x - y; // Brings up an error.
        }

        @let x = 9;
        subtract(x, 2);
    ```

  - ### `Struct Declarations`
    Structs are
