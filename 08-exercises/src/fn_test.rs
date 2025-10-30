// struct Example<'a, A, F>
// where F: Fn(&'a A) -> bool {
//     zero: &'a A,
//     foo: F,
//     bar: F
// }
// impl<'a, A, F> Example<'a, A, F>
// where F: Fn(&'a A) -> bool {
//     fn new(zero: &'a A, foo: F, maybe_bar: Option<F>) -> Example<'a, A, F> {
//         // mismatched types
//         // expected type parameter `F`
//         //          found closure `{closure@src/fn_test.rs:10:42: 10:49}`
//         // every closure has a distinct type and so could not always match the caller-chosen type of parameter `F`
//         let bar: F = maybe_bar.unwrap_or(|a: &A| !foo(a));
//         Example {
//             zero,
//             foo,
//             bar
//         }
//     }
// }

// struct FnPointerExample<'a, A> {
//     zero: &'a A,
//     foo: fn(&'a A) -> bool,
//     bar: fn(&'a A) -> bool
// }
// impl<'a, A> FnPointerExample<'a, A> {
//     fn new(zero: &'a A, foo: fn(&'a A) -> bool, maybe_bar: Option<fn(&'a A) -> bool>) -> FnPointerExample<'a, A> {
//         // mismatched types
//         // expected fn pointer `fn(&A) -> bool`
//         //       found closure `{closure@src/fn_test.rs:34:42: 34:49}`rustcClick for full compiler diagnostic
//         // fn_test.rs(34, 32): arguments to this method are incorrect
//         // fn_test.rs(34, 50): closures can only be coerced to `fn` types if they do not capture any variables
//         // fn_test.rs(34, 22): the return type of this call is `{closure@src/fn_test.rs:34:42: 34:49}` due to the type of the argument passed
//         let bar: fn(&'a A) -> bool = maybe_bar.unwrap_or(|a: &A| !foo(a));
//         // This works, if nothing has to change about the function to satisfy the default
//         let bar: fn(&'a A) -> bool = maybe_bar.unwrap_or(foo);
//         FnPointerExample {
//             zero,
//             foo,
//             bar
//         }
//     }
// }

// struct BoxedFnExample<'a, A> {
//     zero: &'a A,
//     foo: Box<dyn Fn(&'a A) -> bool + 'a>,
//     bar: Box<dyn Fn(&'a A) -> bool + 'a>
// }
// impl<'a, A> BoxedFnExample<'a, A> {
//     fn new(zero: &'a A, foo: Box<dyn Fn(&'a A) -> bool + 'a>, maybe_bar: Option<Box<dyn Fn(&'a A) -> bool + 'a>>) -> BoxedFnExample<'a, A> {
//         // closure may outlive the current function, but it borrows `foo`, which is owned by the current function
//         // may outlive borrowed value `foo`
//         let bar = maybe_bar.unwrap_or(Box::new(|a: &A| !foo(a)));
//         BoxedFnExample {
//             zero,
//             // cannot move out of `foo` because it is borrowed
//             // move out of `foo` occurs here
//             foo,
//             bar
//         }
//     }
// }

// use std::marker::PhantomData;

// struct TypeClassExample<'a, A, FB>
// where FB: FooBar<A> {
//     zero: &'a A,
//     foo_bar: FB
// }
// impl<'a, A, FB> TypeClassExample<'a, A, FB>
// where FB: FooBar<A> {
//     fn new(zero: &'a A, foo_bar: FB) -> TypeClassExample<'a, A, FB> {
//         TypeClassExample { zero, foo_bar }
//     }
// }
// trait FooBar<A> {
//     fn foo(&self, a: &A) -> bool;
//     fn bar(&self, a: &A) -> bool;
// }
// impl<A, F> FooBar<A> for F
// where F: Fn(&A) -> bool {

//     fn foo(&self, a: &A) -> bool {
//         self(a)
//     }

//     fn bar(&self, a: &A) -> bool {
//         !self.foo(a)
//     }
// }
// struct FooAndBar<A, FF, FB>
// where
//     FF: Fn(&A) -> bool,
//     FB: Fn(&A) -> bool
// {
//     foo: FF, bar: FB, phantom: PhantomData<A>,
// }
// impl<A, FF, FB> FooBar<A> for FooAndBar<A, FF, FB>
// where
//     FF: Fn(&A) -> bool,
//     FB: Fn(&A) -> bool
// {
//     fn foo(&self, a: &A) -> bool {
//         (self.foo)(a)
//     }

//     fn bar(&self, a: &A) -> bool {
//         (self.bar)(a)
//     }
// }
