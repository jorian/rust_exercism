pub fn build_proverb(list: Vec<&str>) -> String {
//    unimplemented!("build a proverb from this list of items: {:?}", list)
    if list.len() < 1 { return String::new(); }

    let mut result = list.windows(2)
        .map( |window| format!("For want of a {} the {} was lost.\n", window[0], window[1]))
        .collect::<Vec<String>>()
        .join("");

    result.push_str(&format!("And all for the want of a {}.", &list[0]));

    result
}