pub trait TypeList {}

pub struct Nil;

impl TypeList for Nil {}

pub struct Cons<H, T>(H, T);

impl<H, T: TypeList> TypeList for Cons<H, T> {}

pub trait AppFront<I> {
    type Res;
    fn app_front(self, item: I) -> <Self as AppFront<I>>::Res;
}

impl<I> AppFront<I> for Nil {
    type Res = Cons<I, Nil>;

    fn app_front(self, item: I) -> <Self as AppFront<I>>::Res {
        Cons(item, Nil)
    }
}

impl<I, H, T: TypeList> AppFront<I> for Cons<H, T> {
    type Res = Cons<I, Self>;

    fn app_front(self, item: I) -> <Self as AppFront<I>>::Res {
        Cons(item, self)
    }
}

pub trait VectorList {}

impl VectorList for Nil {}

pub struct VCons<H, T>(Vec<H>, T);

impl<H, T: VectorList> VectorList for VCons<H, T> {}

pub trait VAppFront<I> {
    type Res;
    fn app_front(self, item: Vec<I>) -> <Self as VAppFront<I>>::Res;
}

impl<I> VAppFront<I> for Nil {
    type Res = VCons<I, Nil>;

    fn app_front(self, item: Vec<I>) -> <Self as VAppFront<I>>::Res {
        VCons(item, Nil)
    }
}

impl<I, H, T: VectorList> VAppFront<I> for VCons<H, T> {
    type Res = VCons<I, Self>;

    fn app_front(self, item: Vec<I>) -> <Self as VAppFront<I>>::Res {
        VCons(item, self)
    }
}

pub trait Expand: VectorList {
    type Res;

    fn expand(self) -> <Self as Expand>::Res;
}