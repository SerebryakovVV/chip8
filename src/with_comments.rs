#[allow(dead_code, unused_mut, unused_variables, unused_assignments)]


use rand::{Rng, rng};
use raylib::prelude::*;

macro_rules! extract {
	($instruction:expr, $mask:expr, $offset:expr) => {
		(($instruction & $mask) >> $offset)
	};
}


// we have a 4096 bytes of memory
// 16 registers
// screen, sound, keyboard
// timers


// most programs start at 512 byte


// enum Key {
// 	ZERO,
// 	ONE,
// 	TWO,
// 	THREE,
// 	FOUR,
// 	FIVE,
// 	SIX,
// 	SEVEN,
// 	EIGHT,
// 	NINE,
// 	A,
// 	B,
// 	C,
// 	D,
// 	E,
// 	F
// }


fn check_key_down(key: u8) -> bool {
	todo!()
}

fn wait_for_key() -> u8 {todo!()}


fn main() {

	let mut rnd_gen = rng();


	// let (mut rl_handle, rl_thread) = raylib::init().size(640, 320).build();
	// while !rl_handle.window_should_close() {
	// 	    let mut d = rl_handle.begin_drawing(&rl_thread);

  //       d.clear_background(Color::RAYWHITE);
  //       d.draw_text("Hello from raylib-rs!", 190, 200, 20, Color::DARKGRAY);
	// };
	
	// let a = rnd_gen.random::<u8>();
	// println!("{}, {}, {}, {}", rnd_gen.random::<u8>(), rnd_gen.random::<u8>(), rnd_gen.random::<u8>(), rnd_gen.random::<u8>());



	// return;

	// 4096 bytes of memory
	let mut memory: [u8; 4096] = [0; 4096];
	// 16 8-bit registers, the last one shouldn't be used, because it is used as flag by some instructions
	let mut registers: [u8; 16] = [0; 16];
	// 16-bit register to store a memory address
	let mut mem_addr_reg: u16 = 0;

	
	// two timers
	// the delay timer is active whenever the delay timer register is non-zero
	// all it does is subtract 1 from the delay timer register at 60hz rate until the register is 0, then it stops working
	// the sound time is kinda the same; when >0 the buzzer is buzzing
	// only has one frequency
	let mut delay_reg: u8 = 0;
	let mut sound_reg: u8 = 0;


	// when these two guys are non-zero they are automatically incremented at 60hz
	// program counter
	let mut pc: u16 = 0x200; 
	// stack pointer, topmost level of the stack
	let mut sp: u8 = 0;

	// stack stores the addresses interpreter should return to when finishing a subroutine
	let mut stack: [u16; 16] = [0; 16];


	// the keyboard is 0-9, A-F
	// i need to map it somehow


	// display is (0,0) - (63, 31)
	// monochrome
	let mut display: [[u8; 64]; 32] = [[0; 64]; 32]; 







	// font, 0-F
	  let font: [u8; 80] = [
			0xF0, 0x90, 0x90, 0x90, 0xF0,
			0x20, 0x60, 0x20, 0x20, 0x70,
			0xF0, 0x10, 0xF0, 0x80, 0xF0,
			0xF0, 0x10, 0xF0, 0x10, 0xF0,
			0x90, 0x90, 0xF0, 0x10, 0x10,
			0xF0, 0x80, 0xF0, 0x10, 0xF0,
			0xF0, 0x80, 0xF0, 0x90, 0xF0,
			0xF0, 0x10, 0x20, 0x40, 0x40,
			0xF0, 0x90, 0xF0, 0x90, 0xF0,
			0xF0, 0x90, 0xF0, 0x10, 0xF0,
			0xF0, 0x90, 0xF0, 0x90, 0x90,
			0xE0, 0x90, 0xE0, 0x90, 0xE0,
			0xF0, 0x80, 0x80, 0x80, 0xF0,
			0xE0, 0x90, 0x90, 0x90, 0xE0,
			0xF0, 0x80, 0xF0, 0x80, 0xF0,
			0xF0, 0x80, 0xF0, 0x80, 0x80 
		];


		for (symbol, mem_cell) in font.iter().zip(memory.iter_mut()) {
			*mem_cell = *symbol;
		}



















	// sprites
	// up to 15 bytes so 8x15 image
	// there are also sprites for hex digits in interpreter part of the memory


	// get the program, load to memory, access each instructio, parse it

	// i need to get the rom and put it into the memory at 0x200

	// here i will get two elements of the memory array representing an operation
	// but in the match arms i have u16
	// so i will need to convert two u8 to one u16
	let instruction: u16 = 0;
	match instruction & 0xF000 {
		0x0000 => {
			match instruction & 0x0FFF {
				0x00E0 => { // clear screen
					for pixel_row in &mut display {
						for pixel in pixel_row {
							*pixel = 0;
						}
					}
				},
				0x00EE => { // return from the stack, get the address from the top of the stack, assign program counter to it
					pc = stack[sp as usize];
					sp -= 1;
				},
				_      => todo!(), // just skip, the 0x0nnn instruction was used to jump to machine code routine at nnn
			}
		},
		0x1000 => { // jump to some address nnn in 0x1nnn
			pc = instruction & 0x0FFF;
		},
		0x2000 => { // call to some address
			sp += 1;
			stack[sp as usize] = pc;
			pc = instruction & 0x0FFF;
		},
		0x3000 => { // skip next instruction if the value in register is equal to the byte
			let reg = (instruction & 0x0F00) >> 8;
			if registers[reg as usize] == (instruction & 0x00FF) as u8 {
				// by 2 because instruction are 16 bits, and i will probably have pc incrementation somewhere at the end of the loop
				pc += 2;
			};
		},
		0x4000 => { // same as 0x3, but skip if not equal now
			let reg = (instruction & 0x0F00) >> 8;
				if registers[reg as usize] != (instruction & 0x00FF) as u8 {
				pc += 2;
			};
		},
		0x5000 => { // skipping next instruction if two regisers are equal
			let reg_one = (instruction & 0x0F00) >> 8;
			let reg_two = (instruction & 0x00F0) >> 4;
			if registers[reg_one as usize] == registers[reg_two as usize] {
				pc += 2;
			}
		},
		0x6000 => { // putting a value into a register
			let reg = extract!(instruction, 0x0F00, 8);
			registers[reg as usize] = (instruction & 0x00FF) as u8;
		},
		0x7000 => { // ADD the value to the register
			let reg = extract!(instruction, 0x0F00, 8);
			registers[reg as usize] += (instruction & 0x00FF) as u8;
		},
		0x8000 => { // different types of bit operations in this one
			let reg_x = extract!(instruction, 0x0F00, 8) as usize;
			let reg_y = extract!(instruction, 0x00F0, 4) as usize;
			match instruction & 0x000F {
				// set the x register to the value of y register
				0x0000 => registers[reg_x] = registers[reg_y],
				// bitwise or on two register, then store in the first one
				0x0001 => registers[reg_x] = registers[reg_x] | registers[reg_y],
				// bitwise and on two registers, then store in the first one
				0x0002 => registers[reg_x] = registers[reg_x] & registers[reg_y],
				// bitwise xor on two registers, then store in the first one
				0x0003 => registers[reg_x] = registers[reg_x] ^ registers[reg_y],
				// add two registers, if the result overflows, set the flag
				0x0004 => {
					let (res, did_overflow) = registers[reg_x].overflowing_add(registers[reg_y]);
					registers[15] = if did_overflow {1} else {0}; 
					registers[reg_x] = res;
				},
				// subtract two registers, set the flag for underflow
				0x0005 => { 
					let (res, did_underflow) = registers[reg_x].overflowing_sub(registers[reg_y]);
					registers[15] = if did_underflow {0} else {1};
					registers[reg_x] = res;
				},
				// check the least significant bit of reg_x, set the flag, divide reg_x by 2
				0x0006 => {
					registers[15] = if registers[reg_x] & 0b_0000_0001 == 0x0001 {1} else {0};
					registers[reg_x] >>= 1;
				},
				// subtract two registers, set the flag for underflow
				0x0007 => {
					let (res, did_underflow) = registers[reg_y].overflowing_sub(registers[reg_x]);
					registers[15] = if did_underflow {0} else {1};
					registers[reg_x] = res;
				},
				// check the most significant bit of reg_x, set the flag, multiply by 2
				0x000E => {
					// can also be done as registers[15] = (registers[reg_x] >> 7) & 1;
					registers[15] = if registers[reg_x] & 0b_1000_0000 == 0b_1000_0000 {1} else {0};
					registers[reg_x] <<= 1;
				},
				_      => panic!() // shouldn't be a accessable
			}
		},
		// skip an instruction if two registers are equal
		0x9000 => {
			let reg_x = extract!(instruction, 0x0F00, 8) as usize;
			let reg_y = extract!(instruction, 0x00F0, 4) as usize;
			if registers[reg_x] != registers[reg_y] {
				pc += 2;
			}
		},
		// set the register to some value
		0xA000 => mem_addr_reg = instruction & 0x0FFF,
		// set program counter to some value plus value of register 0
		0xB000 => pc = (instruction & 0x0FFF) + registers[0] as u16,
		// 0xCxkk generate a random number 0-255, AND it with right 8 bits, store result in x register
		0xC000 => {
			let rnd_num = rnd_gen.random::<u8>();
			let reg_x = extract!(instruction, 0x0F00, 8) as usize; 
			registers[reg_x] = rnd_num & ((instruction & 0x00FF) as u8);
		},




			
			// read 'length' bytes from memory starting from mem_addr_reg
			// dislplay them as sprite at (x,y) on the display
			// sprites are XORed onto the screen
			// if this causes any pixel to be erased, reg15 is set to 1, otherwise it is set to 0
			// wrapping

			// this has wrapping and flag for pixel erased
		0xD000 => {
			let reg_x = extract!(instruction, 0x0F00, 8) as usize;
			let reg_y = extract!(instruction, 0x00F0, 4) as usize;
			let mut x = registers[reg_x] as usize;
			let mut y = registers[reg_y] as usize;
			let mut length = instruction & 0x000F; // number of bytes in the sprite, height
			let mut addr = mem_addr_reg as usize;
			registers[15] = 0;
			while length > 0 {
				let pixel_byte = memory[addr];
				for i in (0..8).rev() {
					let wrapped_x = x % 64;
					let wrapped_y = y % 32;
					let old = display[wrapped_y][wrapped_x];
					display[wrapped_y][wrapped_x] ^= (pixel_byte >> i) & 0x0001;
					let new = display[wrapped_y][wrapped_x];
					if old == 1 && new == 0 {registers[15] = 1};
					x+=1;
				}
				x-=8;
				y+=1;
				addr+=1;
				length -= 1;
			}
		},
		0xE000 => {
			match instruction & 0x00FF {
				0x009E => {
					let reg = extract!(instruction, 0x0F00, 8) as usize;
					if check_key_down(registers[reg]) {
						pc += 2;
					};
				},
				0x00A1 => {
					let reg = extract!(instruction, 0x0F00, 8) as usize;
					if !check_key_down(registers[reg]) {
						pc += 2;
					};
				},
				_      => panic!()
			}
		},






		0xF000 => {
			let reg = extract!(instruction, 0x0F00, 8) as usize;
			match instruction & 0x00FF {
				0x0007 => registers[reg] = delay_reg,
				0x000A => {
					let reg = extract!(instruction, 0x0F00, 8) as usize;
					let key = wait_for_key();
					registers[reg] = key;

				},
				0x0015 => delay_reg = registers[reg],
				0x0018 => sound_reg = registers[reg],
				0x001E => mem_addr_reg += registers[reg] as u16,
				// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
				0x0029 => {
					let value = extract!(instruction, 0x0F00, 8);
					mem_addr_reg = value * 5;
				},
				// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
				0x0033 => {
					let reg = extract!(instruction, 0x0F00, 8) as usize;
					let mut num = registers[reg];
					let location = mem_addr_reg as usize;
					memory[location + 2] = num % 10;
					num /= 10;
					memory[location + 1] = num % 10;
					num /= 10;
					memory[location] = num % 10;
				},
				0x0055 => {
					let mut addr = mem_addr_reg as usize;
					let mut reg_counter: usize = 0;
					while reg_counter <= reg {
						memory[addr] = registers[reg_counter];
						addr+=1;
						reg_counter+=1;
					}
				},
				0x0065 => {
					let mut addr = mem_addr_reg as usize;
					let mut reg_counter: usize = 0;
					while reg_counter <= reg {
						registers[reg_counter] = memory[addr];
						addr+=1;
						reg_counter+=1;
					}
				},
				_      => panic!(),
			}
		},
		_      => panic!()
	}


    // instructions
    // there are 36 instructions, 2 bytes each, most significant byte first
    // first byte of each instruction in memory should be located at even address
    // if the program contains sprites, the instructions should be padded accordingly








     

}



#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_3xkk_instruction_calculations() {
		assert_eq!((0x3A28 & 0x0F00) >> 8, 0x000A);
	}

	#[test]
	fn test_extract_macro() {
		assert_eq!((0x3FBC & 0x0F00) >> 8, extract!(0x3FBC, 0x0F00, 8));
	}

	#[test]
	fn test_bitwise_or() {
		assert_eq!(0b10001 | 0b11010, 0b11011);
		assert_eq!(0x0101 | 0x0100, 0x0101);
	}
}