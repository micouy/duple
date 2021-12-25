#[derive(Debug, Copy, Clone)]
pub struct W<T>(T);

pub trait TupleWrap {
    type Wrapped;

    fn wrap(self) -> Self::Wrapped;
}

pub trait TupleUnwrap {
    type Unwrapped;

    fn unwrap(self) -> Self::Unwrapped;
}

impl<A> TupleUnwrap for (A, ()) {
    type Unwrapped = A;

    fn unwrap(self) -> Self::Unwrapped {
        let (a, ()) = self;

        a
    }
}

impl<A, B> TupleWrap for (A, B) {
    type Wrapped = (A, (B, ()));

    fn wrap(self) -> Self::Wrapped {
        let (a, b) = self;

        (a, (b, ()))
    }
}

impl<A, B> TupleUnwrap for (A, (B, ())) {
    type Unwrapped = (A, B);

    fn unwrap(self) -> Self::Unwrapped {
        let (a, (b, ())) = self;

        (a, b)
    }
}

impl<A, B, C> TupleWrap for (A, B, C) {
    type Wrapped = (A, (B, (C, ())));

    fn wrap(self) -> Self::Wrapped {
        let (a, b, c) = self;

        (a, (b, (c, ())))
    }
}

impl<A, B, C> TupleUnwrap for (A, (B, (C, ()))) {
    type Unwrapped = (A, B, C);

    fn unwrap(self) -> Self::Unwrapped {
        let (a, (b, (c, ()))) = self;

        (a, b, c)
    }
}

impl<A, B, C, D> TupleWrap for (A, B, C, D) {
    type Wrapped = (A, (B, (C, (D, ()))));

    fn wrap(self) -> Self::Wrapped {
        let (a, b, c, d) = self;

        (a, (b, (c, (d, ()))))
    }
}

impl<A, B, C, D> TupleUnwrap for (A, (B, (C, (D, ())))) {
    type Unwrapped = (A, B, C, D);

    fn unwrap(self) -> Self::Unwrapped {
        let (a, (b, (c, (d, ())))) = self;

        (a, b, c, d)
    }
}

impl<A, B, C, D, E> TupleWrap for (A, B, C, D, E) {
    type Wrapped = (A, (B, (C, (D, (E, ())))));

    fn wrap(self) -> Self::Wrapped {
        let (a, b, c, d, e) = self;

        (a, (b, (c, (d, (e, ())))))
    }
}

impl<A, B, C, D, E> TupleUnwrap for (A, (B, (C, (D, (E, ()))))) {
    type Unwrapped = (A, B, C, D, E);

    fn unwrap(self) -> Self::Unwrapped {
        let (a, (b, (c, (d, (e, ()))))) = self;

        (a, b, c, d, e)
    }
}

impl<A, B, C, D, E, F> TupleWrap for (A, B, C, D, E, F) {
    type Wrapped = (A, (B, (C, (D, (E, (F, ()))))));

    fn wrap(self) -> Self::Wrapped {
        let (a, b, c, d, e, f) = self;

        (a, (b, (c, (d, (e, (f, ()))))))
    }
}

impl<A, B, C, D, E, F> TupleUnwrap for (A, (B, (C, (D, (E, (F, ())))))) {
    type Unwrapped = (A, B, C, D, E, F);

    fn unwrap(self) -> Self::Unwrapped {
        let (a, (b, (c, (d, (e, (f, ())))))) = self;

        (a, b, c, d, e, f)
    }
}

impl<A, B, R> W<(A, (B, R))> {
    fn rem0(self) -> (B, R) {
        let (_a, rest) = self.0;

        rest
    }
}

impl<A, B, R> W<(A, (B, R))> {
    fn rem1(self) -> (A, R) {
        let (a, (_b, rest)) = self.0;

        (a, rest)
    }
}

impl<A, B, C, R> W<(A, (B, (C, R)))> {
    fn rem2(self) -> (A, (B, R)) {
        let (a, (b, (_c, rest))) = self.0;

        (a, (b, rest))
    }
}

impl<A, B, C, D, R> W<(A, (B, (C, (D, R))))> {
    fn rem3(self) -> (A, (B, (C, R))) {
        let (a, (b, (c, (_d, rest)))) = self.0;

        (a, (b, (c, rest)))
    }
}

impl<A, B, C, D, E, R> W<(A, (B, (C, (D, (E, R)))))> {
    fn rem4(self) -> (A, (B, (C, (D, R)))) {
        let (a, (b, (c, (d, (_e, rest))))) = self.0;

        (a, (b, (c, (d, rest))))
    }
}

