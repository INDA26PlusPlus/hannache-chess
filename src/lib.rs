// mod module;

// use module::whatever;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use std::{collections::binary_heap, io, thread::current, vec};

type BitboardType = u64; //this is like putting th variable Bitboard as type u64, increases readability


#[derive(Clone, Copy, PartialEq, Eq)] //means i can clone it , copy it, check equality with soemthing else.
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum ColorType { //can acess all White pieces at once, and code runs faster than if i have a string called White or Black in struct.
    White,
    Black
}

#[derive(Clone, Copy)] //can copy the värden, smt with owenership
struct BoardRepresentation {  // a struct is a collection of data types
    piece: PieceType, //an index for which board it is
    color: ColorType, //I can use the enum types in the board representation
    position: BitboardType,//how about one board for position and one for all the attacked spots
    attack: BitboardType //So here you will show which place the piece is able to attack 
    //Ig you can use the same board for all the rooks
}

struct MergedBoards {
    white_pos: BitboardType,
    black_pos: BitboardType,
    white_attack: BitboardType,
    black_attack: BitboardType,
    occupied: BitboardType,
    not_occupied: BitboardType
}


fn initialize_pieces() -> Vec<BoardRepresentation>{ //piece, color, position, attack. returns a vector
    //Exaple of instance of this class
    let mut white_pawn = BoardRepresentation{
        piece: PieceType::Pawn,
        color: ColorType::White,
        position: 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000,
    };

    let mut white_bishop = BoardRepresentation{ //ok so like i dont want to change piece nd color
        piece: PieceType::Bishop,
        color: ColorType::White,
        position: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    };

    let mut white_knight = BoardRepresentation{
        piece: PieceType::Knight,
        color: ColorType::White,
        position: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    };
    ////Heres the line PLAN FOR TMR: Fix these functions down here by chanching positition and attack and make the black ones too
    let mut white_rook = BoardRepresentation{
        piece: PieceType::Rook,
        position: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::White
    }; 

    let mut white_queen = BoardRepresentation{
        piece: PieceType::Queen,
        position: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::White
    } ;

    let mut white_king = BoardRepresentation{
        piece: PieceType::King,
        position: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::White
    } ;

    //To black

    let mut black_pawn = BoardRepresentation{
        piece: PieceType::Pawn,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000,
    };

    let mut black_bishop = BoardRepresentation{ //ok so like i dont want to change piece nd color
        piece: PieceType::Bishop,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    };

    let mut black_knight = BoardRepresentation{
        piece: PieceType::Knight,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
    } ;

    let mut black_rook = BoardRepresentation{
        piece: PieceType::Rook,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::Black
    } ;

    let mut black_queen = BoardRepresentation{
        piece: PieceType::Queen,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::Black
    } ;

    let mut black_king = BoardRepresentation{
        piece: PieceType::King,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000, //queen meet queen and king meet king
        attack: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        color: ColorType::Black
    } ;
    let chess_vector = vec![white_pawn, white_bishop, white_knight, white_rook, white_queen, white_king, black_pawn, black_bishop, black_knight, black_rook, black_queen, black_king];
    return chess_vector;
    //Returns array with all the pieces
}

fn merging_boards(all_pieces: &Vec<BoardRepresentation>) -> Vec<BitboardType>{
    let mut all_white_position:BitboardType = all_pieces[0].position;
    for i in 1..6 {
        all_white_position = all_white_position|all_pieces[i].position;
    }

    let mut all_white_attack:BitboardType = all_pieces[0].attack;
    for i in 1..6 {
        all_white_attack = all_white_attack|all_pieces[i].attack;
    }


    let mut all_black_position:BitboardType = all_pieces[6].position;
    for i in 6..12{
        all_black_position = all_black_position|all_pieces[i].position;
    }

    let mut all_black_attack:BitboardType = all_pieces[6].attack;
    for i in 6..12{
        all_black_attack = all_black_attack|all_pieces[i].attack;
    }
    
    let mut all_occupied:BitboardType = all_black_position|all_white_position;
    let mut not_occupied:BitboardType = !all_occupied; //Not occupied.

    let mut merged_boards = vec![all_white_position, all_white_attack, all_black_position, all_black_attack, all_occupied, not_occupied];
    return merged_boards;
}

