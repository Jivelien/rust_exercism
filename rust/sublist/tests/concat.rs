#[test]
fn test_concat() {
    let list_one: &[i32] = &[1, 2, 3];
    let output = sublist::concat_array(list_one);
    assert_eq!(output, String::from("123"));
}

