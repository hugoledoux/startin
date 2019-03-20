extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/predicates.c")
        .compile("libpredicates.a");
}
