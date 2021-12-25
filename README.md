# `duple`

proof of concept library that adds methods to remove tuple fields that are generic over tuple length • this allows to keep the code length *`O(n)`* where *`n`* is the tuple length, without using macros • it should be possible to also add popping from and pushing onto both ends of the tuple as well as removing nth element from the end • operations on the right end of the tuple requires wrapping the tuples from the left, i.e. `((((), a), b), c)`

```rust
use duple::Remove;

// Special case — returns the last element, not a tuple!
assert_eq!(('a', 'b').rem0(), 'b');
assert_eq!(('a', 'b').rem1(), 'a');

assert_eq!(('a', 'b', 'c').rem0(), ('b', 'c'));
assert_eq!(('a', 'b', 'c').rem1(), ('a', 'c'));
assert_eq!(('a', 'b', 'c').rem2(), ('a', 'b'));
```
