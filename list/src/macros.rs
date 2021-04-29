#[macro_export] macro_rules! list {
    ($($e:expr),*) => {
        {
            let mut res = crate::list::nil();

            $(
                res = crate::list::cons($e, res);
            )*

            crate::list::rev(res)
        }
    };
    ($ex:expr; $($id:ident <- $e:expr),*) => {
        {
            let res = crate::type_list::Nil;

            $(
                let res = crate::type_list::Cons($e, res);
            )*

            let res = crate::type_list::Reverse::reverse(res, crate::type_list::Nil);

            crate::list::map(crate::type_list::Expand::expand(res), |x| {
                $(
                    let $id = crate::type_list::First::first(& x);
                    let x = crate::type_list::Pop::pop(x);
                )*

                $ex
            })
        }
    }
}


#[macro_export] macro_rules! list_old {
    ($($e:expr),*) => {
        {
            let mut res = crate::list::nil();

            $(
                res = crate::list::cons($e, res);
            )*

            crate::list::rev(res)
        }
    };
    ($ex:expr; $($id:ident <- $e:expr),*) => {
        {
            let mut res = crate::list::nil();

            $(
                res = crate::list::cons($e, res);
            )*

            res = crate::list::rev(res);

            crate::list::map(crate::demo::expand(res), |mut x| {
                $(
                    let $id = crate::list::next(&mut x).unwrap();
                )*

                $ex
            })
        }
    }
}