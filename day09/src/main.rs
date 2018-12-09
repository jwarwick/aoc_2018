extern crate day09;

fn main() {
    // 416 players; last marble is worth 71975
    let result1 = day09::high_score(416, 71975);
    println!("Part 1 Result: {}", result1);

    // 100x more marbles
    let result2 = day09::high_score(416, 71975 * 100);
    println!("Part 2 Result: {}", result2);
}
