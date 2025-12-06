fn main() {
    println!(
        "{}",
        ms_pdb_linkgen::generate_link(std::env::args_os().nth(1).unwrap()).unwrap()
    );
}