impl<A, B, C, D, E, F, R> W<(A, (B, (C, (D, (E, (F, R))))))> {
    fn rem5(self) -> (A, (B, (C, (D, (E, R))))) {
        let (a, (b, (c, (d, (e, (_f, rest)))))) = self.0;

        (a, (b, (c, (d, (e, rest)))))
    }
}

pub trait Remove: TupleWrap + Sized {
    fn rem0<A, B, R>(self) -> <(B, R) as TupleUnwrap>::Unwrapped
    where
        Self: TupleWrap<Wrapped = (A, (B, R))> + Sized,
        (B, R): TupleUnwrap,
    {
        W(self.wrap()).rem0().unwrap()
    }

    fn rem1<A, B, R>(self) -> <(A, R) as TupleUnwrap>::Unwrapped
    where
        Self: TupleWrap<Wrapped = (A, (B, R))> + Sized,
        (A, R): TupleUnwrap,
    {
        W(self.wrap()).rem1().unwrap()
    }


    fn rem2<A, B, C, R>(self) -> <(A, (B, R)) as TupleUnwrap>::Unwrapped
    where
        Self: TupleWrap<Wrapped = (A, (B, (C, R)))> + Sized,
        (A, (B, R)): TupleUnwrap,
    {
        W(self.wrap()).rem2().unwrap()
    }

    fn rem3<A, B, C, D, R>(
        self,
    ) -> <(A, (B, (C, R))) as TupleUnwrap>::Unwrapped
    where
        Self: TupleWrap<Wrapped = (A, (B, (C, (D, R))))> + Sized,
        (A, (B, (C, R))): TupleUnwrap,
    {
        W(self.wrap()).rem3().unwrap()
    }

    fn rem4<A, B, C, D, E, R>(
        self,
    ) -> <(A, (B, (C, (D, R)))) as TupleUnwrap>::Unwrapped
    where
        Self: TupleWrap<Wrapped = (A, (B, (C, (D, (E, R)))))> + Sized,
        (A, (B, (C, (D, R)))): TupleUnwrap,
    {
        W(self.wrap()).rem4().unwrap()
    }

    fn rem5<A, B, C, D, E, F, R>(
        self,
    ) -> <(A, (B, (C, (D, (E, R))))) as TupleUnwrap>::Unwrapped
where Self:
    TupleWrap<Wrapped = (A, (B, (C, (D, (E, (F, R))))))> + Sized,
    (A, (B, (C, (D, (E, R))))): TupleUnwrap,
{
        W(self.wrap()).rem5().unwrap()
    }
}

impl<T> Remove for T where T: TupleWrap + Sized {}

#[cfg(test)]
mod test {
    use super::Remove;

    #[test]
    fn test() {
        // Special case — returns the last element, not a tuple!
        assert_eq!(('a', 'b').rem0(), 'b');
        assert_eq!(('a', 'b').rem1(), 'a');

        assert_eq!(('a', 'b', 'c').rem0(), ('b', 'c'));
        assert_eq!(('a', 'b', 'c').rem1(), ('a', 'c'));
        assert_eq!(('a', 'b', 'c').rem2(), ('a', 'b'));

        assert_eq!(('a', 'b', 'c', 'd').rem0(), ('b', 'c', 'd'));
        assert_eq!(('a', 'b', 'c', 'd').rem1(), ('a', 'c', 'd'));
        assert_eq!(('a', 'b', 'c', 'd').rem2(), ('a', 'b', 'd'));
        assert_eq!(('a', 'b', 'c', 'd').rem3(), ('a', 'b', 'c'));

        assert_eq!(('a', 'b', 'c', 'd', 'e').rem0(), ('b', 'c', 'd', 'e'));
        assert_eq!(('a', 'b', 'c', 'd', 'e').rem1(), ('a', 'c', 'd', 'e'));
        assert_eq!(('a', 'b', 'c', 'd', 'e').rem2(), ('a', 'b', 'd', 'e'));
        assert_eq!(('a', 'b', 'c', 'd', 'e').rem3(), ('a', 'b', 'c', 'e'));
        assert_eq!(('a', 'b', 'c', 'd', 'e').rem4(), ('a', 'b', 'c', 'd'));

        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem0(),
            ('b', 'c', 'd', 'e', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem1(),
            ('a', 'c', 'd', 'e', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem2(),
            ('a', 'b', 'd', 'e', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem3(),
            ('a', 'b', 'c', 'e', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem4(),
            ('a', 'b', 'c', 'd', 'f')
        );
        assert_eq!(
            ('a', 'b', 'c', 'd', 'e', 'f').rem5(),
            ('a', 'b', 'c', 'd', 'e')
        );
    }
}
