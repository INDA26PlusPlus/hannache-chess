

per color:
unique pieces: 6
total pieces per color: 16

Idea 1:
12 arrays:
mod + downdivide to figure out pos.

## bitboard:
represent black and white
one board for black_player, one for white_player

#if plack_player and black_board 
check red and blue

one dimentional
array = 0000000001000000000... u64
to acess the location: number[row*8 + column]

OR and AND
0 OR 1 -> 1 #is there a character here?
0 AND 1 -> 0 #can i attack here?
1 AND 1 -> 1 #can i walk here/ can i attack here

#attacking
all_white_attack, all_black_position
all_black_attack, all_black_position

#moving
all_white_attack, all_white_position (check if its yourself? no, no need., you can't walk to yourself anyway hahahahahahahahk)
all_black_attack, all_black_postiion



# all pieces + Rules
pawn:
- start: 1/2 steps + can capture diagonally if two steps
- other: 1 step + captures diagonally one
- can't walk through people

bishop:
- captures diagonally
- can't walk through people

knight:
- two one direction then one to the side
- can hop over one character

rook:
- up or side
- can't walk throuhg people
- can switch places with the king

queen:
- up down side to side + diagonally
- can't walk through people

king:
- one all sides, up down side to side + diagonally
- can't walk trough people

can't make oneselves in check
can't walk through people (other than knight)
pawn changes type once at the other side.

how to end
checkmate: checked persons turn and king in check and all moves leads to check
stalemate: 50 turns and no paw has moved or nothing has been captures OR all moves lead to check and king not in check (usually only king left)
give up: you give up press a button typ

make a checkmate function?
check outside the board