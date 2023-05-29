use fill_in;

fn main() {
    let path = std::env::args().nth(1).expect("pls provide input file");
    fill_in::load_and_solve(&path);
}
