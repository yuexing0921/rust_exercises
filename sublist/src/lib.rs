// PartialEq https://rustwiki.org/zh-CN/std/cmp/trait.PartialEq.html
// 一种trait

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
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
