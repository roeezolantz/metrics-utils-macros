use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, ItemFn, LitStr};

struct MacroArgs {
    custom_name: Option<LitStr>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(MacroArgs { custom_name: None });
        }

        let custom_name = input.parse()?;
        Ok(MacroArgs {
            custom_name: Some(custom_name),
        })
    }
}

/// A procedural macro attribute that measures the execution time of an async function.
///
/// This macro wraps an async function to record its execution duration as a histogram metric.
/// The duration is recorded using the `FunctionDurationSeconds` metric with a "function" label
/// containing either the function name or a custom name if provided.
///
/// # Arguments
///
/// * `attr` - Optional custom name for the metric label
/// * `item` - The async function to be measured
///
/// # Examples
///
/// Basic usage with default function name as label:
/// ```ignore
/// use metrics_macros::measured_async_function;
///
/// #[measured_async_function]
/// async fn process_data() {
///     // Function implementation
/// }
/// ```
///
/// Using a custom name for the metric label:
/// ```ignore
/// use metrics_macros::measured_async_function;
///
/// #[measured_async_function("custom_process_name")]
/// async fn process_data() {
///     // Function implementation
/// }
/// ```
///
/// The macro will record timing metrics that can be queried like:
/// `function_duration_seconds{function="process_data"}` or
/// `function_duration_seconds{function="custom_process_name"}`
///
#[proc_macro_attribute]
pub fn measured_async_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let sig = &input_fn.sig;

    let metric_name = match args.custom_name {
        Some(name) => quote! { #name },
        None => quote! { stringify!(#fn_name) },
    };

    let output = quote! {
        #(#attrs)*
        #vis #sig {
            let __measured_async = async move #fn_block;
            let __measured_start = std::time::Instant::now();
            let __measured_result = __measured_async.await;
            let __measured_duration = __measured_start.elapsed().as_millis() as f64;
            metrics::histogram!(
                "async_function_duration_milliseconds",
                &[("function", #metric_name)]
            ).record(__measured_duration);
            __measured_result
        }
    };

    output.into()
}

/// Same as measured_async_function but for sync functions
#[proc_macro_attribute]
pub fn measured_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let vis = &input_fn.vis;
    let attrs = &input_fn.attrs;
    let sig = &input_fn.sig;

    let metric_name = match args.custom_name {
        Some(name) => quote! { #name },
        None => quote! { stringify!(#fn_name) },
    };

    let output = quote! {
        #(#attrs)*
        #vis #sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_block)();
            let duration = start.elapsed().as_millis() as f64;

            metrics::histogram!(
                "function_duration_milliseconds",
                &[("function", #metric_name)]
            ).record(duration);

            result
        }
    };

    output.into()
}
