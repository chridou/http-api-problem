use derive_utils::quick_derive;
use proc_macro::TokenStream;

#[proc_macro_derive(IntoApiError)]
pub fn derive_iterator(input: TokenStream) -> TokenStream {
    quick_derive! {
        input,
        // trait path
        ::http_api_problem::IntoApiError,
        // trait definition
        pub trait IntoApiError {
            fn into_api_error(self) -> ::http_api_problem::ApiError;
        }
    }
}
