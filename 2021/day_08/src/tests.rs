use super::*;

#[test]
fn it_converts_numbers() {
    let numb_8 = str_to_num("gecdbfa").expect("Not recognized number");
    assert_eq!(numb_8, 8);
}

#[test]
fn it_decodes_with_map() {
    let inp = String::from("ebg");
    let mut map = HashMap::new();
    map.insert('e', 'a');
    map.insert('b', 'c');
    map.insert('g', 'f');

    let decoded = convert_str_with_map(&map, &inp);

    assert_eq!(decoded, "acf");
    assert_eq!(str_to_num(decoded.as_str()).unwrap(), 7);
}

#[test]
fn it_reads_number_from_string() {
    assert_eq!(str_to_num("abcefg").unwrap(), 0);
    assert_eq!(str_to_num("cf").unwrap(), 1);
    assert_eq!(str_to_num("acdeg").unwrap(), 2);
    assert_eq!(str_to_num("acdfg").unwrap(), 3);
    assert_eq!(str_to_num("bcdf").unwrap(), 4);
    assert_eq!(str_to_num("abdfg").unwrap(), 5);
    assert_eq!(str_to_num("abdefg").unwrap(), 6);
    assert_eq!(str_to_num("acf").unwrap(), 7);
    assert_eq!(str_to_num("abcdefg").unwrap(), 8);
    assert_eq!(str_to_num("abcdfg").unwrap(), 9);
}
