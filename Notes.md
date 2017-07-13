# The Hash Lookup

The_Rust_Programming_Language/raw-pointers.html

Because we're storing the cache key in the hashmap value, the hashmap
key has to be able to reference (and dereference!) the cache key and
provide the hash value.  That's what the `impl Hash` is for.

It says it right in the manual: creating a raw pointer is a safe
operation, but dereferencing one is never a safer operation since
there's no guarantee it points to a real thing in memory.

# The Equality Operator

I had to ask on Stack Overflow about this.  Apparently, if the method
signature for eq is meth.eq(&Self), then the compiler automatically
provides a referencing operator, so this is REALLY
(&*self.key).eq(&*other.key), but the first "&" is elided because the
compiler will insert it.

This is ugly and non-obvious, and I'm a little annoyed by it.  But
[Stack Overflow was very helpful](https://stackoverflow.com/questions/43218554/using-rust-dereferencing-operators-vs-with-self/43219279#43219279)

0. fn eq(&self, other: &Rhs) -> bool; // Rhs = Self, Self = K, we need two &K's
1. other.k is of type *const K
2. *other.k is of type K
3. &*other.K is of type &K
4. self.k is of type *const K
5. *self.k is of type K

And then this: "Method calls are allowed to automatically reference the
value they are called on."
http://rust-lang.github.io/book/second-edition/ch05-01-method-syntax.html#wheres-the---operator

6. K is automatically referenced to fulfill the signature, &K

The problem I have with Rust is that sometimes it's almost an ML
derived language, and sometimes it's almost C/C++, and often when I
reach for a metaphor to help me understand it's from the wrong side
of that divide.
