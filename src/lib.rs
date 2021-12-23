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

impl<A> TupleUnwrap for W<A, ()> {
    type Unwrapped = A;

    fn unwrap(self) -> Self::Unwrapped {
        let W(a, ()) = self;

        a
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

    fn unwrap(self) -> Self::Unwrapped {
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

    fn unwrap(self) -> Self::Unwrapped {
        let W(a, W(b, W(c, ()))) = self;

        (a, b, c)
    }
}

impl<A, B, C, D> TupleWrap for (A, B, C, D) {
    type Wrapped = W<A, W<B, W<C, W<D, ()>>>>;

    fn wrap(self) -> Self::Wrapped {
        let (a, b, c, d) = self;

        W(a, W(b, W(c, W(d, ()))))
    }
}

impl<A, B, C, D> TupleUnwrap for W<A, W<B, W<C, W<D, ()>>>> {
    type Unwrapped = (A, B, C, D);

    fn unwrap(self) -> Self::Unwrapped {
        let W(a, W(b, W(c, W(d, ())))) = self;

        (a, b, c, d)
    }
}

impl<A, B, C, D, E> TupleWrap for (A, B, C, D, E) {
    type Wrapped = W<A, W<B, W<C, W<D, W<E, ()>>>>>;

    fn wrap(self) -> Self::Wrapped {
        let (a, b, c, d, e) = self;

        W(a, W(b, W(c, W(d, W(e, ())))))
    }
}

impl<A, B, C, D, E> TupleUnwrap for W<A, W<B, W<C, W<D, W<E, ()>>>>> {
    type Unwrapped = (A, B, C, D, E);

    fn unwrap(self) -> Self::Unwrapped {
        let W(a, W(b, W(c, W(d, W(e, ()))))) = self;

        (a, b, c, d, e)
    }
}

impl<A, B, C, D, E, F> TupleWrap for (A, B, C, D, E, F) {
    type Wrapped = W<A, W<B, W<C, W<D, W<E, W<F, ()>>>>>>;

    fn wrap(self) -> Self::Wrapped {
        let (a, b, c, d, e, f) = self;

        W(a, W(b, W(c, W(d, W(e, W(f, ()))))))
    }
}

impl<A, B, C, D, E, F> TupleUnwrap for W<A, W<B, W<C, W<D, W<E, W<F, ()>>>>>> {
    type Unwrapped = (A, B, C, D, E, F);

    fn unwrap(self) -> Self::Unwrapped {
        let W(a, W(b, W(c, W(d, W(e, W(f, ())))))) = self;

        (a, b, c, d, e, f)
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

impl<A, B, C, R> W<A, W<B, W<C, R>>> {
    fn rem3(self) -> W<A, W<B, R>> {
        let W(a, W(b, W(_c, rest))) = self;

        W(a, W(b, rest))
    }
}

impl<A, B, C, D, R> W<A, W<B, W<C, W<D, R>>>> {
    fn rem4(self) -> W<A, W<B, W<C, R>>> {
        let W(a, W(b, W(c, W(_d, rest)))) = self;

        W(a, W(b, W(c, rest)))
    }
}

impl<A, B, C, D, E, R> W<A, W<B, W<C, W<D, W<E, R>>>>> {
    fn rem5(self) -> W<A, W<B, W<C, W<D, R>>>> {
        let W(a, W(b, W(c, W(d, W(_e, rest))))) = self;

        W(a, W(b, W(c, W(d, rest))))
    }
}

impl<A, B, C, D, E, F, R> W<A, W<B, W<C, W<D, W<E, W<F, R>>>>>> {
    fn rem6(self) -> W<A, W<B, W<C, W<D, W<E, R>>>>> {
        let W(a, W(b, W(c, W(d, W(e, W(_f, rest)))))) = self;

        W(a, W(b, W(c, W(d, W(e, rest)))))
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

pub trait Remove3<A, B, C, R>:
    TupleWrap<Wrapped = W<A, W<B, W<C, R>>>> + Sized
where
    W<A, W<B, R>>: TupleUnwrap,
{
    fn rem3(self) -> <W<A, W<B, R>> as TupleUnwrap>::Unwrapped {
        self.wrap().rem3().unwrap()
    }
}

impl<T, A, B, C, R> Remove3<A, B, C, R> for T
where
    T: TupleWrap<Wrapped = W<A, W<B, W<C, R>>>> + Sized,
    W<A, W<B, R>>: TupleUnwrap,
{
}

pub trait Remove4<A, B, C, D, R>:
    TupleWrap<Wrapped = W<A, W<B, W<C, W<D, R>>>>> + Sized
where
    W<A, W<B, W<C, R>>>: TupleUnwrap,
{
    fn rem4(self) -> <W<A, W<B, W<C, R>>> as TupleUnwrap>::Unwrapped {
        self.wrap().rem4().unwrap()
    }
}

impl<T, A, B, C, D, R> Remove4<A, B, C, D, R> for T
where
    T: TupleWrap<Wrapped = W<A, W<B, W<C, W<D, R>>>>> + Sized,
    W<A, W<B, W<C, R>>>: TupleUnwrap,
{
}

pub trait Remove5<A, B, C, D, E, R>:
    TupleWrap<Wrapped = W<A, W<B, W<C, W<D, W<E, R>>>>>> + Sized
where
    W<A, W<B, W<C, W<D, R>>>>: TupleUnwrap,
{
    fn rem5(self) -> <W<A, W<B, W<C, W<D, R>>>> as TupleUnwrap>::Unwrapped {
        self.wrap().rem5().unwrap()
    }
}

impl<T, A, B, C, D, E, R> Remove5<A, B, C, D, E, R> for T
where
    T: TupleWrap<Wrapped = W<A, W<B, W<C, W<D, W<E, R>>>>>> + Sized,
    W<A, W<B, W<C, W<D, R>>>>: TupleUnwrap,
{
}

pub trait Remove6<A, B, C, D, E, F, R>:
    TupleWrap<Wrapped = W<A, W<B, W<C, W<D, W<E, W<F, R>>>>>>> + Sized
where
    W<A, W<B, W<C, W<D, W<E, R>>>>>: TupleUnwrap,
{
    fn rem6(
        self,
    ) -> <W<A, W<B, W<C, W<D, W<E, R>>>>> as TupleUnwrap>::Unwrapped {
        self.wrap().rem6().unwrap()
    }
}

impl<T, A, B, C, D, E, F, R> Remove6<A, B, C, D, E, F, R> for T
where
    T: TupleWrap<Wrapped = W<A, W<B, W<C, W<D, W<E, W<F, R>>>>>>> + Sized,
    W<A, W<B, W<C, W<D, W<E, R>>>>>: TupleUnwrap,
{
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
		// Special case — returns an element, not a tuple!
        assert_eq!(('a', 'b').rem1(), 'b');
        assert_eq!(('a', 'b').rem2(), 'a');

        assert_eq!(('a', 'b', 'c').rem1(), ('b', 'c'));
        assert_eq!(('a', 'b', 'c').rem2(), ('a', 'c'));
        assert_eq!(('a', 'b', 'c').rem3(), ('a', 'b'));

        assert_eq!(('a', 'b', 'c', 'd').rem1(), ('b', 'c', 'd'));
        assert_eq!(('a', 'b', 'c', 'd').rem2(), ('a', 'c', 'd'));
        assert_eq!(('a', 'b', 'c', 'd').rem3(), ('a', 'b', 'd'));
        assert_eq!(('a', 'b', 'c', 'd').rem4(), ('a', 'b', 'c'));

        assert_eq!(('a', 'b', 'c', 'd', 'e').rem1(), ('b', 'c', 'd', 'e'));
        assert_eq!(('a', 'b', 'c', 'd', 'e').rem2(), ('a', 'c', 'd', 'e'));
        assert_eq!(('a', 'b', 'c', 'd', 'e').rem3(), ('a', 'b', 'd', 'e'));
        assert_eq!(('a', 'b', 'c', 'd', 'e').rem4(), ('a', 'b', 'c', 'e'));
        assert_eq!(('a', 'b', 'c', 'd', 'e').rem5(), ('a', 'b', 'c', 'd'));

        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem1(),
            ('b', 'c', 'd', 'e', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem2(),
            ('a', 'c', 'd', 'e', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem3(),
            ('a', 'b', 'd', 'e', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem4(),
            ('a', 'b', 'c', 'e', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem5(),
            ('a', 'b', 'c', 'd', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem6(),
            ('a', 'b', 'c', 'd', 'e')
        );
    }
}
