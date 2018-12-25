extern crate util;
#[macro_use] extern crate scan_fmt;

//use std::str::FromStr;
//use std::collections::HashMap;

fn main() {
    let filename = util::get_argument("input.txt");
    let content = util::string_from_file(&filename);

    //let result1 = in_range(&content);
    //println!("Part 1 Result: {}", result1);

    //let result2 = dist_to_center(&content);
    //println!("Part 2 Result: {}", result2);
}


#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn radius_test() {
    //    let content = "pos=<0,0,0>, r=4"
    //    assert_eq!(in_range(&content), 7);
    //}

}
