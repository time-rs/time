macro_rules! quote {
    ($($x:tt)*) => {{
        use proc_macro::*;
        quote_inner!([] [] $($x)*)
    }};
}

macro_rules! quote_inner {
    ([] []) => {
        TokenStream::new()
    };
    ([] [$($tokens:tt)+]) => {
        TokenStream::from_iter([$($tokens)+])
    };
    ([$($streams:tt)+] []) => {
        TokenStream::from_iter([$($streams)+])
    };
    ([$($streams:tt)+] [$($tokens:tt)+]) => {
        TokenStream::from_iter([
            $($streams)+
            TokenStream::from_iter([$($tokens)+])
        ])
    };

    // Only stream interpolation splits the accumulated token array.
    ([$($streams:tt)*] [] #S($e:expr) $($tail:tt)*) => {
        quote_inner!([
            $($streams)*
            $crate::to_tokens::ToTokenStream::into_token_stream($e),
        ] [] $($tail)*)
    };
    ([$($streams:tt)*] [$($tokens:tt)+] #S($e:expr) $($tail:tt)*) => {
        quote_inner!([
            $($streams)*
            TokenStream::from_iter([$($tokens)+]),
            $crate::to_tokens::ToTokenStream::into_token_stream($e),
        ] [] $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] #($e:expr) $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            $crate::to_tokens::ToTokenTree::into_token_tree($e),
        ] $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] #[$($inner:tt)*] $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            TokenTree::from(Punct::new('#', Spacing::Alone)),
            TokenTree::from(Group::new(
                Delimiter::Bracket,
                quote_inner!([] [] $($inner)*),
            )),
        ] $($tail)*)
    };

    ($streams:tt [$($tokens:tt)*] :: $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            TokenTree::from(Punct::new(':', Spacing::Joint)),
            TokenTree::from(Punct::new(':', Spacing::Alone)),
        ] $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] >> $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            TokenTree::from(Punct::new('>', Spacing::Joint)),
            TokenTree::from(Punct::new('>', Spacing::Alone)),
        ] $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] -> $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            TokenTree::from(Punct::new('-', Spacing::Joint)),
            TokenTree::from(Punct::new('>', Spacing::Alone)),
        ] $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] $l:lifetime $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            TokenTree::from(Punct::new('\'', Spacing::Joint)),
            TokenTree::from(Ident::new(
                stringify!($l).trim_start_matches('\''),
                Span::mixed_site(),
            )),
        ] $($tail)*)
    };

    ($streams:tt [$($tokens:tt)*] $first:tt :: $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            quote_token!($first),
            TokenTree::from(Punct::new(':', Spacing::Joint)),
            TokenTree::from(Punct::new(':', Spacing::Alone)),
        ] $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] $first:tt -> $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            quote_token!($first),
            TokenTree::from(Punct::new('-', Spacing::Joint)),
            TokenTree::from(Punct::new('>', Spacing::Alone)),
        ] $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] $first:tt # $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            quote_token!($first),
        ] # $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] $first:tt $second:tt $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            quote_token!($first),
            quote_token!($second),
        ] $($tail)*)
    };
    ($streams:tt [$($tokens:tt)*] $token:tt $($tail:tt)*) => {
        quote_inner!($streams [
            $($tokens)*
            quote_token!($token),
        ] $($tail)*)
    };
}

macro_rules! quote_token {
    ($i:ident) => {
        TokenTree::from(Ident::new(stringify!($i), Span::mixed_site()))
    };
    (0) => {
        TokenTree::from(Literal::usize_unsuffixed(0))
    };
    ($l:literal) => {
        TokenTree::from(Literal::string(&$l))
    };
    (($($inner:tt)*)) => {
        TokenTree::from(Group::new(
            Delimiter::Parenthesis,
            quote_inner!([] [] $($inner)*),
        ))
    };
    ([$($inner:tt)*]) => {
        TokenTree::from(Group::new(
            Delimiter::Bracket,
            quote_inner!([] [] $($inner)*),
        ))
    };
    ({$($inner:tt)*}) => {
        TokenTree::from(Group::new(
            Delimiter::Brace,
            quote_inner!([] [] $($inner)*),
        ))
    };
    (:) => { TokenTree::from(Punct::new(':', Spacing::Alone)) };
    (=) => { TokenTree::from(Punct::new('=', Spacing::Alone)) };
    (;) => { TokenTree::from(Punct::new(';', Spacing::Alone)) };
    (,) => { TokenTree::from(Punct::new(',', Spacing::Alone)) };
    (.) => { TokenTree::from(Punct::new('.', Spacing::Alone)) };
    (&) => { TokenTree::from(Punct::new('&', Spacing::Alone)) };
    (<) => { TokenTree::from(Punct::new('<', Spacing::Alone)) };
    (>) => { TokenTree::from(Punct::new('>', Spacing::Alone)) };
    (?) => { TokenTree::from(Punct::new('?', Spacing::Alone)) };
    (!) => { TokenTree::from(Punct::new('!', Spacing::Alone)) };
    (|) => { TokenTree::from(Punct::new('|', Spacing::Alone)) };
    (*) => { TokenTree::from(Punct::new('*', Spacing::Alone)) };
    (+) => { TokenTree::from(Punct::new('+', Spacing::Alone)) };
}
