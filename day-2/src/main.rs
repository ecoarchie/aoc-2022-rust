
// A X rock 1
// B Y paper 2 
// C Z scissors 3
// lose 0
// draw 3
// win 6
fn main() {
    let input = "A Y
B X
C Z";
    let mut sum = 0;
    // part 1
    // for i in input.split("\n") {
    //         match i {
    //             "A X" => sum += 4,
    //             "A Y" => sum += 8,
    //             "A Z" => sum += 3,
    //             "B X" => sum += 1,
    //             "B Y" => sum += 5,
    //             "B Z" => sum += 9,
    //             "C X" => sum += 7,
    //             "C Y" => sum += 2,
    //             "C Z" => sum += 6,
    //             _ => sum += 0,
    //         }
    // }

    //part 2
    // X - need to loose
    // Y - need to draw
    // Z - need to win
    for i in input.split("\n") {
            match i {
                "A X" => sum += 3,
                "A Y" => sum += 4,
                "A Z" => sum += 8,
                "B X" => sum += 1,
                "B Y" => sum += 5,
                "B Z" => sum += 9,
                "C X" => sum += 2,
                "C Y" => sum += 6,
                "C Z" => sum += 7,
                _ => sum += 0,
            }
    }

    println!("{}", sum);
}
