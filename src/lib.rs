#[derive(Debug, Copy, Clone)]
pub struct W<F, R>(F, R);

pub trait TupleWrap {
    type Wrapped;

    fn wrap(self) -> Self::Wrapped;
}

pub trait TupleUnwrap {
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
    fn rem1(self) -> W<B, R> {
        let W(_a, rest) = self;

        rest
    }
}

impl<A, B, R> W<A, W<B, R>> {
    fn rem2(self) -> W<A, R> {
        let W(a, W(_b, rest)) = self;

        W(a, rest)
    }
}

pub trait Remove1<A, B, R>: TupleWrap<Wrapped = W<A, W<B, R>>> + Sized
where
    W<B, R>: TupleUnwrap,
{
    fn rem1(self) -> <W<B, R> as TupleUnwrap>::Unwrapped {
        self.wrap().rem1().unwrap()
    }
}

impl<T, A, B, R> Remove1<A, B, R> for T
where
    T: TupleWrap<Wrapped = W<A, W<B, R>>> + Sized,
    W<B, R>: TupleUnwrap,
{
}

pub trait Remove2<A, B, R>: TupleWrap<Wrapped = W<A, W<B, R>>> + Sized
where
    W<A, R>: TupleUnwrap,
{
    fn rem2(self) -> <W<A, R> as TupleUnwrap>::Unwrapped {
        self.wrap().rem2().unwrap()
    }
}

impl<T, A, B, R> Remove2<A, B, R> for T
where
    T: TupleWrap<Wrapped = W<A, W<B, R>>> + Sized,
    W<A, R>: TupleUnwrap,
{
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        dbg!(('a', 'b').rem1());
        dbg!(('a', 'b', 'c').rem1());

        dbg!(('a', 'b').rem2());
        dbg!(('a', 'b', 'c').rem2());
    }
}