fn taking_input() -> [u64; 2] { //takes in like h5 returns (x, y) coordinates

    loop {
    //such as h5, taking the input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let mut input = input.trim().chars();
    // let a = &input;
    if input.clone().count() != 2{ //this uses a clone of input rather than actual input so eats up input.clone()
        //try again
        continue
    }
    
    let allowed_a = "abcdefgh";
    let allowed_b = "12345678";

    let a = input.next();
    let b = input.next();
    let mut coords: [u64; 2] = [0, 0]; //this is an array, always size 2

    if let (Some(x), Some(y)) = (a, b) { //Unwrapping
        if !allowed_a.contains(x) || !allowed_b.contains(y) {
            continue
        } else {
            match x {
                'a' => {
                    coords[0] = 0
                }
                'b' => {
                    coords[0] = 1
                }
                'c' => {
                    coords[0] = 2
                }
                'd' => {
                    coords[0] = 3
                }
                'e' => {
                    coords[0] = 4
                }
                'f' => {
                    coords[0] = 5
                }
                'g' => {
                    coords[0] = 6
                }
                'h' => {
                    coords[0] = 7
                }
                _ => {
                    unreachable!()
                }
            }
            match y {
                '1' => {
                    coords[1] = 0
                }
                '2' => {
                    coords[1] = 1
                }
                '3' => {
                    coords[1] = 2
                }
                '4' => {
                    coords[1] = 3
                }
                '5' => {
                    coords[1] = 4
                }
                '6' => {
                    coords[1] = 5
                }
                '7' => {
                    coords[1] = 6
                }
                '8' => {
                    coords[1] = 7
                }
                _ => {
                    unreachable!()
                }
            }
        }
    } else{continue}

    return coords;
    }
}

fn input_coordinates(column: BitboardType, row: BitboardType) -> BitboardType { //inputs coordinates and makes bitboard.
    let bitboard_index: BitboardType = (row+1) * 8 - (column+1); // this is so that origo is at bottom left 
    //now row and column are "dead"?
    let bitboard_chosen: BitboardType = 0b1<<bitboard_index;

    return bitboard_chosen; //-> type points at what output looks like
    // let mut column: String = String::new();
    // io::stdin().read_line(&mut column).expect("Failed to read");
    //rather than taking in inputs take in parameters instead.
}

fn moving_piece(initial_board:BitboardType, old_coords:[u64; 2], new_coords:[u64; 2]) -> BitboardType{ //changes the bitboard so that it has the new and removes old pos
    let old_boards = 1u64 << ((old_coords[1]+1)*8 - (old_coords[0]+1));
    let new_boards = 1u64 << ((new_coords[1]+1)*8 - (new_coords[0]+1));
    let new_board: BitboardType = (initial_board& !old_boards)|new_boards; //get rid of old and add the new
    return new_board;
}

