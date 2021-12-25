# `duple`

proof of concept library that adds methods to drop tuple fields that are generic over tuple length • this allows to keep the code length *`O(n)`* where *`n`* is the tuple length • it should be possible to also add popping from and pushing onto both ends of the tuple as well as removing nth element from the end • operations on the right end of the tuple requires wrapping the tuples from the left: `((((), a), b), c)`

```
use duple::{Remove1, Remove2, Remove3};

// Special case — removes an element, not a tuple!
assert_eq!(('a', 'b').rem1(), 'b');
assert_eq!(('a', 'b').rem2(), 'a');

assert_eq!(('a', 'b', 'c').rem1(), ('b', 'c'));
assert_eq!(('a', 'b', 'c').rem2(), ('a', 'c'));
assert_eq!(('a', 'b', 'c').rem3(), ('a', 'b'));
```
