use std::io;
const winOptions:[[usize;3];8] = [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]];
fn main(){
    let mut gameField = ["0","1","2","3","4","5","6","7","8"];
    let mut turn = "X";
    while checkForWin(gameField)!=true {
        turn = switch(turn);
        printGameField(gameField, turn);
        gameField[mve(turn, gameField)] = turn;
        if checkForWin(gameField) == true {
            println!("{} won!", turn)
        }
    }
}
fn printGameField(gameField:[&str;9],turn:&str){
    println!("{} {} {}", gameField[0], gameField[1], gameField[2]);
    println!("{} {} {}", gameField[3], gameField[4], gameField[5]);
    println!("{} {} {}", gameField[6], gameField[7], gameField[8]);
    println!();
    println!("{} Turn", turn);
}
fn mve(turn:&str,gameField:[&str;9]) -> usize {
    println!("Please input your move");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error lmao");
    let input: usize = input.trim().parse().expect("Please type a number!");
    if gameField[input] == "X" || gameField[input] == "O" {
        panic!("The field is already taken.");
    }else{
        return input;
    }
}
fn switch(turn: &str) -> &str {
    return match turn {
        "O" => "X",
        "X" => "O",
        _ => panic!("Error!")
    }
}
fn checkForWin(gameField:[&str;9]) -> bool {
    for option in winOptions{
        if gameField[option[0]]==gameField[option[1]] && gameField[option[0]]==gameField[option[2]]{
            return true;
        }
    }
    false
}