#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn equal_array(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let a = first_list.iter().map(|v| v.to_string());
    let mut z: String = String::new();

    for s in a {
        z = format!("{}{}",z, s);
    }
    

    if "hello".contains("ell") {
        Comparison::Equal
    } else {
        Comparison::Unequal
    }
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (first, second) => {
            if first == second {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }

    // match (first_list, second_list) {
    //     ( [] , [_])
    // }
}
