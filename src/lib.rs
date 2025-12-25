#![doc = include_str!("../README.md")]
//!
//! ## Basic route macro usage
//! See the docs of [`axum_typed_routing`] for details on the route macro.
//! For convenience we re-export the route macro & TypedRouter for you
//! so that all you need to use on your side is `use axum_controller::*`
//!
//! ## Controller macro usage
//!
//! This crate also offers a controller() attribute macro.
//! use it like this:
//!
//! ```
#![doc = include_str!("../examples/controller.rs")]
//! ```

#![forbid(unsafe_code)]
#![feature(proc_macro_diagnostic)]
use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    ItemImpl, MetaNameValue,
};
#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

#[derive(Clone, Default)]
struct MyAttrs {
    middlewares: Vec<syn::Expr>,
    path: Option<syn::Expr>,
    state: Option<syn::Expr>,
}

impl Parse for MyAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut path: Option<syn::Expr> = None;
        let mut state: Option<syn::Expr> = None;
        let mut middlewares: Vec<syn::Expr> = Vec::new();

        // some = "values", seperated = "with", commas = true
        for nv in Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?.into_iter() {
            let segs = nv.path.segments.clone().into_pairs();
            let seg = segs.into_iter().next().unwrap().into_value();
            let ident = seg.ident;
            match ident.to_string().as_str() {
                "path" => {
                    if path.is_some() {
                        return Err(syn::Error::new_spanned(path, "duplicate `path` attribute"));
                    }
                    path = Some(nv.value);
                }
                "state" => {
                    if state.is_some() {
                        return Err(syn::Error::new_spanned(
                            state,
                            "duplicate `state` attribute",
                        ));
                    }
                    state = Some(nv.value);
                }
                "middleware" => middlewares.push(nv.value),
                _ => {
                    panic!(
                        "Unknown attribute given to controller macro, only path,state & middleware allowed"
                    )
                }
            }
        }
        Ok(Self {
            state,
            path,
            middlewares,
        })
    }
}

#[derive(Clone)]
struct MyItem {
    struct_name: syn::Type,
    route_fns: Vec<syn::Ident>,
}

impl Parse for MyItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ast: ItemImpl = input.parse()?;
        let struct_name = *(ast.clone().self_ty.clone());
        let mut route_fns: Vec<syn::Ident> = vec![];

        for item in ast.items.iter() {
            if let syn::ImplItem::Fn(impl_item_fn) = item {
                // let fn_name = &impl_item_fn.sig.ident;
                for attr in impl_item_fn.attrs.clone() {
                    if attr.path().is_ident("route") {
                        let fn_name: Ident = impl_item_fn.sig.ident.clone();
                        route_fns.push(fn_name);
                    }
                }
            }
        }

        Ok(Self {
            route_fns,
            struct_name,
        })
    }
}

// TODO add better docs
/// A macro that generates a into_router(\_: State<_>) impl which automatically wires up all `route`'s and the given middlewares, path-prefix etc
///
/// ## Syntax:
/// ```
/// use axum_controller::controller;
///
/// struct ExampleController;
/// #[controller(
///   path = "/asd",
/// )]
/// impl ExampleController { /* ... */ }
/// ```
/// - path
///   - optional, 0-1 allowed, defaults to `"/"`
///   - A path to prefix `.nest` the `routes` in the controller Struct under
/// - state
///   - optional, 0-1 allowed, defaults to `"()"`)
///   - The type signature of the state given to the routes
/// - middleware
///   - optional, 0-n allowed, default to [] (no middlewares)
///   - Middlewares to `.layer` in the created router
///
#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MyAttrs);
    let item2: proc_macro2::TokenStream = item.clone().into();
    let myimpl = parse_macro_input!(item as MyItem);

    let state = args.state.unwrap_or(parse_quote!(()));
    let route_fns = myimpl.route_fns;
    let struct_name = &myimpl.struct_name;
    let route = args.path.unwrap_or(syn::parse_quote!("/"));

    let route_calls = route_fns
        .into_iter()
        .map(move |route| {
            quote! {
            .typed_route(#struct_name :: #route)

                  }
        })
        .collect::<Vec<_>>();

    let nesting_call = quote! {
        .nest(#route, __nested_router)
    };

    let nested_router_qoute = quote! {
        axum::Router::new()
        #nesting_call
    };
    let unnested_router_quote = quote! {
        __nested_router
    };
    let maybe_nesting_call = if let syn::Expr::Lit(lit) = route {
        if lit.eq(&syn::parse_quote!("/")) {
            unnested_router_quote
        } else {
            nested_router_qoute
        }
    } else {
        nested_router_qoute
    };

    let middleware_calls = args
        .middlewares
        .clone()
        .into_iter()
        .map(|middleware| quote! {.layer(#middleware)})
        .collect::<Vec<_>>();

    // TODO Checck if 2 possible to make 2 impls
    // where state of parent router is ()
    // one where it's #state
    let from_controller_into_router_impl = quote! {
        impl #struct_name {
            pub fn into_router(state: #state) -> axum::Router<#state> {
                let __nested_router = axum::Router::new()
                    #(#route_calls)*
                    #(#middleware_calls)*
                    .with_state(state)
                    ;

                    #maybe_nesting_call
            }
        }
    };

    let res: TokenStream = quote! {
        #item2
        #from_controller_into_router_impl
    }
    .into();

    res
}
