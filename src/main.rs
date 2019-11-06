extern crate piston_window;
extern crate find_folder;
extern crate rand;

use piston_window::*;
use std::{thread, time};
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::fmt;
use rand::Rng;

const SCREEN_WIDTH_PIXELS:u32     = 64;
const SCREEN_HEIGHT_PIXELS:u32    = 48;
const PIXEL_SIZE:u32              = 10;

// TODO:
// 25 opcodes were implemented out of 35.
// 10 (or 11 see commented opcode) opcodes need to be implemented.

enum OpCodeSymbol
{
  UNDEF,
  _0NNN, //
  _00E0,
  _00EE,
  _1NNN,
  _2NNN,
  _3XNN,
  _4XNN,
  _5XY0,
  _6XNN,
  _7XNN,
  _8XY0,
  _8XY1,
  _8XY2,
  _8XY3,
  _8XY4,
  _8XY5,
  _8XY6,
  _8XY7,
  _8XYE,
  _9XY0,
  _ANNN, // Set I to the address NNN
  _BNNN,
  _CXNN,
  _DXYN,
  _EX9E,
  _EXA1,
  _FX07,
  _FX0A,
  _FX15,
  _FX18,
  _FX1E,
  _FX29,
  _FX33,
  _FX55,
  _FX65,
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
    if ( self.val & 0xF000 == 0x0 ) // The first 4 bits are 0000 (0x0)
    {
      if ( self.val & 0x0F00 == 0x0 ) // The second 4 bits are 0000 (0x0)
      {
        if ( self.val & 0x00F0 == 0x00E0 ) // The third 4 bits are 1110 (0xE)
        {
          if ( self.val & 0x000F == 0x0 )
          {
            return OpCodeSymbol::_00E0;
          }
          else if ( self.val & 0x000F == 0x000E )
          {
            return OpCodeSymbol::_00EE;
          }
          else
          {
            return OpCodeSymbol::UNDEF;
          }
        }
        else
        {
          return OpCodeSymbol::UNDEF; // Error
        }
      }
      else
      {
        return OpCodeSymbol::_0NNN;
      }
    }
    else if ( self.val & 0xF000 == 0x1000 )
    {
      return OpCodeSymbol::_1NNN;
    }
    else if ( self.val & 0xF000 == 0x2000 )
    {
      return OpCodeSymbol::_2NNN;
    }
    else if ( self.val & 0xF000 == 0x3000 )
    {
      return OpCodeSymbol::_3XNN;
    }
    else if ( self.val & 0xF000 == 0x4000 )
    {
      return OpCodeSymbol::_4XNN;
    }
    else if ( self.val & 0xF000 == 0x5000 )
    {
      return OpCodeSymbol::_5XY0;
    }
    else if ( self.val & 0xF000 == 0x6000 )
    {
      return OpCodeSymbol::_6XNN;
    }
    else if ( self.val & 0xF000 == 0x7000 )
    {
      return OpCodeSymbol::_7XNN;
    }
    else if ( self.val & 0xF000 == 0x8000 )
    {
      match self.val & 0x000F
      {
        0x0 => { return OpCodeSymbol::_8XY0;},
        0x1 => { return OpCodeSymbol::_8XY1;},
        0x2 => { return OpCodeSymbol::_8XY2;},
        0x3 => { return OpCodeSymbol::_8XY3;},
        0x4 => { return OpCodeSymbol::_8XY4;},
        0x5 => { return OpCodeSymbol::_8XY5;},
        0x6 => { return OpCodeSymbol::_8XY6;},
        0x7 => { return OpCodeSymbol::_8XY7;},
        0xE => { return OpCodeSymbol::_8XYE;},
        _   => { return OpCodeSymbol::UNDEF;},
      }
    }
    else if ( self.val & 0xF000 == 0x9000 )
    {
      return OpCodeSymbol::_9XY0;
    }
    else if ( self.val & 0xF000 == 0xA000 )
    {
      return OpCodeSymbol::_ANNN;
    }
    else if ( self.val & 0xF000 == 0xB000 )
    {
      return OpCodeSymbol::_BNNN;
    }
    else if ( self.val & 0xF000 == 0xC000 )
    {
      return OpCodeSymbol::_CXNN;
    }
    else if ( self.val & 0xF000 == 0xD000 )
    {
      return OpCodeSymbol::_DXYN;
    }
    else if ( self.val & 0xF000 == 0xE000 )
    {
      match self.val & 0x000F
      {
        0xE => { return OpCodeSymbol::_EX9E; },
        0x1 => { return OpCodeSymbol::_EXA1; },
        _   => { return OpCodeSymbol::UNDEF; },
      }
    }
    else if ( self.val & 0xF000 == 0xF000 )
    {
      if ( self.val & 0x000F == 0x7 )
      {
        return OpCodeSymbol::_FX07;
      }
      else if ( self.val & 0x000F == 0xA )
      {
        return OpCodeSymbol::_FX0A;
      }
      else if ( self.val & 0x000F == 0x9 )
      {
        return OpCodeSymbol::_FX29;
      }
      else if ( self.val & 0x000F == 0x3 )
      {
        return OpCodeSymbol::_FX33;
      }
      else if ( self.val & 0x00F0 == 0x10 )
      {
        match self.val & 0x000F
        {
          0x5 => { return OpCodeSymbol::_FX15; },
          0x8 => { return OpCodeSymbol::_FX18; },
          0xE => { return OpCodeSymbol::_FX1E; },
          _   => { return OpCodeSymbol::UNDEF; },
        }
      }
      else if ( self.val & 0x00F0 == 0x0050 )
      {
        return OpCodeSymbol::_FX55;
      }
      else if ( self.val & 0x00F0 == 0x0060 )
      {
        return OpCodeSymbol::_FX65;
      }
      else
      {
        return OpCodeSymbol::UNDEF;
      }
    }
    else
    {
      return OpCodeSymbol::UNDEF;
    }
  }
}

