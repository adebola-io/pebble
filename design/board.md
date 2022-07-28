1. All parameters are immutable by default.
2. All function parameters require type annotations.
3. All outer variables and parameters in a struct are read-only, even if they are destructured.
4. Outer variables cannot be accessed from functions. However, other functions, constants, records, structs, enums and interfaces can be accessed.
5. Implementations are done on the struct body.
6. Interfaces can implement other interfaces.
7. Circular and Recursive interface implementations are not possible.
8. Re-implementing an interface on a sub-struct should trigger a warning.
9. The function return type must be specified using the -> operator on the function signature. Else, it default to nil.
10. Ranges can start at negative numbers and move to positive ones.
11. Numbers can either be Hexadecimal, decimal, octal or binary.
12. There is only one self pointer.
13. A record cannot consist of runtime values. All keys and values must be known at compile time.
