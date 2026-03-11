use qubors::Circuit;
use std::collections::HashMap;

#[test]
fn test_bell_state_entanglement() {
    let mut cir = Circuit::new(2);

    cir._h(0);
    cir._cx(0, 1);

    let mut counts = HashMap::new();
    let shots = 1000;

    for _ in 0..shots {
        let result = cir.execute_res();
        *counts.entry(result).or_insert(0) += 1;
    }

    println!("\n--- Execution Results ({} Shots) ---", shots);
    for (state, count) in &counts {
        println!("State |{:02b}⟩: {} occurrences", state, count);
    }

    let count_00 = *counts.get(&0).unwrap_or(&0);
    let count_11 = *counts.get(&3).unwrap_or(&0);
    let errors = counts.get(&1).unwrap_or(&0) + counts.get(&2).unwrap_or(&0);

    println!("-------------------------------");
    println!("Success (00 + 11): {}", count_00 + count_11);
    println!("Invalid States (01 + 10): {}", errors);

    assert_eq!(
        errors, 0,
        "Entanglement failure! Forbidden states detected."
    );

    assert!(count_00 > 400 && count_00 < 600, "Distribution for |00⟩ is outside expected range!");
    assert!(count_11 > 400 && count_11 < 600, "Distribution for |11⟩ is outside expected range!");
}