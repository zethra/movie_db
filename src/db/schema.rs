#[allow(proc_macro_derive_resolution_fallback)]

table! {
    movies (movies_id) {
        movies_id -> Text,
        movies_name -> Text,
        movies_rating -> Text,
        movies_category -> Text,
        movies_format -> Text,
        movies_aspect -> Text,
    }
}