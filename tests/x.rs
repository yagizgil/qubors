use qubors::Circuit;

#[test]
fn test_x() {
    let mut cir = Circuit::new(3);
    cir.x(2);

    for i in 0..cir.state.len() {
        println!(
            "{}. index ({:0width$b}): {}",
            i,
            i,
            cir.state[i],
            width = cir.qubits
        );
    }

}
