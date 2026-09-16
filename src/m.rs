//This is to test the thingie. Temporary.


//white to checkmate black

use std::io;

enum PieceType{
    Pawn, //p
    Knight, //k
    Bishop, //b
    Rook, //r
    Queen, //q
    King //k
}

enum ColorType{
    White,
    Black
}

struct Piece_characteristic{

}


fn main() {

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    //What does &mut input
    println!("{}", input);

    
}