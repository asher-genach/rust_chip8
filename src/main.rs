extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;

const SCREEN_WIDTH_PIXELS:u32     = 64;
const SCREEN_HEIGHT_PIXELS:u32    = 48;
const PIXEL_SIZE:u32              = 10;

struct OpCode
{
  opcode:u16, // 2 bytes opcode
}

//  0x000-0x1FF  - Chip 8 interpreter (contains font set in emu)
//  0x050-0x0A0  - Used for the built in 4x5 pixel font set (0-F)
//  0x200-0xFFF  - Program ROM and work RAM
struct Memory
{
  memory:[u8;4096], // 4K of memory
}

struct Registers
{
  gen_purpose_regs:[u8;16], // V0..VE - 15 8-bit general purpose registers. 16th register carry flag 
  pc_reg:u16, // 0x000-0xFFF
  idx_reg:u16,
  delay_timer_reg:u8,
  sound_timer_reg:u8,
}

struct Graphics
{
  // Add piston window
  gfx:[u8;64*32],
}

impl Graphics
{
  fn draw()
  {
  }
}

// The Chip 8 instruction set has opcodes that allow the program to jump to a certain
// address or call a subroutine. While the specification dont mention a stack, you
// will need to implement one as part of the interpreter yourself. The stack is used
// to remember the current location before a jump is performed. So anytime you
// perform a jump or call a subroutine, store the program counter in the stack before
// proceeding. The system has 16 levels of stack and in order to remember which level
// of the stack is used, you need to implement a stack pointer (sp).
struct Stack
{
  stack:[u16;16],
  sp:u16,
}

// HEX based 0x0-0xF
struct KeyState
{
  key:[u8;16],
}

struct Chip8
{
  draw_flag:bool,
}

impl Chip8
{
  fn new() -> Self
  {
    Chip8 { draw_flag:false, }
  }

  fn initialize(&mut self)
  {

  }

  fn load_game(&mut self, game_name:&str)
  {

  }

  fn emulate_cycle(&mut self)
  {

  }
  
  fn is_draw_flag(&self) -> bool
  {
    self.draw_flag
  }
      
  fn set_keys(&mut self)
  {

  }
}

struct Emulator
{
  window: PistonWindow,
  chip8:  Chip8
}

impl Emulator
{
  fn new() -> Self
  {
    // TODO (Low):
    // ===========
    // This problematic to add a code that can crash inside a constructor.
    // But lets leave it this way for now.
    // Maybe I should return a Result<> and propagate it to the caller
    Emulator{ window:WindowSettings::new("CHIP-8 Emulator",
                [SCREEN_WIDTH_PIXELS*PIXEL_SIZE,
                 SCREEN_HEIGHT_PIXELS*PIXEL_SIZE])
                .exit_on_esc(true).build().unwrap(),
                chip8: Chip8::new() }
  }

  fn setup(&mut self, game_name:&str)
  {
    self.chip8.initialize();
    
    self.chip8.load_game( game_name );
  }

  fn draw_graphics(&self)
  {

  }

  fn main_loop(&mut self)
  {
    loop
    {
      self.chip8.emulate_cycle();
      
      if self.chip8.is_draw_flag()
      {
        self.draw_graphics();
      }

      self.chip8.set_keys();
    }
  }

  fn start(&mut self, game_name:&str)
  {
    self.setup( game_name );

    self.main_loop();
  }
}

fn main()
{ 
  Emulator::new().start("snake");

  thread::sleep(time::Duration::from_millis(3000));
}
