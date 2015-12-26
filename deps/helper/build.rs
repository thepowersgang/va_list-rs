extern crate gcc;

fn main() {
	::gcc::compile_library("libva_list_test.a", &["src/helper.c"]);
}
