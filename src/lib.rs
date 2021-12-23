#[derive(Debug, Copy, Clone)]
struct W<F, R>(F, R);

trait TupleWrap {
    type Wrapped;

    fn wrap(self) -> Self::Wrapped;
}

trait TupleUnwrap {
    type Unwrapped;

    fn unwrap(self) -> Self::Unwrapped;
}

impl<A> TupleWrap for (A,) {
    type Wrapped = W<A, ()>;

    fn wrap(self) -> Self::Wrapped {
        let (a,) = self;

        W(a, ())
    }
}

impl<A> TupleUnwrap for W<A, ()> {
    type Unwrapped = (A,);

    fn unwrap(self) -> (A,) {
        let W(a, ()) = self;

        (a,)
    }
}

impl<A, B> TupleWrap for (A, B) {
    type Wrapped = W<A, W<B, ()>>;

    fn wrap(self) -> Self::Wrapped {
        let (a, b) = self;

        W(a, W(b, ()))
    }
}

impl<A, B> TupleUnwrap for W<A, W<B, ()>> {
    type Unwrapped = (A, B);

    fn unwrap(self) -> (A, B) {
        let W(a, W(b, ())) = self;

        (a, b)
    }
}

impl<A, B, C> TupleWrap for (A, B, C) {
    type Wrapped = W<A, W<B, W<C, ()>>>;

    fn wrap(self) -> Self::Wrapped {
        let (a, b, c) = self;

        W(a, W(b, W(c, ())))
    }
}

impl<A, B, C> TupleUnwrap for W<A, W<B, W<C, ()>>> {
    type Unwrapped = (A, B, C);

    fn unwrap(self) -> (A, B, C) {
        let W(a, W(b, W(c, ()))) = self;

        (a, b, c)
    }
}

impl<A, B, R> W<A, W<B, R>> {
    fn drop1(self) -> W<B, R> {
        let W(_a, rest) = self;

        rest
    }
}

impl<A, B, R> W<A, W<B, R>> {
    fn drop2(self) -> W<A, R> {
        let W(a, W(_b, rest)) = self;

        W(a, rest)
    }
}

fn drop1<T, A, B, R>(tuple: T) -> <W<B, R> as TupleUnwrap>::Unwrapped
where
    T: TupleWrap<Wrapped = W<A, W<B, R>>>,
    W<B, R>: TupleUnwrap,
{
    tuple.wrap().drop1().unwrap()
}

fn drop2<T, A, B, R>(tuple: T) -> <W<A, R> as TupleUnwrap>::Unwrapped
where
    T: TupleWrap<Wrapped = W<A, W<B, R>>>,
    W<A, R>: TupleUnwrap,
{
    tuple.wrap().drop2().unwrap()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
    fn test() {
        dbg!(drop1(('a', 'b')));
        dbg!(drop1(('a', 'b', 'c')));

        dbg!(drop2(('a', 'b')));
        dbg!(drop2(('a', 'b', 'c')));
    }
}
