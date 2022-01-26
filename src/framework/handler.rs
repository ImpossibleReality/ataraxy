use std::future::Future;

trait Handler<Args>: Clone + 'static {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}

macro_rules! handler_tuple ({ $($param:ident)* } => {
    impl<Func, Fut, $($param,)*> Handler<($($param,)*)> for Func
    where
        Func: Fn($($param),*) -> Fut + Clone + 'static,
        Fut: Future,
    {
        type Output = Fut::Output;
        type Future = Fut;

        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
            (self)($($param,)*)
        }
    }
});

handler_tuple! {}
handler_tuple! { Arg1 }
handler_tuple! { Arg1 Arg2 }
handler_tuple! { Arg1 Arg2 Arg3 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 Arg8 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 Arg8 Arg9 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 Arg8 Arg9 Arg10 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 Arg8 Arg9 Arg10 Arg11 }
handler_tuple! { Arg1 Arg2 Arg3 Arg4 Arg5 Arg6 Arg7 Arg8 Arg9 Arg10 Arg11 Arg12 }