impl fmt::Display for OpCode
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  {
    match self.find_opcode_symbol()
    {
      OpCodeSymbol::UNDEF
          => {write!(f, "UNDEF");},
      OpCodeSymbol::_0NNN 
          => {write!(f,"_0NNN");},
      OpCodeSymbol::_00E0
          => {write!(f, "_00E0");},
      OpCodeSymbol::_00EE
          => {write!(f, "_00EE");},
      OpCodeSymbol::_1NNN
          => {write!(f, "_1NNN");},
      OpCodeSymbol::_2NNN
          => {write!(f, "_2NNN");},
      OpCodeSymbol::_3XNN
          => {write!(f, "_3XNN");},
      OpCodeSymbol::_4XNN
          => {write!(f, "_4XNN");},
      OpCodeSymbol::_5XY0
          => {write!(f, "_5XY0");},
      OpCodeSymbol::_6XNN
          => {write!(f, "_6XNN");},
      OpCodeSymbol::_7XNN
          => {write!(f, "_7XNN");},
      OpCodeSymbol::_8XY0
          => {write!(f, "_8XY0");},
      OpCodeSymbol::_8XY1
          => {write!(f, "_8XY1");},
      OpCodeSymbol::_8XY2
          => {write!(f, "_8XY2");},
      OpCodeSymbol::_8XY3
          => {write!(f, "_8XY3");},
      OpCodeSymbol::_8XY4
          => {write!(f, "_8XY4");},
      OpCodeSymbol::_8XY5
          => {write!(f, "_8XY5");},
      OpCodeSymbol::_8XY6
          => {write!(f, "_8XY6");},
      OpCodeSymbol::_8XY7
          => {write!(f, "_8XY7");},
      OpCodeSymbol::_8XYE
          => {write!(f, "_8XYE");},
      OpCodeSymbol::_9XY0
          => {write!(f, "_9XY0");},
      OpCodeSymbol::_ANNN // Set I to the address NNN
          => {write!(f, "_ANNN");},
      OpCodeSymbol::_BNNN
          => {write!(f, "_BNNN");},
      OpCodeSymbol::_CXNN
          => {write!(f, "_CXNN");},
      OpCodeSymbol::_DXYN
          => {write!(f, "_DXYN");},
      OpCodeSymbol::_EX9E
          => {write!(f, "_EX9E");},
      OpCodeSymbol::_EXA1
          => {write!(f, "_EXA1");},
      OpCodeSymbol::_FX07
          => {write!(f, "_FX07");},
      OpCodeSymbol::_FX0A
          => {write!(f, "_FX0A");},
      OpCodeSymbol::_FX15
          => {write!(f, "_FX15");},
      OpCodeSymbol::_FX18
          => {write!(f, "_FX18");},
      OpCodeSymbol::_FX1E
          => {write!(f, "_FX1E");},
      OpCodeSymbol::_FX29
          => {write!(f, "_FX29");},
      OpCodeSymbol::_FX33
          => {write!(f, "_FX33");},
      OpCodeSymbol::_FX55
          => {write!(f, "_FX55");},
      OpCodeSymbol::_FX65
          => {write!(f, "_FX65");},
    }

    write!(f,"")
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
  fn new() -> Self
  {
    Graphics { gfx:[0x0;64*32]  }
  }

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

impl KeyState
{
  fn new() -> Self
  {
    KeyState { key:[0x0;16], }
  }
}

struct Chip8
{
  draw_flag:    bool,
  regs:         Registers,
  memory:       Memory,
  stack:        Stack,
  curr_opcode:  OpCode,
  graphics:     Graphics,
  keys:         KeyState,
}

impl Chip8
{
  fn new() -> Self
  {
    Chip8 { draw_flag:false,
            regs:Registers::new(),
            memory:Memory::new(),
            stack:Stack::new(),
            curr_opcode:OpCode::new(0x0),
            graphics:Graphics::new(),
            keys:KeyState::new(),
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

      if idx % 2 == 1
      {
        let first_half:u8   = self.memory.memory[0x200 + idx - 1];
        let second_half:u8  = self.memory.memory[0x200 + idx];

        let val:u16 = ((first_half as u16 ) << 8) | (second_half as u16);
        
        if val != 0
        {
          /*println!( "Opcode: {:#X}", val );
          println!( "({},{}):", first_half, second_half );*/

          let opcode = OpCode::new(val);

          println!("{}", opcode);
          //println!("{}", opcode.find_opcode_symbol() as u32 );
        }
      }
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
      OpCodeSymbol::_0NNN =>
      {
        // TODO
      },
      
      OpCodeSymbol::_00E0 =>
      {
        // TODO
        // clear the graphics screen.
      },
      
      OpCodeSymbol::_00EE =>
      {
        // Return from a function call. This is my implementation. 
        self.stack.sp -= 1;
        self.regs.pc_reg = self.stack.stack[self.stack.sp as usize];
        self.regs.pc_reg += 2;
      },
      
      OpCodeSymbol::_1NNN =>
      {
        self.regs.pc_reg = opcode.val & 0x0FFF;
      },
      
      OpCodeSymbol::_2NNN =>
      {
        self.stack.stack[self.stack.sp as usize] = self.regs.pc_reg;
        self.stack.sp += 1;
        self.regs.pc_reg = opcode.val & 0x0FFF;
      },
      
      OpCodeSymbol::_3XNN =>
      {
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let NN        = (opcode.val & 0x00FF); // NN is an 8 bit constant (see Wikipedia for chip8).
        
        if self.regs.gen_purpose_regs[reg_num_X] == (NN as u8) 
        {
          self.regs.pc_reg  += 4;
        }
        else
        {
          self.regs.pc_reg  += 2;
        }
      }
      
      OpCodeSymbol::_4XNN =>
      {
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let NN        = (opcode.val & 0x00FF); // NN is an 8 bit constant (see Wikipedia for chip8).
        
        if self.regs.gen_purpose_regs[reg_num_X] != (NN as u8) 
        {
          self.regs.pc_reg  += 4;
        }
        else
        {
          self.regs.pc_reg  += 2;
        }
      }
      
      OpCodeSymbol::_5XY0 =>
      {
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let reg_num_Y = ((opcode.val & 0x00F0) >> 4) as usize;
        
        if self.regs.gen_purpose_regs[reg_num_X] == self.regs.gen_purpose_regs[reg_num_Y] 
        {
          self.regs.pc_reg  += 4;
        }
        else
        {
          self.regs.pc_reg  += 2;
        }
      }
      
      OpCodeSymbol::_6XNN => /* My implementation */
      {
        // Extract X register nums (for VX) out of the opcode.  
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let NN        = (opcode.val & 0x00FF); // NN is an 8 bit constant (see Wikipedia for chip8).
        
        // V[X] = NN
        self.regs.gen_purpose_regs[reg_num_X] = NN as u8; 
        
        // PC += 2
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_7XNN => /* My implementation */
      {
        // Extract X register nums (for VX) out of the opcode.  
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let NN        = (opcode.val & 0x00FF); // NN is an 8 bit constant (see Wikipedia for chip8).
        
        // V[X] += NN
        self.regs.gen_purpose_regs[reg_num_X] += (NN as u8); 
        
        // PC += 2
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_8XY0 => /* My implementation */
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let reg_num_Y = ((opcode.val & 0x00F0) >> 4) as usize;
        
        // V[X] = V[Y]
        self.regs.gen_purpose_regs[reg_num_X] = self.regs.gen_purpose_regs[reg_num_Y]; 
        
        // PC += 2
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_8XY1 => /* My implementation */
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let reg_num_Y = ((opcode.val & 0x00F0) >> 4) as usize;
        
        // V[X] = V[X] | V[Y] /* Bitwise Or */
        self.regs.gen_purpose_regs[reg_num_X] =  self.regs.gen_purpose_regs[reg_num_X] | 
                                                 self.regs.gen_purpose_regs[reg_num_Y]; 
        
        // PC += 2
        self.regs.pc_reg  += 2;
      },

      OpCodeSymbol::_8XY2 => /* My implementation */
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let reg_num_Y = ((opcode.val & 0x00F0) >> 4) as usize;
        
        // V[X] = V[X] & V[Y] /* Bitwise And */
        self.regs.gen_purpose_regs[reg_num_X] =  self.regs.gen_purpose_regs[reg_num_X] & 
                                                 self.regs.gen_purpose_regs[reg_num_Y]; 
        
        // PC += 2
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_8XY3 => /* My implementation */
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let reg_num_Y = ((opcode.val & 0x00F0) >> 4) as usize;
        
        // V[X] = V[X] ^ V[Y] /* Bitwise Xor */
        self.regs.gen_purpose_regs[reg_num_X] =  self.regs.gen_purpose_regs[reg_num_X] ^ 
                                                 self.regs.gen_purpose_regs[reg_num_Y]; 
        
        // PC += 2
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_8XY4 =>
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let reg_num_Y = ((opcode.val & 0x00F0) >> 4) as usize;
        
        // Set the carry flag (VF) 
        if self.regs.gen_purpose_regs[reg_num_Y] + self.regs.gen_purpose_regs[reg_num_X] >  0xFF
        {
          self.regs.gen_purpose_regs[0xF] = 1;
        }
        else
        {
          self.regs.gen_purpose_regs[0xF] = 0;
        }

        // VX += VY
        self.regs.gen_purpose_regs[reg_num_X] += self.regs.gen_purpose_regs[reg_num_Y];

        // PC += 2
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_8XY5 =>
      {
        // TODO
      },
      
      OpCodeSymbol::_8XY6 =>
      {
        // TODO
      },
      
      OpCodeSymbol::_8XY7 =>
      {
        // TODO
      },
      
      /*
       rcaops ??? - missing command. TODO: need to check
      {
      },
      */

      OpCodeSymbol::_9XY0 =>
      {
        let reg_num_X = ((opcode.val & 0x0F00) >> 8) as usize;
        let reg_num_Y = ((opcode.val & 0x00F0) >> 4) as usize;
        
        if self.regs.gen_purpose_regs[reg_num_X] != self.regs.gen_purpose_regs[reg_num_Y] 
        {
          self.regs.pc_reg  += 4;
        }
        else
        {
          self.regs.pc_reg  += 2;
        }
      }
      
      OpCodeSymbol::_ANNN =>
      {
        self.regs.idx_reg = opcode.val & 0x0FFF;
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_BNNN => /* My implementation */
      {
        let NNN = opcode.val & 0x0FFF;
        self.regs.pc_reg = NNN + (self.regs.gen_purpose_regs[0] as u16); // NNN + V0
      },
      
      OpCodeSymbol::_CXNN =>
      {
        let      NN       = opcode.val & 0x00FF;
        let      X        = ((opcode.val & 0x0F00) >> 8) as usize;
        let mut  rng      = rand::thread_rng();
        let      rnd:u16  = rng.gen(); 

        self.regs.gen_purpose_regs[X] = (rnd & NN) as u8;
        
        // PC += 2
        self.regs.pc_reg  += 2;
      },

      // Display pixel at position(X,Y)
      OpCodeSymbol::_DXYN =>
      {
        let reg_num_X:usize = ((opcode.val & 0x0F00) >> 8) as usize;
        let reg_num_Y:usize = ((opcode.val & 0x00F0) >> 4) as usize;
        let x               = self.regs.gen_purpose_regs[reg_num_X];
        let y               = self.regs.gen_purpose_regs[reg_num_Y];
        let height:u8       = (opcode.val & 0x000F) as u8; 

        // (x,y) holds the position. height as height

        let mut pixel;

        self.regs.gen_purpose_regs[0xF] = 0;

        for yline in 0..height
        {
          pixel = self.memory.memory[(self.regs.idx_reg + (yline as u16)) as usize];

          for xline in 0..8
          {
            if pixel & (0x80 >> xline) != 0
            {
              if self.graphics.gfx[ (x + xline + (( y + yline ) * 64)) as usize ] == 1
              {
                self.regs.gen_purpose_regs[0xF] = 1;
              }
              
              // XOR
              self.graphics.gfx[ (x + xline + (( y + yline ) * 64)) as usize ] ^= 1;
            }
          }
        }

        self.draw_flag = true;
        
        // PC += 2
        self.regs.pc_reg  += 2;
      }
  
      OpCodeSymbol::_EX9E =>
      {
        if self.keys.key[(self.regs.gen_purpose_regs[((opcode.val & 0x0F00) >> 8) as usize]) as usize] != 0
        {
          self.regs.pc_reg  += 4;
        }
        else
        {
          self.regs.pc_reg  += 2;
        }
      }
  
      OpCodeSymbol::_EXA1 =>
      {
        if self.keys.key[(self.regs.gen_purpose_regs[((opcode.val & 0x0F00) >> 8) as usize]) as usize ] == 0
        {
          self.regs.pc_reg  += 4;
        }
        else
        {
          self.regs.pc_reg  += 2;
        }
      }

      OpCodeSymbol::_FX07 =>
      {
        let X:usize = ((opcode.val & 0x0F00) >> 8) as usize;

        self.regs.gen_purpose_regs[X] = self.regs.delay_timer_reg; 
        
        self.regs.pc_reg  += 2;
      },

      OpCodeSymbol::_FX0A =>
      {
        //TODO
      },
      
      OpCodeSymbol::_FX15 =>
      {
        let X:usize = ((opcode.val & 0x0F00) >> 8) as usize;

        self.regs.delay_timer_reg = self.regs.gen_purpose_regs[X]; 
        
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_FX18 =>
      {
        let X:usize = ((opcode.val & 0x0F00) >> 8) as usize;

        self.regs.sound_timer_reg = self.regs.gen_purpose_regs[X]; 
        
        self.regs.pc_reg  += 2;
      },
      
      OpCodeSymbol::_FX1E =>
      {
        //TODO
      },
      
      OpCodeSymbol::_FX29 =>
      {
        //TODO
      },
      
      OpCodeSymbol::_FX33 =>
      {
        let reg_num = ((opcode.val & 0x0F00) >> 8) as usize;

        self.memory.memory[self.regs.idx_reg as usize] = self.regs.gen_purpose_regs[reg_num] / 100;
        self.memory.memory[(self.regs.idx_reg+1) as usize] = (self.regs.gen_purpose_regs[reg_num]/10) % 10;
        self.memory.memory[(self.regs.idx_reg+2) as usize] = (self.regs.gen_purpose_regs[reg_num] % 100) % 10;
        self.regs.pc_reg  += 2;
      },

      OpCodeSymbol::_FX55 =>
      {
        //TODO
      },
      
      OpCodeSymbol::_FX65 =>
      {
        //TODO
      },

      _ =>
      {
        println!("Error parsing opcode !!!!!");
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
