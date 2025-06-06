// we have a 4096 bytes of memory
// 16 registers
// screen, sound, keyboard
// timers


// most programs start at 512 byte

fn main() {
    let mut memory: [u8; 4096] = [0; 4096];
    // 16 registers
    let mut v0: u8;
    let mut v1: u8;
    let mut v2: u8;
    let mut v3: u8;
    let mut v4: u8;
    let mut v5: u8;
    let mut v6: u8;
    let mut v7: u8;
    let mut v8: u8;
    let mut v9: u8;
    let mut vA: u8;
    let mut vB: u8;
    let mut vC: u8;
    let mut vD: u8;
    let mut vE: u8;
    let mut vF: u8; // this one shouldn't be used by any program because it is used as flag by some instructoins

    // this is 16 bit register used to store memory address
    let mut I: u16;


    // when this two guys are non-zero they are automatically incremented at 60hz
    let mut delay_reg: u8;
    let mut sound_reg: u8;


    // registers that should not be accessible by programs
    let pc: u16; // program counter
    let sp: u8; // stack pointer, topmost level of the stack


    // stack stores the addresses interpreter should return to when finishing a subroutine
    let stack: [u16; 16];





    // the keyboard is 0-9, A-F
    // i need to map it somehow



    // display is (0,0) - (63, 31)
    // monochrome
    let mut display: [[u8; 64]; 32] = [[0; 64]; 32]; 



    // sprites
    // up to 15 bytes so 8x15 image
    // there are also sprites for hex digits in interpreter part of the memory





    // two timers
    // the delay timer is active whenever the delay timer register is non-zero
    // all it does is subtract 1 from the delay timer register at 60hz rate until the register is 0, then it stops working
    // the sound time is kinda the same; when >0 the buzzer is buzzing
    // only has one frequency





    // instructions
    // there are 36 instructions, 2 bytes each, most significant byte first
    // first byte of each instruction in memory should be located at even address
    // if the program contains sprites, the instructions should be padded accordingly

    // 0nnn - jump to machine code routine at nnn, ignored



    // 00E0 - clear screen
    // for pixel_row in &mut display {
    //     for pixel in pixel_row {
    //         *pixel = 0;
    //     }
    // }



    // 00EE - return from the stack, get the address from the top of the stack, assign program counter to it
    // pc = stack[sp]; // or something like this, indexing only works with usize, so i need to change it to usize, insteade of converting 
    // ok, the compiler can optimize the "as u8", so its probably ok



    // 1nnn - jump to some address, need to set the program counter
    // first we need to get the 12 rightmost bits
    // let jump_instruction = 0x1ABC;
    // let jump_address = jump_instruction & 0x0FFF; 
    // pc = jump_address;
    // so we have the instruction here with the rightmost 12 bits defining the location
    // we do bitwise AND on this value with 0000 1111 1111 1111
    // and we get those bits with 4 leading zeroes
     
    println!("Hello, world!");
}
