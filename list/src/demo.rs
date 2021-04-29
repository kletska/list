use std::fmt::Debug;

use crate::list::*;
#[macro_use]
use crate::macros::*;

fn mass_cons<T: Clone>(x: T, ys: PList<PList<T>>) -> PList<PList<T>> {
    match &*(ys.0) {
        List::Nil => nil(),
        List::Cons(h, t) => 
            cons(cons(x.clone(), h.clone()), mass_cons(x, PList(t.clone())))
    }
}

fn merge<T: Clone>(xs: PList<T>, ys: PList<PList<T>>) -> PList<PList<T>> {
    match &*(xs.0) {
        List::Nil => nil(),
        List::Cons(h, t) => 
            mass_cons(h.clone(), ys.clone()) + merge(PList(t.clone()), ys),
    }
}

pub fn expand<T: Clone>(xs: PList<PList<T>>) -> PList<PList<T>> {
    match &*(xs.0) {
        List::Nil => cons(nil(), nil()),
        List::Cons(h, t) => merge(h.clone(), expand(PList(t.clone())))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn expand() {
        let lst = list!(1, 2, 3);
        let lst0 = list!(4, 5);

        let test = super::expand(list!(lst, lst0));

        println!("{:?}", test);
    }

    #[test]
    fn mass_cons() {
        let lst = list!(1, 2, 3);
        let lst0 = list!(4, 5);

        let test = super::mass_cons(1, list!(lst, lst0));

        println!("{:?}", test);
    }

    #[test]
    fn merge() {
        let lst = list!(1, 2, 3);
        let lst0 = list!(4, 5);
        let lst1 = list!(6, 7);

        let test = super::merge(lst1, list!(lst, lst0));

        println!("{:#?}", test);
    }

    #[test]
    fn list_second_branch() {
        let list = list!((a + b, a, b); 
                                         a <- list!(1, 2, 3), 
                                         b <- list!(4, 5, 6)
                                        );
        print!("{:?}", list);
    }
}


