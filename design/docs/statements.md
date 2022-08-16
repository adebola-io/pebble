<h1 align=center>Statements in Pebble</h1>
The four types of statements in Pebble are:

1. [Declarative Statements](#declarative-statements)
2. [Conditional Statements](#conditional-statements)
3. [Expressive Statements](#expressive-statements)
4. [Iterative Statements](#iterative-statements)

## Declarative Statements

Declarative statements are statements that provide a new variable, type or function into the scope. A key feature of declarative statements in Pebble is that they start with an `injunction`, i.e. a special word beginning with an `@` symbol. As with [expressive statements](#expressive-statements), all declarative statements end with a semicolon.

There are six types of declarative statements in Pebble.

### Variable Declarations

In Pebble, declaration of a new, changeable variable is done using the `@let` injunction. For example:

```pebble
@let foo: String = "bar";
```

This tells the compiler that throughout the scope, the word `foo` is a substitute for the string "bar". Variables can be followed by their _type labels_, which specify what [type](./variables_and_types.md) the variable is. In the example above, the `: String` after `@let foo` tells the compiler that variable `foo` is a string. Type labels are considered good practice and can help a lot while debugging. However, they are not always necessary.

```pebble
@let foo = "bar"; ✔️
```

In the above snippet, the compiler can automatically infer that the `foo` is a string, and it will be treated as a string throughout its lifetime.

Variables can also be declared without an initial value. When variables do not have a value, they are known as `uninitialized variables.` The most important things to note about unintialized variables are:

1. _Their type labels are mandatory_, like so:

```pebble
@let foo; ❌
```

```pebble
@let foo: String; ✔️
```

1. _They cannot be used until they are given a value_, like so:

```pebble
@let foo: String;
println foo; ❌
```

```pebble
@let foo: String;
foo = "bar";
println foo; ✔️
```

For more on types and variable in Pebble, go [here.](./variables_and_types.md)

### Constant Declarations

A constant declaration starts with the `@const` injunction, as opposed to `@let`. Constants behave the same way as variables, but with a few key differences:

1. _Unlike variables, constants must be assigned values when they are declared._

```pebble
@const API_URL: String; ❌
```

```pebble
@const API_URL: String = "https://api.example.com"; ✔️
```

1. _The values of a constant, once set, cannot be changed._

```pebble
@const API_URL: String = "https://api.example.com";
API_URL = "http://newaddress.com"; ❌
```

```pebble
@const API_URL: String = "https://api.example.com";
@const API_URL_2: String = "http://newaddress.com"; ✔️
```

1. _The type labels of constants are mandatory._

```pebble
@const API_URL = "https://api.example.com"; ❌
```

```pebble
@const API_URL: String = "https://api.example.com"; ✔️
```

_`TIP`_ - The convention is to use `SCREAMING_SNAKE_CASE` for constant names, and regular `snake_case` for variable names.

### Function Declarations

A function is an enclosed block of code that performs an action and return a value. Functions are the building blocks of Pebble and many other programming languages. They are defined using the `@function` injunction, like so:

```pebble
## Adds two numbers together.
@function add(m: Number, n: Number) -> Number {
    @let sum = m + n;
    return sum;
}

println add(2, 2); // Will print out 4.
```

```pebble
## Prints a greeting.
@function greet() {
    println "Hello, world!";
}

greet(); // Will print out "Hello, world!".
```

A few things can be picked from the two functions in the code sample above:

1. _A function can have any number of parameters_. Parameters are separated with a comma and any number of spaces.

2. _All the parameters of a function must have type labels._ This tells the Pebble compiler how to treat the value within the code block.

3. _If a function returns a value, the type of that value must be specified._ The `add` function returns `sum`, which is a value that is derived from the addition of numbers m and n. Since the addition of two numbers will also result in a number, when sum is returned, its type must match the return type specified for the function.

4. _The return type **can** be omitted from a function_, as we see in `greet`. When a function has no return type, the compiler infers that the function has no return value. i.e. It returns [`nil`](./nil.md).

5. _Functions are called with parenthesis._ To call a function, all you have to do is write out its name followed by the arguments passed in parenthesis.

```pebble
function_name(parameter);
```

Other things that may not be noticed from the examples but are equally important, are:

1. _The parameters of a function are treated as [constants.](#constant-declarations)_ When parameters are passed into a function, they cannot be changed within the scope of that function. Therefore, assigning new values to a function argument is not allowed. If those values must be changed, consider saving the mutation into another variable.

```pebble
    ## Subtracts from a number.
    @function subtract(x: Number, y: Number) -> Number {
        x = x - y; ❌
        return x;
    }
```

```pebble
    ## Subtracts from a number.
    @function subtract(x: Number, y: Number) -> Number {
        @let z = x - y; ✔️
        return z;
    }
```

1. _Outer variables cannot be accessed from within a function_. A variable that is defined outside of a function cannot be used from within the its scopes. Therefore, the following code sample will fail to compile. **However**, outer constants can be referenced, because they are immutable and the function cannot change their values.

```pebble
@let name = "Sefunmi";

@function getName() {
    println name; ❌
}
```

```pebble
@const name = "Sefunmi";

@function getName() {
    println name; ✔️
}
```

### Model Declarations

Models are
