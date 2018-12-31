#[allow(proc_macro_derive_resolution_fallback)]

table! {
    movies (movies_id) {
        movies_id -> Text,
        movies_title -> Text,
        movies_rating -> Text,
        movies_category -> Text,
        movies_format -> Text,
        movies_aspect -> Text,
        movies_actors -> Text,
        movies_drawer -> Text,
        movies_column -> Text,
    }
}
