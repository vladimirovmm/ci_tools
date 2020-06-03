mod citool;

fn main() {
    let mut citool = citool::CiTools::new(std::env::args().collect());
    citool.run();
}
