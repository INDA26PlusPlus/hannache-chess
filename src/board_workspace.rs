use stdio

type BitboardType = u64; //this is like putting th variable Bitboard as type u64, increases readability



enum PieceType { //can acess all of a certain piece, it's like a filter (my unified type)
    //A piece can only be one of these few types
    //everything in here is calle a variant, use :: toa cess variant
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}

enum ColorType { //can acess all White pieces at once, and code runs faster than if i have a string called White or Black in struct.
    White,
    Black
}

struct BoardRepresentation() { // a struct is a collection of data types
    piece: PieceType, //an index for which board it is
    color: ColorType //I can use the enum types in the board representation
    position: BitboardType,//how about one board for position and one for all the attacked spots
    attack: BitboardType, //So here you will show which place the piece is able to attack 
    //Ig you can use the same board for all the rooks

}

// 6 2 9 6 5 1 0

fn initialize_pieces() {
    //Exaple of instance of this class
    let mut white_Pawn = BoardRepresentation{
        piece: PieceType::Pawn,
        color: ColorType::White,
        position: 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000,
    }

    let mut white_bishop = BoardRepresentation{ //ok so like i dont want to change piece nd color
        piece: PieceType::Bishop,
        color: ColorType::White,
        position: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    }

    let mut white_knight = BoardRepresentation{
        piece: PieceType::Knight,
        color: ColorType::White,
        position: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    } 
    ////Heres the line PLAN FOR TMR: Fix these functions down here by chanching positition and attack and make the black ones too
    let mut white_rook = BoardRepresentation{
        piece: PieceType::Rook,
        position: 0b1000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::White
    } 

    let mut white_queen = BoardRepresentation{
        piece: PieceType::Queen,
        position: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::White
    } 

    let mut white_king = BoardRepresentation{
        piece: PieceType::King,
        position: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::White
    } 

    //To black

    let mut white_Pawn = BoardRepresentation{
        piece: PieceType::Pawn,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000,
    }

    let mut white_bishop = BoardRepresentation{ //ok so like i dont want to change piece nd color
        piece: PieceType::Bishop,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    }

    let mut white_knight = BoardRepresentation{
        piece: PieceType::Knight,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    } 

    let mut white_rook = BoardRepresentation{
        piece: PieceType::Rook,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_1000001,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::Black
    } 

    let mut white_queen = BoardRepresentation{
        piece: PieceType::Queen,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::Black
    } 

    let mut white_king = BoardRepresentation{
        piece: PieceType::King,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000, //queen meet queen and king meet king
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::Black
    } 
}

//I can later check specific traits of this variable
if white_pawn.color == ColorType::White {
    //do something
}

//You chose a part of the board then math it will all the pieces. if it is that piece then 
//How do i modify bitboards

fn choosing() {

}
//For a bitboard like: 0b0011 AND 0b0010 -> 0b0 ? I dont really get it. 

//later check which type it is and based on the piece type it is you modify the attack position. match case is good for this
//check if somethign is in the way too


fn main() {
    let white_pieces: BitboardType = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000; //Underscore is same as without udnerscore it just makes it more readable
    //white_pieces[row*8 + column]
    println!("Hii");
    println!("{:b}", white_pieces) //:b prints it out in binary, defult is decimals
}

