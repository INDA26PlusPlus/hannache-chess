
use std::io;

type BitboardType = u64; //this is like putting th variable Bitboard as type u64, increases readability
type AllPieces = Vec<BoardRepresentation>;

//enums and structs----------

#[derive(Clone, Copy, PartialEq, Eq)] //means i can clone it , copy it, check equality with soemthing else.
pub enum PieceType { //can acess all of a certain piece, it's like a filter (my unified type)
    //A piece can only be one of these few types
    //everything in here is calle a variant, use :: toa cess variant
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorType { //can acess all White pieces at once, and code runs faster than if i have a string called White or Black in struct.
    White,
    Black,
}

impl ColorType {
    //like a method for just this enum
    pub fn opposite(self) -> Self { //when you want to switch sides.
        match self {
            ColorType::White => ColorType::Black,
            ColorType::Black => ColorType::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)] //can copy the värden, smt with owenership
pub struct BoardRepresentation {  // a struct is a collection of data types
    piece: PieceType, //an index for which board it is
    color: ColorType, //I can use the enum types in the board representation
    position: BitboardType,//how about one board for position and one for all the attacked spots
    attack: BitboardType //So here you will show which place the piece is able to attack 
    //Ig you can use the same board for all the rooks
}

//index for merged_boards
pub const I_WHITE_POS:usize = 0;
pub const I_WHITE_ATK:usize = 1; 
pub const I_BLACK_POS:usize = 2; 
pub const I_BLACK_ATK:usize = 3; 
pub const I_OCCUPIED:usize = 4; 
pub const I_NOT_OCCUPIED:usize = 5; 

#[derive(Clone)]
pub struct Board { // a way to keep the all_pieces, merged_boards and turn acessible in many functions
    pub all_pieces: AllPieces,
    pub merged_boards:Vec<BitboardType>,
    pub turn:ColorType,
}

impl Board { //in terms of python think of impl as where everything but the __init__ is
    pub fn new() -> Self{
        let all_pieces = initialize_pieces();
        let merged_boards = merging_boards(&all_pieces);
        
        let mut board = Self { //this is initializing self
            all_pieces,
            merged_boards,
            turn: ColorType::White,
        };
        board.recalculate_attacks(); //Gotta calculate the attakcs first because they are empty..
        return board;
    }

    pub fn recalculate_attacks(&mut self) {
        self.merged_boards = merging_boards(&self.all_pieces); //needs a new merged boards
        for i in 0..self.all_pieces.len() {
            let piece = self.all_pieces[i];
            let atk = update_attack(&piece, &self.merged_boards);
            self.all_pieces[i].attack = atk; //Now i have to make sure this isnt permantne change for when i am testing positions
        }
        self.merged_boards = merging_boards(&self.all_pieces) //Now that i have updated them
    }
    
