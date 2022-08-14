mod adder;

use crate::adder::Adder;


#[test]
fn test_adder() {
    let mut adder = Adder::new();

    let n1: [i8; 5]     = [0, 1, 1, 0, 1];
    let n2: [i8; 5]     = [0, 1, 0, 1, 1];
    let mut result: [i8; 5] = [0; 5];


    for i in (0..5).rev() {
        
        result[i] = adder.add(n1[i], n2[i]);
        
    }

    

    assert_eq!(result, [1, 1, 0, 0, 0]);


}