fn update_attack(chosen:&BoardRepresentation,merged_boards:&Vec<u64>) -> BitboardType{ //takes in one chosen type and the merged boards then returns the new attack_board
    //updates one piece at a time
    let left_wall:BitboardType = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001; //gotta think hard. precomputing
    let right_wall:BitboardType = 0b00000001_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
    let up_wall:BitboardType = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
    let down_wall:BitboardType = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;

    let mut comerades = 0b0;
    let mut enemies = 0b0;
    if chosen.color == ColorType::White {
        comerades = merged_boards[0];
        enemies = merged_boards[2];
    } else {
        comerades = merged_boards[2];
        enemies = merged_boards[0];
    }

    let mut new_attack_board:BitboardType = 0b0;

    match chosen.piece{
        PieceType::Pawn => {
            if chosen.color == ColorType::White {

                // chosen.position 
                for square in 0..64{
                    let mut piece_bitboard = 1u64<<square;
                    if piece_bitboard & chosen.position == 0b0{
                        continue;
                    }
                
                    if square%8 != 0 {
                        new_attack_board = new_attack_board | (piece_bitboard<<9);
                    }
                    if (square+1)%8 != 0{
                        new_attack_board = new_attack_board | (piece_bitboard<<7);
                    }
                    new_attack_board = new_attack_board & !comerades; //everything thats on comerades gets erased.
                
                    //either looop through and get all the individual positions
                    //or take the binary thingie and 
                    }
            } else {
                for square in 0..64{
                    let mut piece_bitboard = 1u64<<square;
                    if piece_bitboard & chosen.position == 0b0{
                        continue;
                    }
                    if square%8 != 0 {
                        new_attack_board = new_attack_board | (piece_bitboard>>7);
                    }
                    if (square+1)%8 != 0{
                        new_attack_board = new_attack_board | (piece_bitboard>>9);
                    }
                    new_attack_board = new_attack_board & !comerades;   
                }
            }
            return new_attack_board;
        }

        PieceType::Bishop => {
            // let mut pos:u64 = 0;
            let order:[(i32, bool, u64); 4] = [
                (7, true, left_wall|up_wall),
                (9, true, right_wall|up_wall),
                (7, false, left_wall|down_wall),
                (9, false, right_wall|down_wall)
            ];

            for square in 0..64{
                let mut piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }

                for (steps, is_left, wall) in order{
                    for i in 0..7{
                        if piece_bitboard & wall != 0b0{
                            break
                        }

                        piece_bitboard = if is_left {piece_bitboard << steps} else {piece_bitboard >> steps};
                        if piece_bitboard & comerades != 0b0 {
                            break
                        }
                        if piece_bitboard & enemies != 0b0{
                            new_attack_board = new_attack_board | piece_bitboard;
                            break
                        }
                        new_attack_board = new_attack_board | piece_bitboard;
                    }
                }
            }
            return new_attack_board
        }

        PieceType::Knight => {
            //if up and up up has a thing you cant go
            //if side and side side, if down and down down.
            //doesnt matetr if its comerade or enemy
            //but check where you're going if theres comerade there. enemy doesnt matter at the palce you're landing
            
            // <<10, >>6, >>10, <<6, >>17, >>15, <<17, <<15

            for square in 0..64 {
                let mut piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }
            
                

                let mut step = 0;
                //right right
                // for i in 1..=2{
                loop {
                    if step == 2{ // x x x. 
                        //checck wall up and wall down
                        if piece_bitboard & up_wall == 0b0{
                            //can go up
                            new_attack_board = new_attack_board | (piece_bitboard<<8)
                        }
                        if piece_bitboard & down_wall == 0b0{
                            //can go down
                            new_attack_board = new_attack_board | (piece_bitboard>>8)
                        }
                    }
                    if piece_bitboard & right_wall != 0b0 { //if zero or first step is on the wall
                        break;
                    }
                    piece_bitboard = piece_bitboard >> 1; //moving to the right by one step
                    step += 1;

                }
                //check wall
                for square in 0..64{
                let mut piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }
                

                step = 0;
                //left left
                // >>1
                loop {
                    if step == 2{ // x x x. 
                        //checck wall up and wall down
                        if piece_bitboard & up_wall == 0b0{
                            //can go up
                            new_attack_board = new_attack_board | (piece_bitboard<<8)
                        }
                        if piece_bitboard & down_wall == 0b0{
                            //can go down
                            new_attack_board = new_attack_board | (piece_bitboard>>8)
                        }
                    }
                    if piece_bitboard & left_wall != 0b0 { //if zero or first step is on the wall
                        break;
                    }
                    piece_bitboard = piece_bitboard << 1; //moving to the left by one step
                    step += 1;

                }

                step = 0;
                // up up
                // << 8
                loop {
                    if step == 2{ // x x x. 
                        //checck wall up and wall down
                        if piece_bitboard & right_wall == 0b0{
                            //can go up
                            new_attack_board = new_attack_board | (piece_bitboard<<1)
                        }
                        if piece_bitboard & left_wall == 0b0{
                            //can go down
                            new_attack_board = new_attack_board | (piece_bitboard>>1)
                        }
                    }
                    if piece_bitboard & up_wall != 0b0 { //if zero or first step is on the wall
                        break;
                    }
                    piece_bitboard = piece_bitboard << 8; //moving to the up by one step
                    step += 1;

                }

                step = 0;
                // down down
                // << 8
                loop {
                    if step == 2{ // x x x. 
                        //checck wall up and wall down
                        if piece_bitboard & right_wall == 0b0{
                            //can go up
                            new_attack_board = new_attack_board | (piece_bitboard<<1)
                        }
                        if piece_bitboard & left_wall == 0b0{
                            //can go down
                            new_attack_board = new_attack_board | (piece_bitboard>>1)
                        }
                    }
                    if piece_bitboard & down_wall != 0b0 { //if zero or first step is on the wall
                        break;
                    }
                    piece_bitboard = piece_bitboard >> 8; //moving to the up by one step
                    step += 1;
                }
            }}

            return new_attack_board
            }

        PieceType::Rook => {
            let order = [
                (1, false, left_wall), //minskar
                (1, true, right_wall), //ökar
                (8, false, down_wall),
                (8, true, up_wall)
            ];
            //step, förminskar/is_left, wall
            for square in 0..64{
                let mut piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }

                for (step, is_left, wall) in order {
                    for i in 0..7{
                        if piece_bitboard & wall != 0b0 {
                            continue //crash into the wall
                        }
                        piece_bitboard = if is_left {piece_bitboard << step} else {piece_bitboard >> step};
                        if piece_bitboard & comerades != 0b0 {
                            break
                        }
                        if piece_bitboard & enemies != 0b0{
                            new_attack_board = new_attack_board | piece_bitboard;
                            break
                        }
                        new_attack_board = new_attack_board | piece_bitboard;
                    }

                    }
                }
            return new_attack_board;
            //up down side to side
            //check all walls

        }

        PieceType::Queen => {
            //rook + bishop
            let order = [
                //rook
                (1, false, left_wall),
                (1, true, right_wall),
                (8, false, down_wall),
                (8, true, up_wall),
                //bishop:
                (7, true, left_wall|up_wall),
                (9, true, right_wall|up_wall),
                (7, false, left_wall|down_wall),
                (9, false, right_wall|down_wall)

            ];
            for square in 0..64{
                let mut piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }

                for (step, is_left, wall) in order {
                    for i in 0..7{
                        if piece_bitboard & wall != 0b0 {
                            continue //crash into the wall
                        }
                        piece_bitboard = if is_left {piece_bitboard << step} else {piece_bitboard >> step};
                        if piece_bitboard & comerades != 0b0 {
                            break
                        }
                        if piece_bitboard & enemies != 0b0{
                            new_attack_board = new_attack_board | piece_bitboard;
                            break
                        }
                        new_attack_board = new_attack_board | piece_bitboard;
                    }

                    }
                }
            return new_attack_board;
        }
        
        PieceType::King => {
            //up down side and side check all walls
            
            let mut piece_bitboard = chosen.position;
            //<1, >1, <8, >8, <7, >7, <9, >9
            //up left, up, up right, right, right down, down, down left, left
            let order = [
                (7, true, up_wall&left_wall),
                (8, true, up_wall),
                (9, true, up_wall&right_wall),
                (1, true, right_wall),
                (7, false, down_wall&right_wall), //minskar
                (8, false, down_wall),
                (9, false, down_wall&left_wall),
                (1, false, left_wall)
            ];
            for (step, is_left, wall) in order{

                if piece_bitboard & wall == 0b0{
                    new_attack_board = if is_left {new_attack_board | piece_bitboard<<step} else {new_attack_board | piece_bitboard>>step};
                }
            }
            return new_attack_board
        }
            //Make a board with all the possible moves then & it with !wall
    }
}

