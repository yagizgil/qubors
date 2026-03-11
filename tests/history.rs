use qubors::{Circuit, OpcodeHistory};
use std::collections::HashMap;

#[test]
fn test_bell_state_entanglement() {
    let mut cir = Circuit::new(2);
    let mut hist = OpcodeHistory::new();

    cir._h(0);
    cir._cx(0, 1);

    let mut counts = HashMap::new();
    let shots = 10;

    for _ in 0..shots {
        let result = cir.execute_res();
        hist.push_all(&cir);
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
    println!("-------------------------------");
    
    for i in hist.get_str_list() {
        println!("{}",i)
    }

}