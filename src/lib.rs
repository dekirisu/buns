use proc_macro::{Group, TokenStream, TokenTree};
use proc_macro2::TokenStream as Token2;
use quote::quote;
use maflow::*;

// Compose \\

    /// Create a code template and use it right after:
    ///
    /// 1. Write the code, use ^0 ^1 .. ^N as placeholders
    /// 2. Write the inserts: #0^1^..^N (numbers = any code)
    /// 3. Here is the point of it: repeat 2. if necessary
    #[proc_macro]
    pub fn sandwich(item:TokenStream) -> TokenStream {compose(item)}
    
    /// Create a code template and use it right after:
    ///
    /// 1. Write the code, use ^0 ^1 .. ^N as placeholders
    /// 2. Write the inserts: #0^1^..^N (numbers = any code)
    /// 3. Here is the point of it: repeat 2. if necessary
    #[proc_macro]
    pub fn compose(item:TokenStream) -> TokenStream {    
        let mut tokens = item.into_iter();
        // collect buns, nom 
        let mut buns = vec![];
        for a in &mut tokens {
            hold!{if a.is_punct() && a.equal_to_str("#")}
            buns.push(a);
        }
        let buns = TokenStream::from_iter(buns);
        // collect patties, nom
        let mut idx = (0,0);
        let mut patties: VecX3<TokenTree> = vec![];
        for token in tokens {
            match token.punct_string().as_str() {
                "#" => {
                    idx.0 += 1;
                    idx.1 = 0;
                    continue;
                },
                "^" => {
                    idx.1 += 1;
                    continue
                },
                _ => {}
            }
            patties.get_and_fill(idx).push(token);
        }
        // make the sandwiches, nom
        let mut sandwiches = vec![];
        for ingredients in patties {
            sandwiches.push(make_sandwich(buns.clone(),&ingredients));
        }
        TokenStream::from_iter(sandwiches) 
    }

    /// Place one chunk of inserts (#0^1^..^N) into the code placeholders (^0 ^1 .. ^N)
    ///
    /// ## Recursivity
    /// Necessary to check every token separately, because of grouping.
    /// 
    /// A suboptimal alternative to this, which could interfere with (token) diagnostics is to convert everything to strings and use e.g. .replace(..)
    fn make_sandwich(buns:TokenStream,ingredients:&Vec<Vec<TokenTree>>) -> TokenStream {
        let mut out = vec![];    
        let mut may_id = false;
        for crumb in buns {match crumb {
            TokenTree::Group(g) => out.push(TokenTree::Group(
                Group::new(g.delimiter(),make_sandwich(g.stream(),ingredients))
            )),
            _ => match may_id && crumb.is_integer() {
                true => {
                    kill!{id = crumb.parse::<usize>()}
                    out.pop();
                    out.extend(ingredients[id].clone());
                    may_id = false;
                }
                false => {
                    may_id = crumb.is_punct() && crumb.equal_to_str("^");
                    out.push(crumb);
                }
            }
        }}
        TokenStream::from_iter(out)
    }

// Prepare \\

    /// Create a code template and use it later (basically simplified macro_rules):
    ///
    /// 1. Define the name of the template (e.g. `burger`)
    /// 2. Write the code, use ^0,^1,..,^N as placeholder
    /// 3. Use it anywhere: `burger!{#0^1^..^N #0^1^..^N}` (numbers = any code)
    #[proc_macro]
    pub fn prepare(item:TokenStream) -> TokenStream {
        preset(item)
    }

    /// Create a code template and use it later (basically simplified macro_rules):
    ///
    /// 1. Define the name of the template (e.g. `burger)
    /// 2. Write the code, use ^0,^1,..,^N as placeholder
    /// 3. Use it anywhere: `burger!{#0^1^..^N #0^1^..^N}` (numbers = any code)
    #[proc_macro]
    pub fn preset(item:TokenStream) -> TokenStream {
        let mut iter = item.into_iter();    
        let name:Token2 = TokenStream::from_iter([iter.next().unwrap()]).into();
        let code = TokenStream::from_iter(iter);
        let docs:Token2 = code.to_docs().into();
        let item:Token2 = code.into();
        quote!{
            /// buns preset, that executes:
            /// ``` rust
            #docs
            /// ```
            #[macro_export]
            macro_rules! #name {(#$($tt:tt)*)=>{
                buns::compose!{#item # $($tt)*}
            }} 
        }.into()
    }

// Helpers \\
use extension_traits::extension as ext;
use std::str::FromStr;

    type VecX3<T> = Vec<Vec<Vec<T>>>;

    #[ext(trait TokenStreamExt)]
    impl TokenStream {
        fn to_docs(&self) -> TokenStream {
            TokenStream::from_str(
                &format!("{self}").to_string().split_inclusive("; ")
                .map(|a|format!("#[doc=r#\"    {a}\"#]")).collect::<String>()
                .replace("^ ","^").replace(" ! ","!").replace(" ;",";")
            ).unwrap()
        }
    }

    #[ext(trait TokenTreeExt)]
    impl TokenTree {
        fn is_literal(&self) -> bool {match self {
            Self::Literal(_) => true,
            _ => false
        }}
        fn is_integer(&self) -> bool {
            exit!{if !self.is_literal()}
            self.parse::<isize>().yay()
        }
        fn is_punct(&self) -> bool {match self {
            Self::Punct(_) => true,
            _ => false
        }}
        /// to_string with type check
        fn punct_string<'a>(&self) -> String {
            exit!{if !self.is_punct()}
            self.to_string()
        }
        fn equal_to_str(&self,text:&str) -> bool {
            &self.to_string() == text
        }
        fn parse<F:FromStr>(&self) -> Option<F> {
            self.to_string().parse::<F>().ok()
        }
    }

    #[ext(trait VecVecExt)]
    impl <T:Default> Vec<Vec<T>> {
        /// initialize empty vecs until possible to address with given indexes
        ///
        /// returns mutable reference to the entry
        fn get_and_fill(&mut self,idx:(usize,usize)) -> &mut T {
            while self.len() <= idx.0 {self.push(vec![]);}
            while self[idx.0].len() <= idx.1 {self[idx.0].push(T::default());}
            &mut self[idx.0][idx.1]
        }
    }

// EOF \\
