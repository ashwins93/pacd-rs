fn main() {
    let src_path = "./site";
    let dest_path = "./build";

    pacd::build_site(src_path, dest_path).unwrap();
}
