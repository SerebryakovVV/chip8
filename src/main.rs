use std::{fs::File, io::Read, process};

#[allow(dead_code, unused_mut, unused_variables, unused_assignments)]



// TODO: add bounds checking
// display
// input
// sound
// timers decrement

use rand::{Rng, rng};
use raylib::prelude::*;

macro_rules! extract {
	($instruction:expr, $mask:expr, $offset:expr) => {
		(($instruction & $mask) >> $offset)
	};
}





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


fn print_before_panic(display: [[u8; 64]; 32]) {
	let (mut rl_handle, rl_thread) = raylib::init().size(640, 320).build();
	while !rl_handle.window_should_close() {
		let mut d = rl_handle.begin_drawing(&rl_thread);

		for (ri, row) in display.iter().enumerate() {
			for (pi, pixel) in row.iter().enumerate() {
				d.draw_rectangle((pi*10) as i32, (ri*10) as i32, 10, 10, if *pixel == 1 { Color::BLACK} else {Color::WHITE});
			}
		}

    d.clear_background(Color::RAYWHITE);
    // d.draw_text("Hello from raylib-rs!", 190, 200, 20, Color::DARKGRAY);
	};
}

fn main() {
	let mut rnd_gen = rng();
	let mut memory: [u8; 4096] = [0; 4096];
	let mut registers: [u8; 16] = [0; 16];
	let mut mem_addr_reg: u16 = 0;
	let mut delay_reg: u8 = 0;
	let mut sound_reg: u8 = 0;
	let mut pc: u16 = 0x200; 
	let mut sp: u8 = 0;
	let mut stack: [u16; 16] = [0; 16];
	let mut display: [[u8; 64]; 32] = [[0; 64]; 32];


	let mut file = File::open("test_opcode.ch8").unwrap_or_else(|e| {
		println!("Error while loading ROM: {}", e);
		process::exit(1);
	});
	file.read(&mut memory[0x200..]).unwrap_or_else(|e| {                   // changed from read_exact to this, because failed to fill the buffer
		println!("Error while moving ROM to memory array: {}", e);
		process::exit(1);
	});


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

	let mut count_cycles = 0;

	loop {


		let instruction = ((memory[pc as usize] as u16) << 8) | (memory[(pc + 1) as usize] as u16);
		match instruction & 0xF000 {
			0x0000 => {
				match instruction & 0x0FFF {
					0x00E0 => {
						for pixel_row in &mut display {
							for pixel in pixel_row {
								*pixel = 0;
							}
						}
					},
					0x00EE => {
						pc = stack[sp as usize];
						sp -= 1;
					},
					_      => {
						// println!("{:#?}", registers);
						// println!("{}", pc);
						// println!("{}", sp);
						// println!("{}", count_cycles);
						// println!("{}", instruction);
						print_before_panic(display);
						todo!()
					},
				}
			},
			0x1000 => {
				pc = instruction & 0x0FFF;
			},
			0x2000 => {
				sp += 1;
				stack[sp as usize] = pc;
				pc = instruction & 0x0FFF;
			},
			0x3000 => {
				let reg = (instruction & 0x0F00) >> 8;
				if registers[reg as usize] == (instruction & 0x00FF) as u8 {
					pc += 2;
				};
			},
			0x4000 => {
				let reg = (instruction & 0x0F00) >> 8;
					if registers[reg as usize] != (instruction & 0x00FF) as u8 {
					pc += 2;
				};
			},
			0x5000 => {
				let reg_one = (instruction & 0x0F00) >> 8;
				let reg_two = (instruction & 0x00F0) >> 4;
				if registers[reg_one as usize] == registers[reg_two as usize] {
					pc += 2;
				}
			},
			0x6000 => {
				let reg = extract!(instruction, 0x0F00, 8);
				registers[reg as usize] = (instruction & 0x00FF) as u8;
			},
			0x7000 => {
				let reg = extract!(instruction, 0x0F00, 8);
				// registers[reg as usize] += (instruction & 0x00FF) as u8;
				registers[reg as usize] = registers[reg as usize].wrapping_add((instruction & 0x00FF) as u8);
			},
			0x8000 => {
				let reg_x = extract!(instruction, 0x0F00, 8) as usize;
				let reg_y = extract!(instruction, 0x00F0, 4) as usize;
				match instruction & 0x000F {
					0x0000 => registers[reg_x] = registers[reg_y],
					0x0001 => registers[reg_x] = registers[reg_x] | registers[reg_y],
					0x0002 => registers[reg_x] = registers[reg_x] & registers[reg_y],
					0x0003 => registers[reg_x] = registers[reg_x] ^ registers[reg_y],
					0x0004 => {
						let (res, did_overflow) = registers[reg_x].overflowing_add(registers[reg_y]);
						registers[15] = if did_overflow {1} else {0}; 
						registers[reg_x] = res;
					},
					0x0005 => { 
						let (res, did_underflow) = registers[reg_x].overflowing_sub(registers[reg_y]);
						registers[15] = if did_underflow {0} else {1};
						registers[reg_x] = res;
					},
					0x0006 => {
						registers[15] = if registers[reg_x] & 0b_0000_0001 == 0x0001 {1} else {0};
						registers[reg_x] >>= 1;
					},
					0x0007 => {
						let (res, did_underflow) = registers[reg_y].overflowing_sub(registers[reg_x]);
						registers[15] = if did_underflow {0} else {1};
						registers[reg_x] = res;
					},
					0x000E => {
						registers[15] = if registers[reg_x] & 0b_1000_0000 == 0b_1000_0000 {1} else {0};
						registers[reg_x] <<= 1;
					},
					_      => panic!()
				}
			},
			0x9000 => {
				let reg_x = extract!(instruction, 0x0F00, 8) as usize;
				let reg_y = extract!(instruction, 0x00F0, 4) as usize;
				if registers[reg_x] != registers[reg_y] {
					pc += 2;
				}
			},
			0xA000 => mem_addr_reg = instruction & 0x0FFF,
			0xB000 => pc = (instruction & 0x0FFF) + registers[0] as u16,
			0xC000 => {
				let rnd_num = rnd_gen.random::<u8>();
				let reg_x = extract!(instruction, 0x0F00, 8) as usize; 
				registers[reg_x] = rnd_num & ((instruction & 0x00FF) as u8);
			},
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
					0x0029 => {
						let value = extract!(instruction, 0x0F00, 8);
						mem_addr_reg = value * 5;
					},
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
		};
		pc+=2;
		count_cycles += 1;
		println!("{:#04x}", instruction);
		// break;	
	}
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