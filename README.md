# `duplo`

proof of concept library that adds methods to drop tuple fields that are generic over tuple length • this allows to keep the code length *`O(n)`* where *`n`* is the tuple length

```
use duplo::{Remove1, Remove2};

let (b,) = ('a', 'b').rem1();
let (a,) = ('a', 'b').rem2();

let (b, c) = ('a', 'b', 'c').rem1();
let (a, c) = ('a', 'b', 'c').rem2();
```
