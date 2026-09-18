# hannache-chess
Chess backend written in Rust


new() method initializes a new game. ans should be run at the start of each game
checking_check() method returns a bool if its in check or not, check -> true, not check -> false

has_legal_move() returns a bool. If there are any moves you can do that will leave your king not in check then it is true.

is_checkmate and is_stalemate returns bools for wether its in checkmate or not

making_move() is where a piece goes from position A to position B. returns a bool, if true it means that the move was sucessful.

merging_boards() returns a list of all the emrged boards, the indexes of each element is predefined: 
pub const I_WHITE_POS:usize = 0;
pub const I_WHITE_ATK:usize = 1; 
pub const I_BLACK_POS:usize = 2; 
pub const I_BLACK_ATK:usize = 3; 
pub const I_OCCUPIED:usize = 4; 
pub const I_NOT_OCCUPIED:usize = 5; 

pawn_legal_moves() returns the legal moves for a pawn in a bitboard.

remove_captured_piece() takes in all_pieces, the turn it is and where the piece has moved to/which. If its white then the whole piece will override the black piece.

single_piece_moves() returns the attack bitboard of one single piece. 

taking_input() I dont know if this is meant to be in backend but it takes in an input in the format "a1" etc. and returns a coordinate x and y

moving_piece() makes a new bitboard for where you want your new position to be.

update_attack() returns the new updated attack bitboard based on the chosen piece you send in.

pawn_upgrades() automatically upgrades your pawn into a queen

is_own_piece() checks if you have chosen you own piece or not during your turn

filter_nonplayable_squares() filters away the squares that are inacessible or that you would never play.

identify_chosen_piece() identifies what piecetype you are and returns the position board and your that pieces indedx in the all_pieces vec

print_possible_moves() prints the possible moves for the chosen_bitboard you put in.

print_board() prints the board where 0,0 is right bottom. 7, 7 is left top




example setup:
fn main() {
    let mut board = Board::new();

    loop {
        print_board(board.merged_boards[I_OCCUPIED]);

        println!("It's {}'s turn to move, enter coordinates: ", if board.turn == ColorType::White {"white"} else {"black"});

        //taking start input:
        let start_coords = taking_input();
        let start_bitboard = input_coordinates(start_coords[0], start_coords[1]);

        
        if is_own_piece(start_bitboard, &board) == false{
            println!("This isnt your piece, try again");
            continue
        }

        print_possible_moves(&board, start_bitboard);

        print_board(board.merged_boards[I_WHITE_POS]|board.merged_boards[I_BLACK_POS]);

        //taking to input:
        println!("Where do you want to move: ");
        let to_coords = taking_input();

        if board.making_move(start_coords, to_coords) {
            println!("move made")
        } else {
            println!("Illegal move, try again")
        }
    }
}