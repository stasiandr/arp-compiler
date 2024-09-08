# Reference

## Variable Declarations

In arp, variables are declared using the **let** keyword. A variable declaration includes the variable name, followed by a colon, and then the **type** of the variable. The type system is static, meaning that the **type** of a variable is known at compile-time. For example:

```rust
let integer_variable: int32 = -1;
let string_variable = "Hello";
```

Here, `integer_variable` is declared as an integer (int32). The variable's type must match the type of the assigned value.
The compiler will try to inherit variable type, but won't always succeed.

System types are:
- int32
- float32
- string
- bool
- *void

## Expressions

Expressions in arp are constructs that evaluate to a value. They can be as simple as a literal value or as complex as a function call. Arithmetic operations, logical operations, and function calls are all expressions. Examples include:

```rust
let sum = 5 + 3;
let isEqual = (a == b);
let result = function_call();
```

Expressions can be combined to create more complex expressions, and the type of an expression must be consistent with its expected type context.

## Control Flow

Arp provides typical control flow structures such as conditional statements and loops:

### Conditional Statements

**If** statements are used to execute code blocks conditionally:

```rust
if condition {
    // code block if condition is true
} else {
    // code block if condition is false
}
```
### Loops

**While** loops execute a block of code repeatedly as long as a given condition is true:

```rust
while condition {
    // code block
}
```

## Functions

Functions in arp are defined using the **fn** keyword, followed by the function name, parameters, return type, and the function body. Functions can return a value, and the return type is specified after the **->** symbol. If function don't have a return type you can leave empty. Here is an example:

```rust
fn add(a: int32, b: int32) -> int32 {
    return a + b;
}

fn store() {
    // ...
}
```

Functions can be called by passing the required arguments:

```rust
let result = add(5, 3);
```

## Classes

In arp, classes are declared using the **class** keyword, followed by the class name and its body. The class body contains fields and methods. For example:

```rust
class Point {
    x: int32;
    y: int32;
}
```

Classes can be instantiated with respective fields. For example:

```rust
let start = Point { 
    x: 1, 
    y: 2 + 3 
};
```

Class method can be get or set using **dot** notation. For example: 

```rust
start.x = 14;
let y = start.y;
```

Class also can be passed as an argument to functions or be return type:

```rust
class PointBuilder {
    min: int32;
    max: int32;
}

fn build() -> Point {
    let random = Random { };
    let x = random.next_int(this.min, this.max);
    let y = random.next_int(this.min, this.max);
    return Point {
        x,
        y
    };
}
```

> Note that for compatibility reasons you can include `.` in a class name, like in next example. But I suppose this is a bad practice.

```rust 
class MyNameSpace.MyClass {
    min: int32;
    max: int32;
}
```

## Implementation

In arp, class methods are defined in *impl* block. for example: 

```rust
impl MyClass {
    fn test() {
        // ...
    }
}
```

If a function is declared with *this* keyword it's considered as _instance method_. If without — _static method_.

> *Note:* _this_ keyword doesn't need a type. It's introduced to later support pointers and mutability checks.  

```rust
impl MyClass {
    fn instance(this, parameter: in32) {
        // ...
    }

    fn static(parameter: in32) {
        // ...
    }
}
```


## Multi-File Setup

Arp supports a multi-file setup, allowing the separation of code across multiple files. This feature aids in better organization and modularization of code. Classes can be imported into other files to share code and functionality.

```python
from path.to.file import MyClass
```

All arp files in a project must be inside `src/` folder. Entry point must be in file called `main.arp`. Declaring `fn main() {}` in this file is optional.

If you want to load managed Dll you can achieve it by using extern keyword:

```python
from extern System.Console.dll import System.Console
```

## arpm.toml

The arpm.toml file is used to define the configuration settings for compiling a project in the arp programming language. This file is structured into three main sections: package, dotnet, and dev. Each section contains specific configuration options that influence the compilation process.

### Overview of arpm.toml Structure

The arpm.toml file is composed of the following sections:

- [package]: General project information.
- [dotnet]: .NET-specific configuration settings.
- [dev]: Development settings.

Example arpm.toml:

```toml
[package]
name = "example_project"

[dotnet]
target = "net8.0"

[dev]
clean_build = false
```

### Detailed Breakdown
#### 1. [package] Section
This section provides general information about the package, such as its name.

name (String):
The name of the package. If not specified, it defaults to "unknown".
Example:

```toml
[package]
name = "example_project"
``` 
#### 2. [dotnet] Section
The [dotnet] section contains settings specific to .NET, which are crucial for the compilation process when targeting .NET platforms.

- sdk (String): Specifies the .NET SDK to be used. The default value is "Microsoft.Net.Sdk.il/8.0.0".
- target (String): Defines the target framework. The default is "net8.0".
- output_exe (Boolean): Determines whether the output should be an executable (true) or a library (false). The default is true, meaning an executable will be generated.
- self_contained (Boolean): Specifies whether the application should be self-contained, including all necessary dependencies within the output. The default is true.

Example:

```toml
[dotnet]
sdk = "Microsoft.Net.Sdk.il/8.0.0"
target = "net8.0"
output_exe = true
self_contained = true
```

#### 3. [dev] Section
The [dev] section includes settings that are useful during the development process.

- clean_build (Boolean): Indicates whether a clean build should be performed, which deletes all previous build artifacts before compiling. The default is false.

Example:

```toml
[dev]
clean_build = false
```

#### Default Values
If any of the settings in the arpm.toml file are omitted, the following default values are used:

```toml
[package]
name = "unknown"

[dotnet]
sdk = "Microsoft.Net.Sdk.il/8.0.0"
target = "net8.0"
output_exe = true
self_contained = true

[dev]
clean_build = false
```

## Standard Library

Arp currently comes without a standard library. However, you can easily import external DLLs, such as System.Console, to provide the necessary functionality. This approach works quite well in many cases and allows for compatibility with the .NET ecosystem, enabling you to leverage existing libraries until a more comprehensive standard library is developed for Arp.

## Language Server

Before introducing some breaking changes, the language server was capable of highlighting syntactic errors during the lexer and parser stages. Unfortunately, this feature is currently non-functional. However, I hope to eventually return to the LSP (Language Server Protocol) implementation and restore this useful functionality.