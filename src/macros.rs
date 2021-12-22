/// Macro generating a type for a linked list of given
/// contents
#[macro_export]
macro_rules! TList {
    [ $head:ty, $( $tail:ty  ),+ ] => {
        $crate::Cons<$head, TList![$($tail)+]>
    };
    [ $head:ty ] => {
        $crate::Cons<$head, $crate::Nil<$head>>
    };
    [] => {
        $crate::Nil<()>
    };
}

/// Macro for creating a linked list ergonomically
#[macro_export]
macro_rules! tlist {
    [ $head:tt, $( $tail:tt ),+ ] => {
        $crate::Cons($head, tlist![$($tail),+])
    };
    [ $head:tt ] => {
        $crate::Cons($head, $crate::Nil(std::marker::PhantomData))
    };
    [] => {
        $crate::Nil(PhantomData)
    };
}
