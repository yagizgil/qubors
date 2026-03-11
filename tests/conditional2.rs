use qubors::Circuit;
use qubors::gates;

#[test]
fn test_conditional_reset_multi_qubit() {
    let mut cir = Circuit::new(3);
    let shots = 1000;
    let mut final_counts = std::collections::HashMap::new();

    cir._x(2); 

    cir._h(0);
    cir._measure(0, 0);
    
    cir.c_if(0, 1, gates::X { target: 0 });

    for _ in 0..shots {
        let result = cir.execute_res();
        *final_counts.entry(result).or_insert(0) += 1;
    }

    for (state, count) in &final_counts {
        println!("State |{:03b}⟩ (Dec: {}): {} occurrences", state, state, count);
    }


    let count_4 = *final_counts.get(&4).unwrap_or(&0);
    let digerleri = shots - count_4;

    println!("-------------------------------");
    println!("Success |100⟩: {}", count_4);
    println!("Other States: {}", digerleri);

}