extern crate cc;

fn main() {
    ::cc::Build::new()
        .file("src/helper.c")
        .compile("libva_list_test.a");
}
