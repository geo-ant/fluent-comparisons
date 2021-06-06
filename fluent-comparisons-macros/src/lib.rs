//! This crate contains the procedural macros for the fluent-comparisons crate

use proc_macro::TokenStream;

use syn::{BinOp, Expr, ExprLit};
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use proc_macro2::Span;
use syn::parse_macro_input;

use quote::quote;

mod keywords {
    syn::custom_keyword!(satisfy);
    syn::custom_keyword!(map);
    syn::custom_keyword!(of);
}

#[derive(Debug,Clone)]
struct ExactlyMacroPat {
    number_of : NumberOf,
    comparison_expr : ComparisonExpression,
}

impl Parse for ExactlyMacroPat {
    fn parse(input: & ParseBuffer) -> Result<Self, syn::Error> {

        Ok(Self {
            number_of : input.parse()?,
            comparison_expr : input.parse()?,
        })
    }
}

/// a structure to parse the pattern `$n::tt of`, where n must be an unsigned integer number
#[derive(Debug,Clone)]
struct NumberOf {
    /// the number
    n : Number,
    /// the keyword of
    of : keywords::of,
}

#[derive(Clone,Debug)]
enum Number {
    Literal(syn::LitInt),
    Identifier(syn::Ident),
}

use syn::parse::discouraged::Speculative;

impl Parse for Number {
    fn parse(input: & ParseBuffer) -> Result<Self, syn::Error> {
        let fork = input.fork();
        if let Ok(literal) = fork.parse::<syn::LitInt>() {
            input.advance_to(&fork);
            return Ok(Self::Literal(literal))
        }
        let fork = input.fork();
        if let Ok(ident) = fork.parse::<syn::Ident>() {
            input.advance_to(&fork);
            return Ok(Self::Identifier(ident))
        }

        Err(syn::Error::new(input.span(),"expected integer literal or identifier"))
    }
}

impl Parse for NumberOf {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        Ok(Self {
            n : input.parse()?,
            of : input.parse()?,
        })
    }
}

/// Parses a pattern `{$($exprs:expr),*} $op:tt $rhs:expr`, or `{$($exprs:expr),*}.map($($f:tt)*) $op:tt $rhs:expr`,
/// or `{$($exprs:expr),*}.satisfy($($f:tt)*)`. This is the pattern that is given
/// to (most of) the other macros in the crate.
/// This is not the full expression that will be passed to the exactly! macro, but it might come
/// in handy, when I want to refactor the other macros as procedural macros as well.
#[derive(Clone, Debug)]
struct ComparisonExpression {
    /// the list of expressions in the curly braces
    exprs: Vec<Expr>,
    /// a transformation applied to the data. There may be no transform,
    /// in which case this variant Identity
    transform: Transform,
    /// the binary comparison operator
    op: BinOp,
    /// the right hand side
    rhs: Expr,
}

/// check whether the given comparison operator is allowed.
/// # Returns
/// Ok(()) if the comparison operator is allowed, otherwise a descriptive error.
fn check_comparison_operator(op: &BinOp) -> Result<(), syn::Error> {
    match op {
        BinOp::Eq(_) | BinOp::Lt(_) | BinOp::Le(_) | BinOp::Ne(_) | BinOp::Ge(_) | BinOp::Gt(_) => { Ok(()) }
        _ => {
            Err(syn::Error::new(op.span(), "Illegal comparison operator. The only allowed operators are ==, !=, <=, >=, <, >"))
        }
    }
}

impl Parse for ComparisonExpression {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {

        let content;
        let _: syn::token::Brace = syn::braced!(content in input);
        let exprs = Punctuated::<Expr, syn::Token![,]>::parse_terminated(&content)?;
        let transform_call: TransformCall = input.parse()?;


        // parse the operators based on the transformation. For .satisfy(...) we expect no
        // op and rhs given and will assign (op,rhs) = (==, true). Otherwise we parse what the
        // user wrote
        let (op, rhs) : (BinOp, Expr)= match transform_call {
            TransformCall::Identity | TransformCall::Map(_) => {
                (input.parse()?, input.parse()?)
            }
            TransformCall::Satisfy(_) => {
                // here we make sure that nothing is left to parse after .satisfy(...)
                if !input.is_empty() {
                    return Err(syn::Error::new(input.span(), "Unexpected tokens after .satisfy(...)"));
                }
                // the equality comparison operator "=="
                let equality_operator = BinOp::Eq(syn::token::EqEq {spans : [Span::mixed_site();2]});
                // the bool literal "true"
                let true_expression = Expr::Lit(ExprLit{attrs: Default::default(), lit : syn::Lit::Bool(syn::LitBool{span : Span::mixed_site(),value:true})});
                (equality_operator,true_expression)
            }
        };

        let transform = match transform_call {
            TransformCall::Identity => { Transform::Identity}
            TransformCall::Map(trafo) => { Transform::Map(trafo)}
            TransformCall::Satisfy(trafo) => {Transform::Map(trafo)}
        };
        
        Ok(Self {
            exprs: exprs.into_iter().collect(),
            transform,
            op,
            rhs
        })
    }
}

//TODO this is all a little high concept for now but I want to keep this extendable to other transformations
// which follow the same logic as satisfy / map but do not require all expressions to be of the same
// type.
// This transform type is something that is found out by parsing ComparisonExpression because
// the comparison expression will parse the TransformCall and then itself transform a .satisfies(f) into
// a comparison expression with operator as " == " and rhs "true". So by the point the comparison
// expression is finally parsed, all mentions of satisfy will be gone and it will be transformed into
// the equivalent expression containing map.
#[derive(Clone, Debug)]
enum Transform {
    /// no transformation is applied to the data
    Identity,
    Map(proc_macro2::TokenStream),
}

#[derive(Clone, Debug)]
enum TransformCall {
    Identity,
    Map(proc_macro2::TokenStream),
    Satisfy(proc_macro2::TokenStream),
}

#[derive(Clone, Debug)]
enum TransformKind {
    Map,
    Satisfy,
}

impl Parse for TransformKind {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        //TODO check if this pattern is ok, or if I need to use the lookahead
        // see https://docs.rs/syn/1.0.72/syn/parse/index.html
        // for the pattern with the lookahead1.
        if input.peek(keywords::map) {
            let _: keywords::map = input.parse()?;
            Ok(Self::Map)
        } else if input.peek(keywords::satisfy) {
            let _: keywords::satisfy = input.parse()?;
            Ok(Self::Satisfy)
        } else {
            Err(syn::Error::new(input.span(), "Unknown transformation.Expected either 'map' or `satisfy`"))
        }
    }
}

impl Parse for TransformCall {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        if input.peek(syn::Token![.]) {
            let _: syn::Token![.] = input.parse()?;
            let transform_kind: TransformKind = input.parse()?;
            let content;
            let _: syn::token::Paren = syn::parenthesized!(content in input);
            let transformation: proc_macro2::TokenStream = content.parse()?;
            match transform_kind {
                TransformKind::Map => { Ok(Self::Map(transformation)) }
                TransformKind::Satisfy => {
                    Ok(Self::Satisfy(transformation))
                }
            }
        } else {
            Ok(Self::Identity)
        }
    }
}

#[proc_macro]
pub fn exactly(input: TokenStream) -> TokenStream {
    let i2 = input.clone();
    let _x = parse_macro_input!(i2 as ExactlyMacroPat);

    let tokens = quote! {
        true
    };
    tokens.into()
}