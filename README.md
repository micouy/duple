# `duple`

proof of concept library that adds methods to remove tuple fields that are generic over tuple length • this allows to keep the code length *`O(n)`*, where *`n`* is the tuple length, without using macros, instead of *`O(n^2)`* (that would be the case if each method had to be implemented for each tuple length) • it should be possible to also add popping from and pushing onto both ends of the tuple as well as removing nth element from the end • operations on the right end of the tuple requires wrapping the tuples from the left, i.e. `((((), a), b), c)`

all of the methods follow the same pattern: they take a type implementing `TupleWrap` with the `Wrapped` associated type of desired 'depth' • calling `TupleWrap::wrap` on a 'flat' tuple returns a corresponding nested tuple, i.e. `(a, b, c)` is turned into `(a, (b, (c, ())))` • then, a function generic over the 'tail' of the nested tuple removes the nth level • a nested tuple with the nth level removed is then unwrapped into a corresponding 'flat' tuple

```rust
use duple::prelude::*;

// Special case — returns the last element, not a tuple!
assert_eq!(('a', 'b').rem0(), 'b');
assert_eq!(('a', 'b').rem1(), 'a');

assert_eq!(('a', 'b', 'c').rem0(), ('b', 'c'));
assert_eq!(('a', 'b', 'c').rem1(), ('a', 'c'));
assert_eq!(('a', 'b', 'c').rem2(), ('a', 'b'));
```