fn pawn_upgrades(){
    //change the struct, take away one pawn and add one of the piece you chose
}

fn checking_check(attack_board:BitboardType, king:BoardRepresentation){
    //if any of the merged attacks overlap with king
    if attack_board & king.position != 0b0 {

    }
}

fn check_checkmate(){
    //this should be like checking new board

}

fn check_stalemate(){

}

fn check_pawn_upgrades(){
    //honestly enough to check those who have moved
}

fn print_board(bitboard:BitboardType) {
    println!("*-------------------*");
    for row in (0..8).rev(){
        print!("{} | ", row+1);
        for column in 0..8{
            let square = row*8+column;
            let thing_there = (bitboard >> square) & 0b1;
            if thing_there != 0{
                print!("1 ");
            } else {
                print!(". ");
            }
        }
        println!("|");
    }
    println!("*-------------------*")
}

fn filter_noneplayable_squares(start_bitboard:BitboardType, merged_boards:&Vec<u64>, turn:ColorType) -> bool{
    if start_bitboard&merged_boards[4] != 0b0{
        //position is not occupied
        return false;
    }
    if turn == ColorType::White{ //I'm sorry this is burning my eyes.
        if merged_boards[0] & start_bitboard == 0b0{ //you chose a black piece
            return false;
        }
    } else { //its blacks turn
        if merged_boards[2] & start_bitboard == 0b0{ //You chose white piece
            return false;
        }
    }
}

