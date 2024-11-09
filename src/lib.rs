#![feature(macro_metavar_expr)]
#![no_std]

/// link an ident (turned into a macro) to pairs of tokens (buns)...
/// ```rust
///     buns::preset!{usemod [mod ][;] [use ][::*;]}
/// ```
///
/// ...and define idents to sandwich between later
/// ```rust
///     usemod!{util,nice}
///     // Same as:
///     // mod util; use util::*;
///     // mod nice; use nice::*;
/// ```
#[macro_export]
macro_rules! preset {($name:ident $($tt:tt)*)=>{
    #[macro_export]
    macro_rules! $name {($$($$item:ident),*)=>{
        buns::apply!{$($tt)* $$($$item),*}
    }}
}} 

macro_rules! apply_helper {
    ($([$($pre:tt)*][$($post:tt)*])*) => {
        /// define pairs of tokens (buns) and list multiple idents to sandwich between
        /// ```rust
        ///     buns::apply!{[mod ][;] [use ][::*;] util,nice}
        ///     // Same as:
        ///     // mod util; use util::*;
        ///     // mod nice; use nice::*;
        /// ```
        #[macro_export]
        macro_rules! apply {$(
            ($$([$($$$pre:tt)*][$($$$post:tt)*])+$$($$name:ident),*) 
            => {$$($$( $($$$pre)* $$name $($$$post)* )*)+};
        )*}
    };
}

apply_helper!{
    [][] [][a] [][a b] [][a b c] 
    [][a b c d] [][a b c d e]

    [z][a] [z][a] [z][a b] [z][a b c] 
    [z][a b c d] [z][a b c d e]

    [z y][] [z y][a] [z y][a b] [z y][a b c] 
    [z y][a b c d] [z y][a b c d e]

    [z y x][] [z y x][a] [z y x][a b] 
    [z y x][a b c] [z y x][a b c d] [z y x][a b c d e]

    [z y x w][] [z y x w][a] [z y x w][a b] 
    [z y x w][a b c] [z y x w][a b c d] [z y x w][a b c d e]

    [z y x w v][] [z y x w v][a] [z y x w v][a b] 
    [z y x w v][a b c] [z y x w v][a b c d] [z y x w v][a b c d e]
}