    pub fn making_move(&mut self, start_coords:[u64; 2], to_coords:[u64; 2]) -> bool{ //if it didnt work -> false, if it worked -> true
        let start_bitboard: BitboardType = input_coordinates(start_coords[0], start_coords[1]);
        let to_bitboard: BitboardType = input_coordinates(to_coords[0], to_coords[1]);

        let (_, current_piece_i) = identify_chosen_piece(board.clone(), start_bitboard);

        if filter_noneplayable_squares(start_bitboard, to_bitboard, &self) == false {
            return false;
        }

        //finished the filter

        //testing on cloned board to see checkmate -------------- WIP
        let mut temp_board = self.clone(); //now this has all the traits of board but when i modify it it isnt borad.

        //do the move on the temp board
        temp_board.all_pieces[current_piece_i].position = moving_piece(temp_board.all_pieces[current_piece_i].position, start_coords, to_coords);
        
        //change the position
        temp_board.all_pieces[current_piece_i].position = moving_piece(self.all_pieces[current_piece_i].position.clone(), start_coords, to_coords);
        //update attack
        temp_board.recalculate_attacks();


        if self.checking_check() {
            check_checkmate(self.clone());
        }

        self.all_pieces = temp_board.all_pieces; //does this change all of self.all_pieces?

        //means you dont put your king in danger.



        //check for pawn upgrade
        if self.all_pieces[current_piece_i].piece == PieceType::Pawn && (to_coords[1] == 0 || to_coords[1] == 7){
            pawn_upgrades(self.all_pieces[current_piece_i]);
        }

        //check if the game has ended

        return true
    }
}

//-----------


//Initializing pieces and merging and uhh pawn legal moves ---------

fn initialize_pieces() -> Vec<BoardRepresentation> {
    let white_pawn = BoardRepresentation {
        piece: PieceType::Pawn,
        color: ColorType::White,
        position: 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let white_bishop = BoardRepresentation {
        piece: PieceType::Bishop,
        color: ColorType::White,
        position: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let white_knight = BoardRepresentation {
        piece: PieceType::Knight,
        color: ColorType::White,
        position: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let white_rook = BoardRepresentation {
        piece: PieceType::Rook,
        color: ColorType::White,
        position: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let white_queen = BoardRepresentation {
        piece: PieceType::Queen,
        color: ColorType::White,
        position: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let white_king = BoardRepresentation {
        piece: PieceType::King,
        color: ColorType::White,
        position: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
 
    let black_pawn = BoardRepresentation {
        piece: PieceType::Pawn,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
        attack: 0,
    };
    let black_bishop = BoardRepresentation {
        piece: PieceType::Bishop,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
        attack: 0,
    };
    let black_knight = BoardRepresentation {
        piece: PieceType::Knight,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
        attack: 0,
    };
    let black_rook = BoardRepresentation {
        piece: PieceType::Rook,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
        attack: 0,
    };
    let black_queen = BoardRepresentation {
        piece: PieceType::Queen,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
        attack: 0,
    };
    let black_king = BoardRepresentation {
        piece: PieceType::King,
        color: ColorType::Black,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
        attack: 0,
    };
 
    vec![
        white_pawn, white_bishop, white_knight, white_rook, white_queen, white_king,
        black_pawn, black_bishop, black_knight, black_rook, black_queen, black_king,
    ]
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

pub fn pawn_legal_moves(pawn:&BoardRepresentation, merged_boards:&Vec<u64>) -> BitboardType{
    let up_wall:BitboardType = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
    let down_wall:BitboardType = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;

    let white_start:BitboardType = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
    let black_start: BitboardType = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000

    let mut legal_moves:BitboardType = 0b0;
    
    let mut comerades = 0b0;
    let mut enemies = 0b0;
    if pawn.color == ColorType::White {
        comerades = merged_boards[0];
        enemies = merged_boards[2];
    } else {
        comerades = merged_boards[2];
        enemies = merged_boards[0];
    }
    
    for square in 0..64{
        let mut piece_bitboard = 1u64<<square;
        if piece_bitboard & pawn.position == 0b0{
            continue;
        }

        //if you are on the beginning line you can jump two steps -> an passant blocks behind

        if pawn.color == ColorType::White{

            if piece_bitboard & white_start != 0b0 {
                //an extra board for an passant vunerable moves TODO
                legal_moves = legal_moves | (piece_bitboard << 16);
            }
            //if you are at the beginning
            if piece_bitboard & up_wall == 0b0{
                if (piece_bitboard << 8) & comerades == 0 && piece_bitboard << 8 & enemies == 0 {
                    //if theres nothing in front of you, you can move
                    legal_moves = legal_moves | (piece_bitboard << 8);
                } 
            }
        } else {
            if piece_bitboard & down_wall == 0b0{
                if (piece_bitboard >> 8) & comerades == 0 && piece_bitboard >> 8 & enemies == 0 {
                    legal_moves = legal_moves | (piece_bitboard >> 8);
                } 
            }
        }
    }


        return legal_moves
}


//-------

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

fn pawn_upgrades(current:BoardRepresentation){
    //change the struct, take away one pawn and add one of the piece you chose
}

// fn big_filter(board: Board, start_bitboard:BitboardType, to_bitboard:BitboardType) -> bool {
    

//     if filter_noneplayable_squares(start_bitboard, &board) == false {
//         return false;
//     }

//     let (_, current_piece_i) = identify_chosen_piece(board.clone(), start_bitboard);

//     let current_piece = board.all_pieces[current_piece_i];
//     let legal_moves:BitboardType = if current_piece.piece == PieceType::Pawn{//Because pawns move differently
//         pawn_legal_moves(&current_piece, &board.merged_boards)
//     } else {
//         current_piece.attack
//     };

//     if (legal_moves & to_bitboard) == 0b0 {
//         return false;
//     }
//     return true;

// }

fn check_checkmate(board: Board) -> bool{
    if board.checking_check() == false{
        return false
    }

    //if 0 legal moves -> checkmate
    //check if anything will lead to not check
    //no way i have to do every single move right...?
    return true;

}
fn checking_check(board: Board) -> bool {
    if board.turn == ColorType::White{ // 5 = white king
        if board.all_pieces[5].position & board.merged_boards[I_BLACK_ATK] != 0{
            return true;
        } else {
            return false;
        }
    } else {
        if board.all_pieces[11].position & board.merged_boards[I_WHITE_ATK] != 0{
            return true;
        } else {
            return false;
        }
    }
}

fn generate_legal_moves(board: Board) -> Vec<BitboardType>{
    //Give it a clone!!
    //if its whites turn you check if its checkmate for white

    let mut legal_moves:Vec<BitboardType> = vec![];
    //check per color ever piece and every possible move.

    if board.turn == ColorType::White {
        for i in 6..12 { //white_king = 5, black_king = 11
            //now check each available move for each pieces
            if board.all_pieces[i].piece == PieceType::Pawn {
                //for i in range of all the pawns that exist....
                let potentially_legal_move = pawn_legal_moves(&board.all_pieces[i], &board.merged_boards);
                //this is per piece. 
                if board.all_pieces[i].position & 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000 != 0b0{
                    board.all_pieces[i].position = board.all_pieces[i].position<<8;
                } else {

                }
            }
        }
    } else {
        for i in 0..6 {

        }

    }

    return legal_moves;
    
    //it passes the filter

    //If its in check
}

fn filter_noneplayable_squares(start_bitboard:BitboardType, to_bitboard:BitboardType, board:&Board) -> bool{
    if start_bitboard&board.merged_boards[4] != 0b0{
        //position is not occupied
        return false;
    }
    if board.turn == ColorType::White{ //I'm sorry this is burning my eyes.
        if board.merged_boards[0] & start_bitboard == 0b0{ //you chose a black piece
            return false;
        }        
    } else { //its blacks turn
        if board.merged_boards[2] & start_bitboard == 0b0{ //You chose white piece
            return false;
        } 
    }
    //ok this is kinda slow, i'll optimise if i have time
    let (_, current_piece_i) = identify_chosen_piece(board.clone(), start_bitboard);

    let current_piece = board.all_pieces[current_piece_i];
    let legal_moves:BitboardType = if current_piece.piece == PieceType::Pawn{//Because pawns move differently
        pawn_legal_moves(&current_piece, &board.merged_boards)
    } else {
        current_piece.attack
    };

    if (legal_moves & to_bitboard) == 0b0 {
        return false;
    }

    return true //if it passed filter then return true
}

fn identify_chosen_piece(board:Board, chosen_bitboard:BitboardType) -> (u64, usize){
    let mut initial_board:u64 = 0; 
    let mut current_piece_i:usize = 0;
    if board.turn == ColorType::White{
        for i in 0..6 { //now this has to become something otherwise its unsafe
            if chosen_bitboard&board.all_pieces[i].position != 0b0{
                //you found it!
                current_piece_i = i;
                initial_board = board.all_pieces[i].position;
                break;
            }
    }} else {
        for i in 6..12 { //now this has to become something otherwise its unsafe
            if chosen_bitboard&board.all_pieces[i].position != 0b0{
                //you found it!
                current_piece_i = i;
                initial_board = board.all_pieces[i].position;
                break;
            }
        }
    }
    return (initial_board, current_piece_i);

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

// fn temporary_check_charckmate(all_pieces: &Vec<BoardRepresentation>, 
//     initial_board:BitboardType, 
//     start_coords:[u64; 2], to_coords:[u64; 2],){
    
// }

fn print_possible_moves(){}
//all_pieces is public somehow
fn print_piece_position(piece:PieceType){}

fn all(column: u64, row: u64, checking: Vec<BoardRepresentation>){
        
    //initializes all the pieces and merged boards and intializes the position of each merged 
    // let all_pieces = initialize_pieces(); //within this generate the new attack board
    // let merged_boards: Vec<u64> = merging_boards(&all_pieces);
    // let turn = ColorType::White;
    let board = Board::new();

    loop {
        let start_coords = taking_input();
        let to_coords = taking_input();


        //---choosing character---
        //creates a bitboard of the input coordiantes -- kinda like chosing character.
        let start_bitboard: BitboardType = input_coordinates(start_coords[0], start_coords[1]);
        let to_bitboard: BitboardType = input_coordinates(to_coords[0], to_coords[1]);


        //FILTER: if you chose nothing or chose wrong color or to somewhere you can't go then stop.
        if filter_noneplayable_squares(start_bitboard, &board) == false {
            continue
        }
        //TO DO: If you chose a spot where you can't go filter away too 
        //just pawn has a seperate can_walk_to, others can_walk_to = attack.

        //-----------------------



        //Who am i---------------------
        let (initial_board, current_piece_i) = identify_chosen_piece(board.clone(), start_bitboard);
        //------------------------//

        //check for pawn upgrades. should check at end of moving honestly
        if board.all_pieces[current_piece_i].piece == PieceType::Pawn && (to_coords[1] == 0 || to_coords[1] == 7){
            pawn_upgrades(board.all_pieces[current_piece_i]);
        }

        //----------trying to move---------------------------------####

        //-------temporarily move it to check checkmate----------

        let mut temp_current: BoardRepresentation = BoardRepresentation{
            piece: (board.all_pieces[current_piece_i].piece),
            color: board.all_pieces[current_piece_i].color,
            position: moving_piece(initial_board, start_coords, to_coords),
            attack: 0b0
        };
        //recompute all the attack pos both white and black. except the piece i am right now current_piece_i ----

        //create a list for all the pieces i will be going through
        let mut update_attack_list:Vec<BoardRepresentation> = Vec::with_capacity(6);
        if board.turn == ColorType::White{
            for i in 0..6 {
                if board.all_pieces[i].piece != temp_current.piece {
                    update_attack_list[i] = board.all_pieces[i];
                    continue
                }
                update_attack_list[i] = temp_current
            }
        }


        let mut temp_merged_boards = board.merged_boards.clone(); //I hope this copies the merged boards
        
        //merge the piecess positions (ex, white_merged_pos)
        let mut temporary_new_merged:BitboardType = 0b0;
        
        //adds all
        for i in 0..6{
            if i == current_piece_i{
                temporary_new_merged = temporary_new_merged | temp_current.position;
                continue;
            }
            temporary_new_merged = temporary_new_merged | board.all_pieces[i].position;
        }
        temp_merged_boards[0] = temporary_new_merged; //the white_pos position

        temporary_new_merged = 0b0;
        for i in 6..12 { 
            if i == current_piece_i{
                temporary_new_merged = temporary_new_merged | temp_current.position;
                continue
            }
            temporary_new_merged = temporary_new_merged | board.all_pieces[i].position;
            }
        //update the attack for all the positions. 
        


        //check if it leads to check or not for yourself.
        //if check dont allow -> try again
        //if not check -> continue

        //------------
        //Trying to move________________________

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

//Testing---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


//general plan:
//have some functions:
/* 
 - chose character
 - print_all_possile_moves and which character am i?
 - try making a move <- this one will probably be the most logical and most code

 - is check?
 - is over?
    - checkmate, stalemate.
 */