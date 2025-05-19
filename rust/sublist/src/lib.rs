#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn equal_array(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let l1 = concat_array(first_list);
    let l2 = concat_array(second_list);

    if l1 == l2 {
        return Comparison::Equal;
    }
    Comparison::Unequal
}

fn foo(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let l1 = concat_array(first_list);
    let l2 = concat_array(second_list);
    
    
    if l1.contains(l2.as_str()) {
        return Comparison::Superlist
    }
    if l2.contains(l1.as_str()) {
        return Comparison::Sublist
    }
    Comparison::Unequal
}

// "125" 

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (first, second) => {
            if first == second {
                equal_array(first_list, second_list)
            } else {
                foo(first_list, second_list)
            }
        }
    }

}

fn concat_array(list: &[i32]) -> String {
    let list_to_string: Vec<String> = list.iter().map(|v| v.to_string()).collect();

    list_to_string.join("")
}
