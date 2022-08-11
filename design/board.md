1. All parameters are immutable by default.
2. Type annotations are compulsory for function parameters, except in function expressions where they can be inferred.
3. All outer variables and parameters in a function are read-only, even if they are destructured.
4. Outer variables cannot be accessed from functions. However, other functions, constants, records, structs, enums and interfaces can be accessed.
5. Implementations are done on the struct body.
6. Variables cannot be destructured from constants.
7. Interfaces can implement other interfaces.
8. Circular and Recursive interface implementations are not possible.
9. Re-implementing an interface on a sub-struct should trigger a warning.
10.   The function return type must be specified using the -> operator on the function signature. Else, it defaults to nil.
11.   It is not possible to return values that are properties of a struct that was defined in the function.
12.   Numeric range boundaries can only be integers, and the lower limit must be lesser than the upper limit.
13.   Numbers can either be Hexadecimal, decimal, octal or binary.
14.   There is only one self pointer.
15.   A record cannot consist of runtime values. All keys and values must be known at compile time.
16.   Constant structs cannot call mutative methods from within function scopes other than the ones in which they were defined.
17.   Test blocks can only be defined at the top level.
