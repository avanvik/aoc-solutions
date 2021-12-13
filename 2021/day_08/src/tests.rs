use super::*;

#[test]
fn it_converts_numbers() {
    let numb_8 = str_to_num("gecdbfa").expect("Not recognized number");
    assert_eq!(numb_8, 8);
}
