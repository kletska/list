use std::iter::Filter;

use crate::list::*;

#[derive(Clone, Debug)]
pub struct Nil;

#[derive(Clone, Debug)]
pub struct Cons<H, T>(pub H, pub T);

pub trait MassAdd<Y> {
    type Res;

    fn mass_add(self, ys: Y) -> <Self as MassAdd<Y>>::Res;
}

impl<T: Clone, L: Clone> MassAdd<PList<L>> for T {
    type Res = PList<Cons<T, L>>;

    fn mass_add(self, ys: PList<L>) -> <Self as MassAdd<PList<L>>>::Res {
        map(ys, |x| {
            Cons(self.clone(), x)
        })
    }
}

pub trait Merge<Y> {
    type Res;

    fn merge(self, ys: Y) -> <Self as Merge<Y>>::Res;
}

impl<T: Clone, L: Clone> Merge<PList<L>> for PList<T> 
where T: MassAdd<PList<L>, Res = PList<Cons<T, L>>> {
    type Res = PList<Cons<T, L>>;

    fn merge(self, ys: PList<L>) -> <Self as Merge<PList<L>>>::Res {
        fold(
            map(self, |x| {
                x.mass_add(ys.clone())
            }),
            nil(),
            |a, b| a + b
        )
    }
}

pub trait Expand {
    type Res;

    fn expand(self) -> <Self as Expand>::Res;
}

impl Expand for Nil {
    type Res = PList<Nil>;

    fn expand(self) -> <Self as Expand>::Res {
        cons(Nil, nil())
    }
}

pub trait RemovePList {
    type Res;
}

impl<T> RemovePList for PList<T> {
    type Res = T;
}

impl<H: Clone + RemovePList, T: Clone, U: Clone> Expand for Cons<H, T> 
where H: Merge<<T as Expand>::Res, Res = PList<Cons<<H as RemovePList>::Res, U>> >,
      T: Expand<Res = PList<U>> {
    type Res = <H as Merge<<T as Expand>::Res> >::Res;

    fn expand(self) -> PList<Cons<<H as RemovePList>::Res, U>> {
        let rec = self.1.expand();
        self.0.merge(rec)
    }
}


pub trait First {
    type Res;

    fn first(&self) -> <Self as First>::Res;
}

impl<H: Clone, T> First for Cons<H, T> {
    type Res = H;

    fn first(&self) -> <Self as First>::Res {
        self.0.clone()
    }
}

pub trait Pop {
    type Res;

    fn pop(self) -> <Self as Pop>::Res;
}

impl<H, T> Pop for Cons<H, T> {
    type Res = T;

    fn pop(self) -> <Self as Pop>::Res {
        self.1
    }
}

pub trait Reverse<Acc> {
    type Res;

    fn reverse(self, acc: Acc) -> <Self as Reverse<Acc>>::Res;
}

impl<T> Reverse<T> for Nil {
    type Res = T;

    fn reverse(self, acc: T) -> <Self as Reverse<T>>::Res {
        acc 
    }
}

impl<H, T, Acc> Reverse<Acc> for Cons<H, T> 
where T: Reverse<Cons<H, Acc>> {
    type Res = <T as Reverse<Cons<H, Acc>>>::Res;

    fn reverse(self, acc: Acc) -> <Self as Reverse<Acc>>::Res {
        self.1.reverse(Cons(self.0, acc)) 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let test = Cons(cons(false, cons(true, nil())), Cons(cons(10, nil()), Nil));
        let test2 = test.expand();

        print!("{:?}", test2);
    }
}


