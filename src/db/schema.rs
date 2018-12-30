#[allow(proc_macro_derive_resolution_fallback)]

table! {
    movies (movies_id) {
        movies_id -> Text,
        movies_name -> Text,
        movies_rating -> Text,
        movies_category -> Text,
        movies_format -> Text,
        movies_aspect -> Text,
        movies_actors -> Text,
        movies_studio_id -> Text,
    }
}

table! {
    studios (studios_id) {
        studios_id -> Text,
        studios_name -> Text,
    }
}