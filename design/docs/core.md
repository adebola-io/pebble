<h1 align=center><code>core</code></h1>
The Pebble core library is a set of functions, interfaces and classes that are made available by default in all implementations of the Pebble programming language. The library serves as the basis of all development of software in Pebble.

## `assert`

---

### `assert.isTrue: (condition: Boolean) -> Nil`

---

### `assert.isFalse: (condition: Boolean) -> Nil`

---

### `assert.isEqual: <T implements Equatable>(item: T, item2: T) -> Nil`

---

### `assert.isNotEqual: <T implements Equatable>(condtion: Boolean) -> Nil`

---

### `assert.isGreater: <T implements Comparison>(left: T, right: T)`

---

### `assert.isLesser: <T implements Comparison>(left: T, right: T)`

---

### `assert.calls: (f: Function, times: UnsignedInt) -> Nil`

---

### `assert.AssertionError`

---

---

<br>

## `io`

---

### `io.readLine: () -> Result<String, IOError>`

---

### `io.print: (message: String) -> Result<Nil, IOError>`

---

### `io.printLine: () -> Result<Nil, IOError>`

---

### `io.printErr: (message: String) -> Result<Nil, IOError>`

---

### `io.IOError`

---

---

<br>

## `process`

---

### `process.args: ArrayList<String>`

---

### `process.current_directory: Path`

---

### `process.exit: (code: UnsignedInt) -> Nil`

Terminates the current program with an error code.

---

---

<br>

## `crypto`

---

---

<br>

## `bytes`

---

---

<br>

## `fs`

### `fs.FileBuffer`

#### `FileBuffer.toString: () -> String`

### `fs.readFile: () -> FileBuffer`

### `fs.deleteFile: (path: String) -> Result<Nil, FileSystemError>`

### `fs.copyFile: (source_path: String, dest_path: String) -> Result<Nil, FileSystemError>`

### `fs.makeDirectory: (path: String) -> Result<Nil, FileSystemError>`

---

---

<br>

## `math`

---

### `math.pow: (x: Number, y: Number) -> Number`

---

### `math.sin: (x: Number) -> Number`

---

### `math.cos: (x: Number) -> Number`

---

### `math.cube: (x: Number) -> Number`

---

### `math.square: (x: Number) -> Number`

---

### `math.sqrt: (x: Number) -> Number`

---

### `math.cbrt: (x: Number) -> Number`

---

### `math.inverse: (x: Number) -> Number`

---

### `math.psrandom: (range: Range<Number>) -> Number`

---

### `math.add: <T implements Addition>(x: T, y: T) -> T`

---

### `math.subtract: <T implements Subtraction>(x: T, y: t) -> T`

---

### `math.divide: <T implements Division>(x: T, y: T) -> T`

---

### `math.multiply: <T implements Multipication>(x: T, y: T) -> T`

---

---

<br>

## `json`

---

---

<br>

## `prelude`

All functions, interfaces and classes included by default in a Pebble program namespace.

---

### `prelude.String`

The base class for dealing with the creation, utilization and formatting of text strings.

#### `String.length: UnsignedInt`

#### `String.isEmpty: Boolean`

#### `String.clone: () -> String`

#### `String.pushString: (str: String) -> Nil`

#### `String.pushChar: (char: Char) -> Nil`

#### `String.popChar: () -> Option<Char>`

#### `String.contains: (str: String) -> Boolean`

#### `String.toLowerCase: () -> String`

#### `String.toUpperCase: () -> String`

#### `String.toCapitalCase: () -> String`

#### `String.toToggleCase: () -> String`

#### `String.toCamelCase: () -> String`

#### `String.startsWith: (str: String) -> Boolean`

#### `String.endsWith: (str: String) -> Boolean`

#### `String.trimStart: (str: String) -> String`

#### `String.trimEnd: (str: String) -> String`

#### `String.padStart: (index: UnsignedInt) -> String`

#### `String.padEnd: (index: UnsignedInt) -> String`

#### `String.chars: () -> Iterator<Char>`

#### `String.charAt: (index: UnsignedInt) -> Option<Char>`

#### `String.overwrite: (substring: String, substitute: String) -> String`

#### `String.charCodeAt: (index: UnsignedInt) -> Option<Number>`

#### `String.slice: (start: Integer, end: Integer) -> String`

#### `String.split: (separator: String) -> Iterator<String>`

---

### `prelude.Char`

The base class for character data in Pebble.

#### `Char.isWhiteSpace: () -> Boolean`

#### `Char.toString: () -> String`

#### `Char.code: Number`

#### `Char.utf8: () -> Number`

#### `Char.clone: () -> Char`

---

### `prelude.Number`

The base class for dealing with numeric values of any kind. It is a wrapper around the [`UnsignedInt`](#preludeunsignedint), [`Float`](#preludefloat) and [`Integer`](#preludeinteger) classes.

#### `Number.toString: (radix: UnsignedInt) -> Result<String, ParseError>`

#### `Number.approx: (precision: UnsignedInt) -> Number`

#### `Number.toFloat: () -> Float`

#### `Number.toUnsignedInt: () -> UnsignedInt`

#### `Number.toInteger: () -> Integer`

#### `Number.clone: () -> Number`

---

### `prelude.UnsignedInt`

The `UnsignedInt` class refers to a set of integer numbers with no signed bit. i.e. all positive whole numbers.

---

### `prelude.Float`

---

### `prelude.Integer`

---

### `prelude.Array`

---

---

<br>

## `net`

---

---

<br>

## `collections`

---

### `collections.HashMap`

---

### `collections.ArrayStack`

---

---

<br>

## `time`

---

---

<br>

## `async`

---

---

<br>

## `zlib`

---

---

<br>

## `internals`

---

---

<br>

## `dynamic`

---

### `dynamic.ALL: Any`

---

---

<br>

## `threads`

---

---

<br>

## `ffi`
