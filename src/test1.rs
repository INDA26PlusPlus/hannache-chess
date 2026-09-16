
// enum ColorType{
//     White,
//     Black
// }

// struct TestTest {
//     // hi: [u64; 12] //What is this?
//     permanent: ColorType,
//     variable: u64
// }

// // impl Ee {
//     fn hello() {
//         println!("Hello");
//     }

//     fn modifying(&mut self, new_position){// -> Self { //-> Self means that it has created a new instance of Ee (which in this case it hasnt)
//         self.position = new_position
//     }
// }


// #[test]
fn main() {
    // let mut test = TestTest{
    //     permanent: White,
    //     variable: 30
    // }
    // if (0b0011 ^ 0b0010) != 0b0 {//0 1 -> 0, 1 1 -> 1
    //     println!("Hi");
    // }
    let column = 0;
    let row = 3;

    let position = row*8 + column;
    //if its like 
    //0000
    //1100
    //then (0 indexed) (row+1)*8 - column so like this is uh like math

    // let og_bitboard: u64 = 0b1:


    println!("{:}", 0b000100); //in decimal form
    println!("{:b}", 0b000100); //in binary form
    let a:u64 = 0b1;
    // let b:u64 = 0b10011000; //this is the same as doing 00000011 ^ 11111100
    // let mut c:u64 = a ^ b;
    let mut d:u64 = &a<<63; //den e typ noll indexerat.
    println!("{:b}", a);
    println!("this is d: {:b}", d);
    let d:u64 = d<<2;
    println!("{:b}", a);
    println!("{:b}", d);
    // let mut temp_board::u64 = 0b1;//ig rest is filled with 0


    // // temp_board[row*8+chosen] = 1
    // println!("{}", temp_board);

}
    // Ee::hello();

    // let instance1 = Ee {
    //     // hi: 12
    //     color: ColorType::White,
    //     position:1,
    // };
    
    // println!("{}", instance1.position);
    // instance1.modifying(2);
    // println!("{}", instance1.position);