fn all(column: u64, row: u64, checking: Vec<BoardRepresentation>){
        
    //initializes all the pieces and merged boards and intializes the position of each merged 
    let all_pieces = initialize_pieces(); //within this generate the new attack board
    let merged_boards = merging_boards(&all_pieces);
    let i_not_occupied = 5; let i_occupied = 4; let i_black_atk = 3; let i_black_pos = 2; let i_white_atk = 1; let i_white_pos = 0;
    let turn = ColorType::White;

    loop {
        let start_coords = taking_input();
        let to_coords = taking_input();

        //creates a bitboard of the input coordiantes
        let start_bitboard: BitboardType = input_coordinates(start_coords[0], start_coords[1]);
        let to_bitboard: BitboardType = input_coordinates(to_coords[0], to_coords[1]);


        //FILTER: if you chose nothing or chose wrong color or to somewhere you can't go then stop.
        if start_bitboard&merged_boards[i_occupied] != 0b0{
            //position is not occupied
            continue
        }
        if turn == ColorType::White{ //I'm sorry this is burning my eyes.
            if merged_boards[i_white_pos] & start_bitboard == 0b0{ //you chose a black piece
                continue;
            }
        } else { //its blacks turn
            if merged_boards[i_black_pos] & start_bitboard == 0b0{ //You chose white piece
                continue;
            }
        }
        //TO DO: If you chose a spot where you can't go filter away too 
        //just pawn has a seperate can_walk_to, others can_walk_to = attack.


        //identify the piece that was chosen using all_pieces
        //0..6 are the white 6..12 are the black.

        let mut initial_board:u64 = 0; 
        let mut current_piece_i:usize = 0;
        if turn == ColorType::White{
        for i in 0..6 { //now this has to become something otherwise its unsafe
            if start_bitboard&all_pieces[i].position != 0b0{
                //you found it!
                current_piece_i = i;
                initial_board = all_pieces[i].position;
                break;
            }
        }} else{
        for i in 6..12 { //now this has to become something otherwise its unsafe
            if start_bitboard&all_pieces[i].position != 0b0{
                //you found it!
                current_piece_i = i;
                initial_board = all_pieces[i].position;
                break;
            }
        }}

        //check for pawn upgrades.
        if all_pieces[current_piece_i].piece == PieceType::Pawn && (to_coords[1] == 0 || to_coords[1] == 7){
            pawn_upgrades(all_pieces[current_piece_i])
        }


        //-------temporarily move it to check checkmate---------------------------------------.

        let temporary_board = moving_piece(initial_board, start_coords, to_coords);
        //this is after the move
        let current_piece_type:PieceType = all_pieces[current_piece_i].piece;
        let current_piece_color:ColorType = all_pieces[current_piece_i].color;

        let mut temp_current: BoardRepresentation = BoardRepresentation{
            piece: (all_pieces[current_piece_i].piece),
            color: all_pieces[current_piece_i].color,
            position: temporary_board,
            attack: 0b0
        };
        //recompute all the attack pos both white and black. except the piece i am right now current_piece_i ----

        //create a list for all the pieces i will be going through
        let mut update_attack_list:Vec<BoardRepresentation> = Vec::with_capacity(6);
        if turn == ColorType::White{
            for i in 0..6 {
                if all_pieces[i].piece != current_piece_type {
                    update_attack_list[i] = all_pieces[i];
                    continue
                }
                update_attack_list[i] = temp_current
            }
        }


        let temp_merged_boards = merged_boards; //I hope this copies the merged boards
        //merge the piecess positions (ex, white_merged_pos)
        let mut temporary_new_merged:BitboardType = 0b0;
        for i in 0..6{
            if i == current_piece_i{
                temporary_new_merged = temporary_new_merged | temporary_board;
                continue;
            }
            temporary_new_merged = temporary_new_merged | all_pieces[i].position;
        }
        temp_merged_boards[0] = temporary_new_merged; //the white_pos position

        temporary_new_merged = 0b0;
        for i in 6..12 { 
            if i == current_piece_i{
                temporary_new_merged = temporary_new_merged | temporary_board;
                continue
            }
            temporary_new_merged = temporary_new_merged | all_pieces[i].position;
            }
        //update the attack for all the positions. 
        


        //check if it leads to check or not for yourself.
        //if check dont allow -> try again
        //if not check -> continue

        //------------


        //I'm so confused what is this doing?
        let mut theres_a_piece:bool = false;
        for i in 0..12 {
            if &start_bitboard&checking[i].position != 0b0 { //
                let mut chosen =  BoardRepresentation{
                    position: start_bitboard,
                    piece: checking[i].piece,
                    color: checking[i].color,
                    attack: 0b0
                };
                theres_a_piece = true;
                break;
            }
        }
        //else: theres no piece on the spot you've chosen.
        // if made_a_struct == false {
        //     //you havent made a struct


    //     }
}}


