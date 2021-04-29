use std::{collections::binary_heap::Iter, rc::Rc};

#[derive(Clone, Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>)
}

#[derive(Clone, Debug)]
pub struct PList<T>(pub Rc<List<T>>);

pub fn nil<T>() -> PList<T> {
    PList(Rc::new(List::Nil))
}

pub fn cons<T>(head: T, tail: PList<T>) -> PList<T> {
    PList(Rc::new(List::Cons(head, tail.0)))
}

fn inner_rev<T: Clone>(xs: PList<T>, acc: PList<T>) -> PList<T> {
    match &*(xs.0) {
        List::Nil => acc,
        List::Cons(h, t) => 
            inner_rev(PList(t.clone()), cons(h.clone(),acc))
    }
}

pub fn rev<T: Clone>(xs: PList<T>) -> PList<T> {
    inner_rev(xs, nil())
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::Nil
    }
}

impl<'a, T> Iterator for &'a List<T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            List::Nil => None,
            List::Cons(head, tail) => {
                *self = tail;
                Some(head)
            },
        }
    }
}

impl<T: Clone> std::ops::Add for PList<T> {
    type Output = Self;

    fn add(self, rhs: PList<T>) -> Self::Output {
        match &*(self.0) {
            List::Nil => rhs,
            List::Cons(h, t) => cons(h.clone(), PList(t.clone()) + rhs),
        }
    }
}

pub fn map<A: Clone, B, F>(a: PList<A>, f: F) -> PList<B>
where F: Fn(A) -> B {
    match &*a.0 {
        List::Nil => nil(),
        List::Cons(h, t) => cons(f(h.clone()), map(PList(t.clone()), f))
    }
}

pub fn next<T: Clone>(lst: &mut PList<T>) -> Option<T> {
    let (res, new_lst) = match &*(*lst).0 {
        List::Nil => (None, nil()),
        List::Cons(h, t) => {
            (Some(h.clone()), PList(t.clone()))
        },
    };

    *lst = new_lst;
    res
}