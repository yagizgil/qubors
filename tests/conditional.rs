use qubors::Circuit;
use qubors::gates;
#[test]
fn test_quantum_conditional() {
    let mut cir = Circuit::new(3);

    cir._x(0); 

    cir._h(1);
    cir._cx(1, 2);

    cir._cx(0, 1);
    cir._h(0);

    cir._measure(0, 0); 
    cir._measure(1, 1);

    cir.c_if(1, 1, gates::X { target: 2 });
    cir.c_if(0, 1, gates::Z { target: 2 });

    cir.execute_res();

    let final_state = cir.measure(); 
    
    println!("success: |{:03b}⟩", final_state);
}