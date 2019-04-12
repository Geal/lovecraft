# You've met with a terrible fate, haven't you?

Long ago, every Rust program came with a set of Lovecraft quotes that would
be displayed on panic. Unfortunately, for [a lot of important reasons](https://github.com/rust-lang/rust/issues/13871)
like making "hello world" programs smaller, they were removed.

Thankfully, this crate brings disorder and dread back into your programs.

With the following code:

```rust
extern crate lovecraft;

fn main() {
    lovecraft::invoke();

    panic!("Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn");
}
```

This will print:

```
$ ./main
Ph'nglui mglw'nafh Cthulhu R'lyeh wgah'nagl fhtagn

It was from the artists and poets that the pertinent answers came, and I
know that panic would have broken loose had they been able to compare notes.
As it was, lacking their original letters, I half suspected the compiler of
having asked leading questions, or of having edited the correspondence in
corroboration of what he had latently resolved to see.
```

You can also choose your own quotes with the function: `lovecraft::panic_quotes(quotes: &'static [&'static str], default: &'static str)`

