use qubors::Circuit;

#[test]
fn test_hadamard() {
    let mut cir = Circuit::new(5);
    cir.h(3);
    cir.h(4);
    cir.h(2);

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
