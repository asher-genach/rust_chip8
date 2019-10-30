extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use std::{thread, time};
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;

const SCREEN_WIDTH_PIXELS:u32     = 64;
const SCREEN_HEIGHT_PIXELS:u32    = 48;
const PIXEL_SIZE:u32              = 10;

enum OpCodeSymbol
{
  UNDEF,
  ANNN, // Set I to the address NNN
}

struct OpCode
{
  val:u16, // 2 bytes opcode
}

impl OpCode
{
  fn new(val:u16) -> Self
  {
    OpCode { val }
  }

  fn find_opcode_symbol(&self) -> OpCodeSymbol
  {
    if ( self.val & 0xA000 != 0 )
    {
      return OpCodeSymbol::ANNN;
    }
    else
    {
      return OpCodeSymbol::UNDEF;
    }
  }
}

//  0x000-0x1FF  - Chip 8 interpreter (contains font set in emu)
//  0x050-0x0A0  - Used for the built in 4x5 pixel font set (0-F)
//  0x200-0xFFF  - Program ROM and work RAM
struct Memory
{
  memory:[u8;4096], // 4K of memory
}

impl Memory
{
  fn new() -> Self
  {
    Memory { memory:[0x0;4096] }
  }
}

struct Registers
{
  gen_purpose_regs:[u8;16], // V0..VE - 15 8-bit general purpose registers. 16th register carry flag 
  pc_reg:u16, // 0x000-0xFFF
  idx_reg:u16,
  delay_timer_reg:u8,
  sound_timer_reg:u8,
}

impl Registers
{
  fn new() -> Self
  {
    Registers
    {
      gen_purpose_regs:[0x0;16],
      pc_reg:0x0,
      idx_reg:0x0,
      delay_timer_reg:0,
      sound_timer_reg:0
    }
  }
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

impl Stack
{
  fn new() -> Self
  {
    Stack { stack:[0x0;16], sp:0x0 }
  }
}

// HEX based 0x0-0xF
struct KeyState
{
  key:[u8;16],
}

struct Chip8
{
  draw_flag:    bool,
  regs:         Registers,
  memory:       Memory,
  stack:        Stack,
  curr_opcode:  OpCode,
}

impl Chip8
{
  fn new() -> Self
  {
    Chip8 { draw_flag:false,
            regs:Registers::new(),
            memory:Memory::new(),
            stack:Stack::new(),
            curr_opcode:OpCode::new(0x0)
          }
  }

  fn initialize(&mut self)
  {
    // Initialize registers and memory once.
    self.regs.pc_reg   = 0x200;            // The program counter starts at address 0x200.
    self.curr_opcode   = OpCode::new(0x0); // Reset current opcode. 
    self.regs.idx_reg  = 0x0;              // Reset idx register.
    self.stack.sp      = 0x0;              // Reset stack pointer. 

    // Clear display	
    // Clear stack
    // Clear registers V0-VF
    // Clear memory
 
    // Load fontset
    /*for(int i = 0; i < 80; ++i)
      memory[i] = chip8_fontset[i];*/		
 
    // Reset timers
  }

  fn load_game(&mut self, file_name:&str)
  {
    let mut file = File::open(file_name).unwrap();
    
    let mut buffer = [0u8;0xDFF];
    
    file.read(&mut buffer).unwrap();

    for idx in 0..0xDFF
    {
      self.memory.memory[0x200 + idx] = buffer[idx];      

      /*      
      if buffer[idx] > 0
      {
        println!("{} {}", buffer[idx], self.memory.memory[0x200+idx]);
      }
      */
    }

    println!("Game loaded successfully...");
  }

  // Every cycle, the method emulateCycle is called which emulates
  // one cycle of the Chip 8 CPU. During this cycle, 
  // the emulator will Fetch, Decode and Execute one opcode.
  
  // Fetch opcode:
  // =============
  // During this step, the system will fetch one opcode from the
  // memory at the location specified by the program counter (pc).
  // In our Chip 8 emulator, data is stored in an array in which
  // each address contains one byte. As one opcode is 2 bytes long,
  // we will need to fetch two successive bytes and merge them to
  // get the actual opcode.
  fn fetch_opcode(&self) -> OpCode
  {
    let first_half:u8   = self.memory.memory[self.regs.pc_reg as usize];
    let second_half:u8  = self.memory.memory[(self.regs.pc_reg as usize + 1)];

    let val:u16 = ((first_half as u16 ) << 8) | (second_half as u16);

     OpCode::new(val)
  }

  fn execute_opcode(&mut self, opcode_symbol:OpCodeSymbol, opcode:OpCode)
  {
    match opcode_symbol
    {
      OpCodeSymbol::ANNN =>
      {
        self.regs.idx_reg = opcode.val & 0x0FFF;
        self.regs.pc_reg  += 2;
      },
      _ =>
      {
      }
    }
  }

  fn emulate_cycle(&mut self)
  {
    // Fetch opcode
    let cur_opcode = self.fetch_opcode();

    // Decode opcode
    let  opcode_symbol = cur_opcode.find_opcode_symbol();
    
    // Execute opcode
    self.execute_opcode( opcode_symbol, cur_opcode );

    // Update timers
    if self.regs.delay_timer_reg > 0
    {
      self.regs.delay_timer_reg -= 1;
    }

    if self.regs.sound_timer_reg > 0
    {
      if self.regs.sound_timer_reg == 1
      {
        println!("BEEP!");
      }

      self.regs.sound_timer_reg -= 1;
    }

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
  Emulator::new().start("pong.rom");

  // thread::sleep(time::Duration::from_millis(3000));
}
