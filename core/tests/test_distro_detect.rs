use linops_core::distro;

#[test]
fn test_detect_returns_some_distro() {
    let d = distro::detect::detect();
    println!("detected distro: {:?}", d);
}
