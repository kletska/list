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
