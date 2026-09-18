
use std::{future::pending, io};

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

    pub fn checking_check(&self) -> bool {
        let king_index = if self.turn == ColorType::White {5} else {11};
        let enemy_attack = if self.turn == ColorType::White {
            self.merged_boards[I_BLACK_ATK]
        } else {
            self.merged_boards[I_WHITE_ATK]
        };
        self.all_pieces[king_index].position & enemy_attack != 0 //True if in check
    }

    pub fn has_legal_move(&self) -> bool {
        //simulating on a cloned board: trying every move
        //if theres no legal moves + check -> checkmate
        let (start, end) = if self.turn == ColorType::White{(0, 6)} else {(6, 12)};

        for i in start..end { //for each piece
            let piece = self.all_pieces[i];
            if piece.position == 0 {
                continue; //theres no more of this type
            }

            for from_sq in 0..64 { //check each square
                let from_bit = 1u64 << from_sq;
                if piece.position & from_bit == 0 {
                    continue;
                }

                let targets = single_piece_moves(&piece, from_bit, &self.merged_boards);

                for to_sq in 0..64 {
                    let to_bit = 1u64 << to_sq;
                    if targets & to_bit == 0 {
                        continue;
                    }
                    
                    let mut temp = self.clone();
                    temp.all_pieces[i].position = (temp.all_pieces[i].position & !from_bit) | to_bit;
                    //everything thats not in from bit and add to bit
                    temp.recalculate_attacks();

                    if !temp.checking_check() {
                        return true; //you can end up in not check
                    }
                }

            }
        }
        return false;
    }

    pub fn is_checkmate(&self) -> bool {
        self.checking_check() && !self.has_legal_move() //its in check and can't move
    }

    pub fn is_stalemate(&self) -> bool {
        !self.checking_check() && !self.has_legal_move() //not in check but has no moves
    }
    
    pub fn making_move(&mut self, start_coords:[u64; 2], to_coords:[u64; 2]) -> bool{ //if it didnt work -> false, if it worked -> true
        let start_bitboard: BitboardType = input_coordinates(start_coords[0], start_coords[1]);
        let to_bitboard: BitboardType = input_coordinates(to_coords[0], to_coords[1]);

        
        if !filter_noneplayable_squares(start_bitboard, to_bitboard, self) {
            return false;
        }
        
        let (_, current_piece_i) = identify_chosen_piece(&self, start_bitboard);
        

        //testing on cloned board to see checkmate -------------- WIP
        let mut temp_board = self.clone(); //now this has all the traits of board but when i modify it it isnt borad.

        //do the move on the temp board
        temp_board.all_pieces[current_piece_i].position = moving_piece(temp_board.all_pieces[current_piece_i].position, start_coords, to_coords);

        remove_captured_piece(&mut temp_board.all_pieces, self.turn, to_bitboard);

        
        temp_board.recalculate_attacks();


        if temp_board.checking_check() {
            return false; //if you leave king in check not good
        }

        //committing to the move! //means you dont put your king in danger.
        self.all_pieces = temp_board.all_pieces; 
        self.merged_boards = temp_board.merged_boards;
        

        //check for pawn upgrade
        if self.all_pieces[current_piece_i].piece == PieceType::Pawn && (to_coords[1] == 0 || to_coords[1] == 7){ //if its at either wall
            pawn_upgrades(&mut self.all_pieces[current_piece_i]);
            self.recalculate_attacks();
        }

        self.turn = self.turn.opposite();

        if self.is_checkmate() {
            println!("Checkmate!")
        } else if self.is_stalemate() {
            println!("Stalemate!")
        }

        return true
    }
}

//Initializing pieces and merging and uhh pawn legal moves ---------