#[test]
fn testiing_bin_subtraction() {
    let x: BitboardType= 0b000111;
    let y:BitboardType = 0b000101;
    let k: u64 = 5; //can subtract int from bin
    let z = x-y;//can subtract bin and bin
    let l = x-k;
    println!("{:b}",z);
    println!("{:b}", l);

}

#[test]
fn mai() {//if its main i can't run it T^T
    let white_pieces: BitboardType = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000; //Underscore is same as without udnerscore it just makes it more readable
    //white_pieces[row*8 + column]
    println!("Hii");
    println!("{:b}", white_pieces) //:b prints it out in binary, defult is decimals
}

#[test]
fn test_trim() {
  // create some strings
  let string1 = " Welcome to Edpresso    ";
  let string2 = "Educative is the best!  \n   ";
  let string3 = "     Rust is very interesting!";

  // trim the strings
  let trim1 = string1.trim();
  let trim2 = string2.trim();
  let trim3 = string3.trim();

  // print the trims
  println!("The string before trim is '{}' and length is {}", string1, string1.len());
  println!("The string when trimmed is '{}' and length is {}", trim1, trim1.len());

  println!("\nThe string before trim is '{}' and length is {}", string2, string2.len());
  println!("The string when trimmed is '{}' and length is {}", trim2, trim2.len());

  println!("\nThe string before trim is '{}' and length is {}", string3, string3.len());
  println!("The string when trimmed is '{}' and length is {}", trim3, trim3.len());

}


#[test]
fn test_bin_shifting(){
    let column = 0;
    let row = 3;

    let position = row*8 + column;

    println!("{:}", 0b000100); //in decimal form
    println!("{:b}", 0b000100); //in binary form
    let a:u64 = 0b11;
    let b:u64 = 0b10011000; //this is the same as doing 00000011 ^ 11111100
    let mut c:u64 = a & b;
    let d:u64 = &c<<10;
    println!("{:b}", c);
    println!("{:b}", d);

}