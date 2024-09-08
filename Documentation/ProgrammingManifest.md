# Manifest

> Arp is not currently in the state described here. In fact, it doesn’t even support generics yet. What follows represents my vision for the language—ideas and aspirations that I hope to see realized one day.

I believe C# suffers from a few major design issues (often referred to as the “billion-dollar mistakes”) and several smaller ones, which impact both productivity and safety:

- Null references.
- Exception handling.
- ~~Not being written in rust.~~ Mutability.

Arp aims to address these problems by adopting concepts from modern programming languages, while remaining compatible with the .NET infrastructure. Arp compiles into managed DLLs and allows for seamless interoperability with C#. Although it is currently a toy language, the design choices behind Arp provide the foundation for it to potentially become a default choice for scenarios where C# is used today.

Let’s now delve into the key design ideas behind Arp.

## Null references.

How often have you found yourself debugging null reference exceptions? In Arp, class constructors cannot leave fields uninitialized. Furthermore, the concept of null doesn’t even exist at the lexer stage. The result? No null values, no null reference exceptions!

> *Solution:* Arp utilizes the Option monad to express optionality. While it is more verbose, it makes the presence of optional fields explicit and eliminates the unpredictability of null references.

## Exceptions.

One of the most painful aspects of C# is dealing with exceptions. In large projects, it’s difficult to ensure that your code won’t be interrupted by an unexpected exception from a third-party library or package.

> *Solution:* Arp uses the `Result` monad. If an operation might fail, it returns `Result<Res, Exception>`. This makes potential failures explicit in the type system, and consumers of your code must handle these cases explicitly, ensuring that failure handling is more predictable and visible.

## Mutability.

Another frustration in C# is the uncertainty around data mutability. Every time you pass an array, list, or any reference type to a function, you can’t be certain that the method won’t mutate the internal state of your data. While it’s possible to use interfaces like `IReadOnlyList` or pass copies of values, in practice, this is often overlooked, leaving us hoping for the best.

> *Solution:* Arp introduces the _mut_ keyword, which allows compile-time validation of variable mutability. Although it’s not a silver bullet, it’s a significant step toward safer code by preventing unintended mutations.

> *Note:* This approach also leads to improved code readability and enables certain optimizations, but that’s a topic for another time.

## Minor Issues

### Reflection

Reflection in C# is not inherently bad, but it is not powerful enough for some production scenarios. Arp offers macros, which are far more powerful, enabling capabilities such as deriving fields, generating implementations, injecting code, and much more.
 
### Standard library


C# once embraced the idea that “everything is an object,” leading to concepts like the ubiquitous `ToString()` method. However, Arp pushes this idea further by embracing more abstract and flexible concepts like `IAdd` or `IMultiply` for overriding operators such as `+` and `*`. These interfaces can be implemented for basic types like `int` or `string`. Imagine a function like `fn sum<T: IAdd>(IList<T>)` that sums any type implementing `IAdd`. Similarly, concepts like Rust’s `serde` inspire the idea of defining a standard interface for serialization and deserialization across all types in the language.

### Other

- *Union Types and Tagged Unions:* These are incredibly versatile and useful data structures that provide greater flexibility and safety in handling different cases.
- *Functions as First-Class Citizens:* In C#, it’s common to find yourself writing `static Extension` classes just to define `some_method(this ClassName var)`. Arp improves on this by allowing more intuitive and natural handling of function extensions.
