use darling::{ast, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Endpoint, attributes(endpoint))]
pub fn derive_endpoint(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let receiver = match EndpointReceiver::from_derive_input(&input) {
        Ok(receiver) => receiver,
        Err(e) => return e.write_errors().into(),
    };
    let tokens = quote!(#receiver);
    tokens.into()
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(endpoint), supports(struct_any))]
struct EndpointReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), EndpointFields>,
    method: syn::Ident,
    path: String,
}

#[derive(Debug, FromField)]
#[darling(attributes(endpoint))]
struct EndpointFields {
    ident: Option<syn::Ident>,
    ty: syn::Type,

    #[darling(default)]
    body: bool,

    #[darling(default)]
    skip: bool,
}

impl ToTokens for EndpointReceiver {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let generics = &self.generics;
        let data = &self.data;
        let method = &self.method;
        let path = &self.path;

        let (imp, ty, wher) = generics.split_for_impl();

        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        let mut endpoint = path.clone();
        let mut endpoint_field: Option<&syn::Ident> = None;
        let mut query_params = vec![];
        let mut body_params = vec![];

        for field in fields {
            if field.skip {
                continue;
            }

            let field_name = field.ident.as_ref().expect("Should never be unnamed");
            let field_name_str = field_name.to_string();

            if endpoint_field.is_none() && path.contains(&format!("{{{field_name_str}}}")) {
                endpoint = endpoint.replace(&format!("{{{field_name_str}}}"), "{}");
                endpoint_field = Some(field_name);
                continue;
            }

            let field_name_str = if field_name_str.eq("type_") || field_name_str.eq("_type") {
                "type".to_owned()
            } else {
                field_name_str
            };

            if !field.body {
                if type_is_option(&field.ty) {
                    query_params.push(quote! {
                        params.push_opt(#field_name_str, self.#field_name.as_ref());
                    });
                } else if type_is_vec(&field.ty) {
                    query_params.push(quote! {
                        params.push(#field_name_str, &self.#field_name.join(","));
                    });
                } else {
                    query_params.push(quote! {
                        params.push(#field_name_str, &self.#field_name);
                    });
                }
            }

            if field.body {
                body_params.push(quote! {
                    #field_name_str: self.#field_name,
                });
            }
        }

        let endpoint_impl = if let Some(field_name) = endpoint_field {
            quote! {
                format!(#endpoint, self.#field_name).into()
            }
        } else {
            quote! {
                #path.into()
            }
        };

        let params_impl = quote! {
            let mut params = crate::api::QueryParams::default();
            #(#query_params)*
            params
        };

        let body_impl = if body_params.is_empty() {
            quote! {
                Ok(None)
            }
        } else {
            quote! {
                crate::api::JsonParams::into_body(&serde_json::json!({
                    #(#body_params)*
                }))
            }
        };

        tokens.extend(quote! {
            impl #imp crate::api::Endpoint for #ident #ty #wher {
                fn method(&self) -> http::Method {
                    http::Method::#method
                }

                fn endpoint(&self) -> std::borrow::Cow<'static, str> {
                    #endpoint_impl
                }

                fn parameters(&self) -> crate::api::QueryParams<'_> {
                    #params_impl
                }

                fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, crate::api::BodyError> {
                    #body_impl
                }
            }
        });
    }
}

fn type_is_option(ty: &syn::Type) -> bool {
    type_parameter_of_option(ty).is_some()
}

fn type_is_vec(ty: &syn::Type) -> bool {
    type_parameter_of_vec(ty).is_some()
}

fn type_parameter_of_option(ty: &syn::Type) -> Option<&syn::Type> {
    let path = match ty {
        syn::Type::Path(ty) => &ty.path,
        _ => return None,
    };

    let last = path.segments.last().expect("Should never be empty");
    if last.ident != "Option" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(bracketed) = &last.arguments else {
        return None;
    };

    if bracketed.args.len() != 1 {
        return None;
    }

    match &bracketed.args[0] {
        syn::GenericArgument::Type(arg) => Some(arg),
        _ => None,
    }
}

fn type_parameter_of_vec(ty: &syn::Type) -> Option<&syn::Type> {
    let path = match ty {
        syn::Type::Path(ty) => &ty.path,
        _ => return None,
    };

    let last = path.segments.last().expect("Should never be empty");
    if last.ident != "Vec" {
        return None;
    }

    let syn::PathArguments::AngleBracketed(bracketed) = &last.arguments else {
        return None;
    };

    if bracketed.args.len() != 1 {
        return None;
    }

    match &bracketed.args[0] {
        syn::GenericArgument::Type(arg) => Some(arg),
        _ => None,
    }
}
