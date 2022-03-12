macro_rules! quote {
    () => (::proc_macro::TokenStream::new());
    ($($x:tt)*) => (quote_inner!([$($x)*] -> []));
}

macro_rules! quote_inner {
    // Base case
    ([] -> []) => (::proc_macro::TokenStream::new());
    ([] -> [$($accum:tt)*]) => {
        [$($accum)*].iter().cloned().collect::<::proc_macro::TokenStream>()
    };

    // Symbols and symbol pairs
    ([:: $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new(':', ::proc_macro::Spacing::Joint)
        )),
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new(':', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([.. $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('.', ::proc_macro::Spacing::Joint)
        )),
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('.', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([: $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new(':', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([= $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('=', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([; $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new(';', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([, $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new(',', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([. $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('.', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([& $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('&', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([<< $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('<', ::proc_macro::Spacing::Joint)
        )),
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('<', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([< $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('<', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([>> $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('>', ::proc_macro::Spacing::Joint)
        )),
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('>', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([> $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('>', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([-> $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('-', ::proc_macro::Spacing::Joint)
        )),
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('>', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([? $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('?', ::proc_macro::Spacing::Alone)
        )),
    ]));
    ([| $($tail:tt)*] -> [$($accum:tt)*] ) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('|', ::proc_macro::Spacing::Alone)
        )),
    ]));

    // Identifier
    ([$i:ident $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Ident::new(stringify!($i), ::proc_macro::Span::mixed_site())
        )),
    ]));

    // Literal
    ([$l:literal $($tail:tt)*] -> [$($expanded:tt)*]) => (quote_inner!([$($tail)*] -> [$($expanded)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Literal::string($l)
        )),
    ]));


    // Lifetime
    ([$l:lifetime $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Punct::new('\'', ::proc_macro::Spacing::Joint)
        )),
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(
            ::proc_macro::Ident::new(
                stringify!($l).trim_start_matches(|c: char| c == '\''),
                ::proc_macro::Span::mixed_site())
        )),
    ]));

    // Groups
    ([($($inner:tt)*) $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(::proc_macro::Group::new(
            ::proc_macro::Delimiter::Parenthesis,
            quote_inner!([$($inner)*] -> []))
        )),
    ]));
    ([{$($inner:tt)*} $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(::proc_macro::Group::new(
            ::proc_macro::Delimiter::Brace,
            quote_inner!([$($inner)*] -> [])
        ))),
    ]));
    ([[$($inner:tt)*] $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        ::proc_macro::TokenStream::from(::proc_macro::TokenTree::from(::proc_macro::Group::new(
            ::proc_macro::Delimiter::Bracket,
            quote_inner!([$($inner)*] -> [])
        ))),
    ]));

    // Interpolated values
    ([#($e:expr) $($tail:tt)*] -> [$($accum:tt)*]) => (quote_inner!([$($tail)*] -> [$($accum)*
        $crate::to_tokens::ToTokens::into_token_stream($e),
    ]));
}
