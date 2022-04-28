fn main() {
    let mut args = std::env::args_os();

    println!(
        "{}",
        ms_pdb_linkgen::generate_link(&args.nth(1).unwrap()).unwrap()
    );
}
