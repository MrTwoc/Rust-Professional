use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    // todo!()
    let input_str:Vec<&str> = input_str.split(',').collect();
    let mut count:HashSet<&str> = HashSet::new();
    for i in 0..input_str.len(){
        count.insert(input_str[i]);
    }
    return count.len();
}
