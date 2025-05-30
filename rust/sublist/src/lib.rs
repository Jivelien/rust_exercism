use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


pub fn sublist2<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let superlist = second_list.is_empty()
        || first_list
        .windows(second_list.len())
        .any(|x| x == second_list);
    let sublist = first_list.is_empty()
        || second_list
        .windows(first_list.len())
        .any(|x| x == first_list);
    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }
}

pub fn sublist<T:Display>(first_list: &[T], second_list: &[T]) -> Comparison {
    let l1 = concat_array(first_list);
    let l2 = concat_array(second_list);

    if l1 == l2 {
        return Comparison::Equal;
    }
    if l1.contains(l2.as_str()) {
        return Comparison::Superlist;
    }
    if l2.contains(l1.as_str()) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}

fn concat_array<T: Display>(list: &[T]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let list_to_string: Vec<String> = list.iter().map(|v| v.to_string()).collect();
    const SEP: &str = ":";
    let list_to_string_with_separator = list_to_string.join(SEP);
    format!("{SEP}{list_to_string_with_separator}{SEP}")
}
