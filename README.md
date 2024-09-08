# Arp language

This repository contains the compiler for the Arp programming language. Arp is designed to offer a safer and more accessible approach to writing code, which is ultimately translated to CIL, making it inherently compatible with the .NET ecosystem, particularly with C#.

> *Disclaimer:* This project was initially conceived as part of my personal compiler research. Having undergone three complete rewrites, I ultimately decided to finalize this iteration. Due to personal circumstances, I completed it under time constraints, which has resulted in varying code quality across different modules. For instance, the code in _arp_parser_ is more polished than _arp_ast_processor_, and _arp_ast_processor_ is more refined than _arp_compiler_. Even this documentation has been prepared at the last minute. 
> Additionally, as you might expect, the compiler is currently quite unstable. I hope to revisit this project in the future to improve both its stability and quality.

Unfortunately, almost all of the language design ideas I originally envisioned were not fully implemented due to time limitations and complexity. For further insights into the issues with C# that inspired this project, you can read my thoughts in the [Programming Manifest](./Documentation/ProgrammingManifest.md).

## Running the Arp Compiler

> Please note that, due to its current state of instability, using the compiler may require a lot of troubleshooting. For example external imports won't work on your machine due to hardcoded paths.

If you wish to experiment with the Arp compiler, you will need the following:

- .NET 8.0 installed.
- Compilation of _arp_compiler_ from source.
- A project structure similar to this [example](./examples/hello_world/).
- Run the compiler with the command: `arpc run`.

For those interested in exploring more of the language’s features, please refer to the [Language Reference](./Documentation/Reference.md).