#[test]
fn test_measurement() {
    let mut count_0 = 0;
    let mut count_1 = 0;

    for _ in 0..100 {
        let mut cir = qubors::Circuit::new(2);
        cir.h(0);
        let result = cir.measure();

        if result == 0 {
            count_0 += 1;
        } else {
            count_1 += 1;
        }
    }

    println!("0 ratio: {}", count_0);
    println!("1 ratio: {}", count_1);

    assert!(count_0 > 30 && count_0 < 70);
    assert!(count_1 > 30 && count_1 < 70);
}