use includedir_codegen::{self, Compression};

fn main() {
    includedir_codegen::start("DB_SETUP")
        .dir("sql", Compression::Gzip)
        .build("db_setup.rs")
        .unwrap();
//    includedir_codegen::start("STATIC_FILES")
//        .dir("static", Compression::Gzip)
//        .build("static_files.rs")
//        .unwrap();
}
