#![doc = include_str!("../README.md")]
//!
//! This crate is an extension of [`axum_typed_routing`]. It uses the `#[route]`
//! attribute and the `TypedRouter` trait from that crate to discover and wire up
//! handler methods. You **must** add `axum-typed-routing` to your own
//! `Cargo.toml` dependencies for this crate to work.
//!
//! ## Route macro usage
//! See the docs of [`axum_typed_routing`] for details on the `#[route]` macro.
//!
//! ## Controller macro usage
//!
//! This crate offers the `controller()` attribute macro.
//! use it like this:
//!
//! ```
#![doc = include_str!("../examples/controller.rs")]
//! ```

#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse_quote;
use syn::Token;

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    ItemImpl, MetaNameValue,
};

#[derive(Default)]
struct ControllerAttrs {
    middlewares: Vec<syn::Expr>,
    path: Option<syn::Expr>,
    state: Option<syn::Expr>,
}

impl Parse for ControllerAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut path: Option<syn::Expr> = None;
        let mut state: Option<syn::Expr> = None;
        let mut middlewares: Vec<syn::Expr> = Vec::new();

        for nv in Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)? {
            let segs = nv.path.segments.clone().into_pairs();
            let seg = segs.into_iter().next().unwrap().into_value();
            let ident = seg.ident;
            match ident.to_string().as_str() {
                "path" => {
                    if path.is_some() {
                        return Err(syn::Error::new_spanned(
                            &nv.path,
                            "duplicate `path` attribute",
                        ));
                    }
                    path = Some(nv.value);
                }
                "state" => {
                    if state.is_some() {
                        return Err(syn::Error::new_spanned(
                            &nv.path,
                            "duplicate `state` attribute",
                        ));
                    }
                    state = Some(nv.value);
                }
                "middleware" => middlewares.push(nv.value),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &nv.path,
                        format_args!(
                            "unknown attribute `{}`; expected `path`, `state`, or `middleware`",
                            ident
                        ),
                    ));
                }
            }
        }

        Ok(Self {
            middlewares,
            path,
            state,
        })
    }
}

struct ControllerImpl {
    struct_name: syn::Path,
    route_fns: Vec<syn::Ident>,
}

impl Parse for ControllerImpl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ast: ItemImpl = input.parse()?;
        let struct_name = match *ast.self_ty {
            syn::Type::Path(syn::TypePath { path, .. }) => path,
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "expected a path (struct name) as the impl target",
                ))
            }
        };
        let mut route_fns: Vec<syn::Ident> = vec![];

        for item in &ast.items {
            if let syn::ImplItem::Fn(impl_item_fn) = item {
                for attr in &impl_item_fn.attrs {
                    if attr.path().is_ident("route") {
                        let fn_name: Ident = impl_item_fn.sig.ident.clone();
                        route_fns.push(fn_name);
                    }
                }
            }
        }

        Ok(Self {
            struct_name,
            route_fns,
        })
    }
}

/// A macro that generates `into_router` and `into_app_router` methods which
/// automatically wire up all `#[route]` handlers and the given middlewares,
/// path-prefix etc.
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
pub fn controller(attrs: TokenStream, c_impl: TokenStream) -> TokenStream {
    let parsed_attrs = match syn::parse::<ControllerAttrs>(attrs) {
        Ok(args) => args,
        Err(err) => return err.to_compile_error().into(),
    };
    let parsed_impl = match syn::parse::<ControllerImpl>(c_impl.clone()) {
        Ok(myimpl) => myimpl,
        Err(err) => return err.to_compile_error().into(),
    };

    let state = parsed_attrs.state.unwrap_or_else(|| parse_quote!(()));
    let route_fns = parsed_impl.route_fns;
    let struct_name = &parsed_impl.struct_name;
    let route = parsed_attrs.path.unwrap_or_else(|| syn::parse_quote!("/"));

    let no_routes_warning = if route_fns.is_empty() {
        quote! {
            #[deprecated(note = "#[controller] applied to impl with no #[route] attributes")]
            const __AXUM_CONTROLLER_NO_ROUTES_WARNING: () = ();
            const _: () = __AXUM_CONTROLLER_NO_ROUTES_WARNING;
        }
    } else {
        quote! {}
    };

    let route_calls = route_fns
        .into_iter()
        .map(move |route| {
            quote! {
            .typed_route(#struct_name :: #route)
                  }
        })
        .collect::<Vec<_>>();

    let nesting_call = quote! {
        .nest(#route, nested_router)
    };

    let nested_router_quote = quote! {
        axum::Router::new()
        #nesting_call
    };
    let unnested_router_quote = quote! {
        nested_router
    };
    let maybe_nesting_call = if let syn::Expr::Lit(lit) = route {
        if lit.eq(&syn::parse_quote!("/")) {
            unnested_router_quote
        } else {
            nested_router_quote
        }
    } else {
        nested_router_quote
    };

    let middleware_calls = parsed_attrs
        .middlewares
        .into_iter()
        .map(|middleware| quote! {.layer(#middleware)})
        .collect::<Vec<_>>();

    let from_controller_into_router_impl = quote! {
        impl #struct_name {
            pub fn into_app_router(state: #state) -> axum::Router<()> {
                Self::into_router()
                    .with_state(state)
            }

            pub fn into_router() -> axum::Router<#state> {
                let nested_router = axum::Router::new()
                    #(#route_calls)*
                    #(#middleware_calls)*
                    ;

                    #maybe_nesting_call
            }
        }
    };

    let c_impl: proc_macro2::TokenStream = c_impl.into();
    let res: TokenStream = quote! {
        #c_impl
        #from_controller_into_router_impl
        #no_routes_warning
    }
    .into();

    res
}
