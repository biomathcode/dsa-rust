

Closures in rust: 
Closures are similar to annonamous function, or lambda functions

Syntax:

```rust

let add = |x, y| {println!("{}", x+y)};

add(1, 2);
add(354135, 235125132);
add(-1, 40);

```

Closures in rust will infer types at compile time

But you can't so somthing like

```rust

   let pt = |x| println!("{}", x);
   
   pt(2314);
   
   pt("hello world");
```

Why doesn't this work?

Rust does not have generic closures


## Closures and Environment
The environment for a closure can include bindings from its enclosing scope in addition to parameters and local bindings.
It borrows the binding. If we do something that would conflict with that binding, we get an error. 




### What does the `move` keyword does?

`move` closures: they give a closure its own stack frame. Without move, a closure may be tied to the stack frame that created it, while a move closure is self-contained. This means that you cannot generally return a non-move closure from a function, for example.


### Function overloading

Overloading is not supported – each function has a single implementation.
Always takes a fixed number of parameters. Default arguments are not supported. Macros can be used to support variadic functions.
Always takes a single set of parameter types. These types can be generic, which will be covered later.





#### References

https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/closures.html