fn initialize_pieces() -> Vec<BoardRepresentation> {
    let black_pawn = BoardRepresentation {
        piece: PieceType::Pawn,
        color: ColorType::Black,
        position: 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let black_bishop = BoardRepresentation {
        piece: PieceType::Bishop,
        color: ColorType::Black,
        position: 0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let black_knight = BoardRepresentation {
        piece: PieceType::Knight,
        color: ColorType::Black,
        position: 0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let black_rook = BoardRepresentation {
        piece: PieceType::Rook,
        color: ColorType::Black,
        position: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let black_queen = BoardRepresentation {
        piece: PieceType::Queen,
        color: ColorType::Black,
        position: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
    let black_king = BoardRepresentation {
        piece: PieceType::King,
        color: ColorType::Black,
        position: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
        attack: 0,
    };
 
    let white_pawn = BoardRepresentation {
        piece: PieceType::Pawn,
        color: ColorType::White,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
        attack: 0,
    };
    let white_bishop = BoardRepresentation {
        piece: PieceType::Bishop,
        color: ColorType::White,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
        attack: 0,
    };
    let white_knight = BoardRepresentation {
        piece: PieceType::Knight,
        color: ColorType::White,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
        attack: 0,
    };
    let white_rook = BoardRepresentation {
        piece: PieceType::Rook,
        color: ColorType::White,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
        attack: 0,
    };
    let white_queen = BoardRepresentation {
        piece: PieceType::Queen,
        color: ColorType::White,
        position: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
        attack: 0,
    };
    let white_king = BoardRepresentation {
        piece: PieceType::King,
        color: ColorType::White,
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
    let mut all_white_attack:BitboardType = all_pieces[0].attack;

    for i in 0..6 {
        all_white_position = all_white_position|all_pieces[i].position;
        all_white_attack = all_white_attack|all_pieces[i].attack;
    }

    let mut all_black_position:BitboardType = all_pieces[6].position;
    let mut all_black_attack:BitboardType = all_pieces[6].attack;

    for i in 6..12{
        all_black_position = all_black_position|all_pieces[i].position;
        all_black_attack = all_black_attack|all_pieces[i].attack;
    }
    
    let all_occupied:BitboardType = all_black_position|all_white_position;
    let not_occupied:BitboardType = !all_occupied; //Not occupied.

    vec![
        all_white_position, 
        all_white_attack, 
        all_black_position, 
        all_black_attack, 
        all_occupied, 
        not_occupied
    ]
}



pub fn pawn_legal_moves(pawn:&BoardRepresentation, merged_boards:&Vec<u64>) -> BitboardType{

    let mut legal_moves:BitboardType = 0;
    let occupied = merged_boards[I_OCCUPIED];
    let enemies = if pawn.color == ColorType::White {
        merged_boards[I_BLACK_POS]
    } else {
        merged_boards[I_WHITE_POS]
    };

    let (start_row, forward): (usize, i32) = if pawn.color == ColorType::White {
        (1, 8)
    } else {
        (6, -8)
    }; //if you are on the start row on not

    for square in 0..64 {
        let piece_bitboard = 1u64 << square;
        if piece_bitboard & pawn.position == 0 {
            continue; // if its not pawn
        }

        let row = square/8;
        // let col = (square%8) as i32; //converts the type
        
        //take one step forward
        let one_step_sq = square as i32 + forward; //so the index of the square in front
        if one_step_sq < 64 && one_step_sq >= 0 {
            let one_step_bitboard = 1u64 << one_step_sq;
            if one_step_bitboard & occupied != 0 { //if you go to an occupied square
                continue;
            }
            legal_moves = legal_moves | one_step_bitboard;

            //either take one step or two
            if row == start_row {
                //you can take two steps.
                let two_steps_sq: i32 = one_step_sq + forward;
                let two_steps_bitboard = 1u64 << two_steps_sq; //pushes to the index of it
                if two_steps_bitboard & occupied != 0{ // there is something there
                    continue;
                }
                legal_moves = legal_moves | two_steps_bitboard;
            }

            if (square+1)%8 != 0 {
                let right_bitboard = one_step_bitboard>> 1;
                if right_bitboard & enemies != 0 {
                    legal_moves |= right_bitboard
                }
            }
            if (square)%8 != 0 {
                let left_bitboard = one_step_bitboard << 1;
                if left_bitboard & enemies != 0 {
                    legal_moves |= left_bitboard
                }
            }
        }
        //diagonal captures
        //if you can capture then you can move
        
        
    }

        return legal_moves
}

fn remove_captured_piece(all_pieces: &mut Vec<BoardRepresentation>, color:ColorType, to_bitboard:BitboardType) {
    let (start, end) = if color == ColorType::White { (6, 12) } else { (0, 6) };
    for i in start..end {
        all_pieces[i].position &= !to_bitboard //if you are overlapping with to_bitboards -> gone
    }
}

//returns the attack_bitboard of a single piece
pub fn single_piece_moves(piece:&BoardRepresentation, start_bitboard:BitboardType, merged_boards:&Vec<u64>) -> BitboardType {

    //because rn theres only BoardRepresentation of whole sets of pieces.
    let single_piece = BoardRepresentation{
        piece: piece.piece,
        color: piece.color, 
        position: start_bitboard,
        attack: 0,
    };

    if single_piece.piece == PieceType::Pawn{
        pawn_legal_moves(&single_piece, merged_boards) //
    } else {
        update_attack(&single_piece, merged_boards)
    }
}

fn taking_input() -> [u64; 2] { //takes in like h5 returns (x, y) coordinates

    loop {
        // println!("hi");
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
    1u64<<bitboard_index

    //-> type points at what output looks like    // let mut column: String = String::new();    // io::stdin().read_line(&mut column).expect("Failed to read");    //rather than taking in inputs take in parameters instead.
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
    let right_wall:BitboardType = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
    let up_wall:BitboardType = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
    let down_wall:BitboardType = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;

    let comerades = if chosen.color == ColorType::White{merged_boards[I_WHITE_POS]} else {merged_boards[I_BLACK_POS]};
    let enemies = if chosen.color == ColorType::White{merged_boards[I_BLACK_POS]} else {merged_boards[I_WHITE_POS]};

    let mut new_attack_board:BitboardType = 0;

    match chosen.piece{
        PieceType::Pawn => {
            // forward = if chosen.color == ColorType::White {8} else {-8};
            // square_i = chosen as i32; 
            if chosen.color == ColorType::White {
                // chosen.position 
                for square in 0..64{
                    let piece_bitboard = 1u64<<square;
                    if piece_bitboard & chosen.position == 0b0{
                        continue;
                    }
                
                    if square%8 != 0 { //not on left side (A)
                        new_attack_board = new_attack_board | (piece_bitboard<<7);
                    }
                    if (square+1)%8 != 0{ // not on right side (H)
                        new_attack_board = new_attack_board | (piece_bitboard<<9); //moves right
                    }
                    new_attack_board = new_attack_board & !comerades; //everything thats on comerades gets erased.
                
                    //either looop through and get all the individual positions
                    //or take the binary thingie and 
                    }
            } else {
                for square in 0..64{
                    let piece_bitboard = 1u64<<square;
                    if piece_bitboard & chosen.position == 0b0{
                        continue;
                    }
                    if square%8 != 0 {
                        new_attack_board = new_attack_board | (piece_bitboard>>9);
                    }
                    if (square+1)%8 != 0{
                        new_attack_board = new_attack_board | (piece_bitboard>>7);
                    }
                    new_attack_board = new_attack_board & !comerades;   
                }
            }
            return new_attack_board & !comerades; //i never check if i walk into comerades or not. Dont want to be able to attack comerades.
        }

        PieceType::Bishop => {
            // let mut pos:u64 = 0;
            let order:[(i32, bool, u64); 4] = [
                (7, true, left_wall|up_wall),
                (9, true, right_wall|up_wall),
                (7, false, right_wall|down_wall),
                (9, false, left_wall|down_wall)
            ];

            for square in 0..64{
                let piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }

                for (step, is_left, wall) in order {
                    if piece_bitboard & wall != 0b0 {
                        continue //crash into the wall
                    }

                    for i in 1..8{
                        let check_pos = if is_left {piece_bitboard << step*i} else {piece_bitboard >> step*i};
                        
                        if check_pos & comerades != 0 {
                            break
                        }

                        new_attack_board = new_attack_board | check_pos;

                        if check_pos & wall != 0 || check_pos & enemies != 0 {
                            break
                        }
                    }
                }
            }
            return new_attack_board;
        }

        PieceType::Knight => {
            //if up and up up has a thing you cant go
            //if side and side side, if down and down down.
            //doesnt matetr if its comerade or enemy
            //but check where you're going if theres comerade there. enemy doesnt matter at the palce you're landing
            
            // let row_78:u64 = 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
            // let row_12:u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111;

            let col_gh:u64 = 0b11000000_11000000_11000000_11000000_11000000_11000000_11000000_11000000;
            let col_ab:u64 = 0b00000011_00000011_00000011_00000011_00000011_00000011_00000011_00000011;

            for square in 0..64 {
                let piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }
                let row = square/8; //0 -> 7
                if piece_bitboard & left_wall == 0 {
                    if row >= 2 {new_attack_board |= piece_bitboard >> 17;}
                    if row <= 5 {new_attack_board |= piece_bitboard << 15;}
                }
                if piece_bitboard & col_ab == 0 {
                    if row <= 6 {new_attack_board |= piece_bitboard << 6;}
                    if row >= 1 {new_attack_board |= piece_bitboard >> 10;} 
                }
                if piece_bitboard & right_wall == 0 {
                    if row <= 5 {new_attack_board |= piece_bitboard << 17;}
                    if row >= 2 {new_attack_board |= piece_bitboard >> 15;}
                }
                if piece_bitboard & col_gh == 0 {
                    if row <= 6 {new_attack_board |= piece_bitboard << 10;}
                    if row >= 1 {new_attack_board |= piece_bitboard >> 6;}
                }
            }
            return new_attack_board & !comerades;
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
                let piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }

                for (step, is_left, wall) in order {
                    if piece_bitboard & wall != 0b0 {
                        continue //crash into the wall
                    }

                    for i in 1..8{
                        let check_pos = if is_left {piece_bitboard << step*i} else {piece_bitboard >> step*i};
                        
                        if check_pos & comerades != 0 {
                            break
                        }

                        new_attack_board = new_attack_board | check_pos;

                        if check_pos & wall != 0 || check_pos & enemies != 0 {
                            break
                        }
                    }
                }
            }
            return new_attack_board;
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
                (7, false, right_wall|down_wall),
                (9, false, left_wall|down_wall)

            ];
            for square in 0..64{
                let mut piece_bitboard = 1u64<<square;
                if piece_bitboard & chosen.position == 0b0{
                    continue;
                }

                for (step, is_left, wall) in order {
                    if piece_bitboard & wall != 0b0 {
                        continue 
                    }

                    for i in 1..8{
                        let check_pos = if is_left {piece_bitboard << step*i} else {piece_bitboard >> step*i};
                        if check_pos & comerades != 0 {
                            break
                        }
                        new_attack_board = new_attack_board | check_pos;
                        if check_pos & wall != 0 || check_pos & enemies != 0 {
                            break
                        }
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
                (7, true, up_wall|left_wall),
                (8, true, up_wall),
                (9, true, up_wall|right_wall),
                (1, true, right_wall),
                (7, false, down_wall|right_wall), //minskar
                (8, false, down_wall),
                (9, false, down_wall|left_wall),
                (1, false, left_wall)
            ];
            for (step, is_left, wall) in order{

                if piece_bitboard & wall == 0b0{
                    new_attack_board = if is_left {new_attack_board | piece_bitboard<<step} else {new_attack_board | piece_bitboard>>step};
                }
            }

            return new_attack_board & !comerades
        }
        //Make a board with all the possible moves then & it with !wall
    }
}

fn pawn_upgrades(pawn:&mut BoardRepresentation){
    //change the struct, take away one pawn and add one of the piece you chose
    pawn.piece = PieceType::Queen;
}

fn is_own_piece(chosen_bitboard:BitboardType, board: &Board) -> bool{ //check if youo have chosen a piece of your own color
    if chosen_bitboard & board.merged_boards[I_NOT_OCCUPIED] != 0 {
        //nothing is there. You can't chose it
        return false;
    }
    if board.turn == ColorType::White {
        //you want to chose a white pos
        if chosen_bitboard & board.merged_boards[I_WHITE_POS] != 0 {
            return true
        }
        return false
        //you chose a white pos
    } else {
        chosen_bitboard & board.merged_boards[I_BLACK_POS] != 0 
        //returns true if you chose a piece of your own color
    }
}

fn filter_noneplayable_squares(start_bitboard:BitboardType, to_bitboard:BitboardType, board:&Board) -> bool{
    if !is_own_piece(start_bitboard, board) {
        return false;
    }

    let (_, current_piece_i) = identify_chosen_piece(board, start_bitboard);
    let current_piece = board.all_pieces[current_piece_i];

    let legal_moves = single_piece_moves(&board.all_pieces[current_piece_i], start_bitboard, &board.merged_boards);

    // let legal_moves:BitboardType = if current_piece.piece == PieceType::Pawn{//Because pawns move differently
    //     pawn_legal_moves(&current_piece, &board.merged_boards)
    // } else {
    //     current_piece.attack
    // };

    if legal_moves & to_bitboard == 0b0 {
        return false;
    }
    return true //if it passed filter then return true
}

fn identify_chosen_piece(board:&Board, chosen_bitboard:BitboardType) -> (u64, usize){
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


pub fn print_possible_moves(board: &Board, chosen_bitboard:BitboardType) {
    //Chosen bitboards
    let (_, current_i) = identify_chosen_piece(board, chosen_bitboard);
    let piece = board.all_pieces[current_i];
    let moves = single_piece_moves(&piece, chosen_bitboard, &board.merged_boards);
    print_board(moves);

}


pub fn print_board(bitboard:BitboardType) {
    println!("*-------------------*");
    for row in (0..8).rev(){
        print!("{} | ", row+1);
        for column in (0..8).rev(){
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
    println!("*-------------------*");
    println!("    a b c d e f g h  ");
}


#[test]
fn play_game() {
    let mut board = Board::new();

    loop {
        print_board(board.merged_boards[I_OCCUPIED]);

        println!("It's {}'s turn to move, enter coordinates: ", if board.turn == ColorType::White {"white"} else {"black"});

        //taking start input:
        let start_coords = taking_input();
        let start_bitboard = input_coordinates(start_coords[0], start_coords[1]);

        // println!("hii");
        // println!("{:b}", start_bitboard);
        // println!("{}", is_own_piece(start_bitboard, &board));
        if is_own_piece(start_bitboard, &board) == false{
            println!("This isnt your piece, try again");
            continue
        }

        print_possible_moves(&board, start_bitboard);

        print_board(board.merged_boards[I_WHITE_POS]|board.merged_boards[I_BLACK_POS]);

        //taking to input:
        println!("Where do you want to move: ");
        let to_coords = taking_input();
        // let to_bitboard = input_coordinates(to_coords[0], to_coords[1]);

        if board.making_move(start_coords, to_coords) {
            println!("move made")
        } else {
            println!("Illegal move, try again")
        }
        // filter_noneplayable_squares(start_bitboard, to_bitboard, &board)
    }
}


#[test]
fn hexa_to_bin() {
    let file_a: u64 = 0x0101_0101_0101_0101;
    let file_b: u64 = 0x0202_0202_0202_0202;
    let file_g: u64 = 0x4040_4040_4040_4040;
    let file_h: u64 = 0x8080_8080_8080_8080;
    println!("{:b}", file_a);
    println!("{:b}", file_b);
    println!("{:b}", file_g);
    println!("{:b}", file_h);
}

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
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {

//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


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