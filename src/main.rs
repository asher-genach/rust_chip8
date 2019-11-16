use piston_window::*;
use std::{thread, time};
use std::fs::File;
use std::fs;
use std::io::Read;
use std::io::prelude::*;
use std::fmt;
use rand::Rng;

//Colors

const COLOR_BLACK:[f32;4] = [0.0, 0.0, 0.0, 1.0];
const COLOR_RED:  [f32;4] = [1.0, 0.0, 0.0, 1.0];
const COLOR_GREEN:[f32;4] = [0.0, 1.0, 0.0, 1.0];

const SCREEN_WIDTH_PIXELS:usize     = 64;
const SCREEN_HEIGHT_PIXELS:usize    = 32;
const PIXEL_SIZE:usize              = 10;

enum OpCodeSymbol
{
  UNDEF,
  _0NNN,
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
  _ANNN,
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

    write!(f," :Value={:#X}", self.val); 

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

  fn clear(&mut self)
  {
    for idx in 0..4096
    {
      self.memory[idx] = 0x0;
    }
  }
}

struct Registers
{
  V:[u8;16], // V0..VE - 15 8-bit general purpose registers. 16th register carry flag 
  PC:u16, // 0x000-0xFFF
  I:u16,
  DELAY_TIMER:u8,
  SOUND_TIMER:u8,
}

impl Registers
{
  fn new() -> Self
  {
    Registers
    {
      V:[0x0;16],
      PC:0x0,
      I:0x0,
      DELAY_TIMER:0,
      SOUND_TIMER:0
    }
  }

  fn clear(&mut self)
  {
    for idx in 0..16
    {
      self.V[idx] = 0x0;
    }

    self.PC = 0x200; // PC starts at address 0x200

    self.I = 0x0;

    self.DELAY_TIMER = 0x0;

    self.SOUND_TIMER = 0x0;
  }
}

impl fmt::Display for Registers 
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  {
    for idx in 0..16
    {
      if idx % 4 == 0
      {
        write!(f,"\n");
      }

      write!(f, "V[{}]={:#X} ", idx, self.V[idx]);
    }

    write!(f, "\n");

    write!(f, "PC:{:#X}, I:{:#X}, DELAY_TIMER:{:#X}, SOUND_TIMER:{:#X}", self.PC, self.I, self.DELAY_TIMER, self.SOUND_TIMER); 
    
    write!(f, "")
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

  fn clear(&mut self)
  {
    for pixel_idx in 0..64*32 // 2048
    {
      self.gfx[pixel_idx] = 0x0;
    }
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

  fn clear(&mut self)
  {
    for idx in 0..16
    {
      self.stack[idx] = 0x0;
    }

    self.sp = 0x0;
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
  key:          [u8;16], // HEX based 0x0-0xF
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
            key:[0x0;16],
          }
  }

  fn init_fontset(&mut self)
  {
    let chip8_fontset:[u8;80] =
    [ 
      0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
      0x20, 0x60, 0x20, 0x20, 0x70, // 1
      0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
      0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
      0x90, 0x90, 0xF0, 0x10, 0x10, // 4
      0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
      0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
      0xF0, 0x10, 0x20, 0x40, 0x40, // 7
      0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
      0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
      0xF0, 0x90, 0xF0, 0x90, 0x90, // A
      0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
      0xF0, 0x80, 0x80, 0x80, 0xF0, // C
      0xE0, 0x90, 0x90, 0x90, 0xE0, // D
      0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
      0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];
    
    for i in 0..80
    {
      self.memory.memory[i] = chip8_fontset[i];
    }
  }

  fn initialize(&mut self)
  {
    // Initialize registers and memory once.
    self.curr_opcode = OpCode::new(0x0); // Reset current opcode. 

    // Clear stack
    self.stack.clear(); // Reset stack and stack pointer.
    
    // Clear registers V0-VF, PC, I, timers(delay, sound).
    self.regs.clear();
    
    // Clear memory
    self.memory.clear();
 
    // Clear display
    self.graphics.clear();

    // Load fontset
    self.init_fontset();

    // Clear the screen
    self.draw_flag = true;
  }

  fn load_game(&mut self, file_name:&str)
  {
    let     file_size  = fs::metadata(file_name).unwrap().len() as usize;
    let mut file       = File::open(file_name).unwrap();
    let mut buffer     = [0u8;0xDFF];

    println!("File size:{}", file_size);
    
    file.read(&mut buffer).unwrap();

    for idx in 0..file_size/*0xDFF*/
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

          //println!("{}", opcode);
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
  fn fetch_opcode(&mut self)
  {
    let first_half:u8   = self.memory.memory[self.regs.PC as usize];
    let second_half:u8  = self.memory.memory[(self.regs.PC as usize + 1)];

    let val:u16 = ((first_half as u16 ) << 8) | (second_half as u16);

    self.curr_opcode = OpCode::new(val);

    /*println!("{}", self.regs);
    println!("opcode:{}", self.curr_opcode);*/
  }

  fn execute_opcode(&mut self)
  {
    let  opcode_symbol = self.curr_opcode.find_opcode_symbol();

    match opcode_symbol
    {
/*      OpCodeSymbol::_0NNN =>
      {
      }, */
      
      OpCodeSymbol::_00E0 =>
      {
        // clear the graphics screen.
        self.graphics.clear();
        
        self.regs.PC += 2;
        
        self.draw_flag = true;
      },
      
      OpCodeSymbol::_00EE =>
      {
        // Return from a function call. This is my implementation. 
        self.stack.sp -= 1;
        self.regs.PC = self.stack.stack[self.stack.sp as usize];
        self.regs.PC += 2;
      },
      
      OpCodeSymbol::_1NNN =>
      {
        self.regs.PC = self.curr_opcode.val & 0x0FFF;
      },
      
      OpCodeSymbol::_2NNN =>
      {
        self.stack.stack[self.stack.sp as usize] = self.regs.PC;
        self.stack.sp += 1;
        self.regs.PC = self.curr_opcode.val & 0x0FFF;
      },
      
      OpCodeSymbol::_3XNN =>
      {
        let X   = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let NN  = (self.curr_opcode.val & 0x00FF); // NN is an 8 bit constant (see Wikipedia for chip8).
        
        if self.regs.V[X] == (NN as u8) 
        {
          self.regs.PC  += 4;
        }
        else
        {
          self.regs.PC  += 2;
        }
      }
      
      OpCodeSymbol::_4XNN =>
      {
        let X   = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let NN  = (self.curr_opcode.val & 0x00FF); // NN is an 8 bit constant (see Wikipedia for chip8).
        
        if self.regs.V[X] != (NN as u8) 
        {
          self.regs.PC  += 4;
        }
        else
        {
          self.regs.PC  += 2;
        }
      }
      
      OpCodeSymbol::_5XY0 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;
        
        if self.regs.V[X] == self.regs.V[Y] 
        {
          self.regs.PC  += 4;
        }
        else
        {
          self.regs.PC  += 2;
        }
      }
      
      OpCodeSymbol::_6XNN => /* My implementation */
      {
        // Extract X register nums (for VX) out of the opcode.  
        let X    = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let NN   = (self.curr_opcode.val & 0x00FF); // NN is an 8 bit constant (see Wikipedia for chip8).
        
        // V[X] = NN
        self.regs.V[X] = NN as u8; 
        
        // PC += 2
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_7XNN => /* My implementation */
      {
        // Extract X register nums (for VX) out of the opcode.  
        let X    = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let NN   = (self.curr_opcode.val & 0x00FF); // NN is an 8 bit constant (see Wikipedia for chip8).
        
        // V[X] += NN
        //self.regs.V[X] += (NN as u8); 
        self.regs.V[X] = self.regs.V[X].wrapping_add(NN as u8); 
        
        // PC += 2
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_8XY0 => /* My implementation */
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;
        
        // V[X] = V[Y]
        self.regs.V[X] = self.regs.V[Y]; 
        
        // PC += 2
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_8XY1 => /* My implementation */
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;
        
        // V[X] = V[X] | V[Y] /* Bitwise Or */
        self.regs.V[X] =  self.regs.V[X] | 
                                         self.regs.V[Y]; 
        
        // PC += 2
        self.regs.PC  += 2;
      },

      OpCodeSymbol::_8XY2 => /* My implementation */
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;
        
        // V[X] = V[X] & V[Y] /* Bitwise And */
        self.regs.V[X] =  self.regs.V[X] & 
                                         self.regs.V[Y]; 
        
        // PC += 2
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_8XY3 => /* My implementation */
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;
        
        // V[X] = V[X] ^ V[Y] /* Bitwise Xor */
        self.regs.V[X] =  self.regs.V[X] ^ 
                                         self.regs.V[Y]; 
        
        // PC += 2
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_8XY4 =>
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;
        
        // Set the carry flag (VF) 
        if (self.regs.V[Y] as u16) + (self.regs.V[X] as u16)>  0xFF
        {
          self.regs.V[0xF] = 1;
        }
        else
        {
          self.regs.V[0xF] = 0;
        }

        // VX += VY
        //self.regs.V[X] += self.regs.V[Y];
        self.regs.V[X] = self.regs.V[X].wrapping_add(self.regs.V[Y]);

        // PC += 2
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_8XY5 =>
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;

        // Set the carry flag (VF) if there is a borrow 
        if self.regs.V[X] < self.regs.V[Y]
        {
          self.regs.V[0xF] = 0;
        }
        else
        {
          self.regs.V[0xF] = 1;
        }

        // VX -= VY
        // self.regs.V[X] -= self.regs.V[Y];
        self.regs.V[X] = self.regs.V[X].wrapping_sub(self.regs.V[Y]);

        // PC += 2
        self.regs.PC  += 2;

      },
      
      OpCodeSymbol::_8XY6 =>
      {
        // VX >> 1
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        //self.regs.V[0xF] = self.regs.V[X] & 0x80; // 0x80 the first bit 0b10000000
        // shift right
        self.regs.V[0xF] = self.regs.V[X] & 0x1; // 0x1 the first bit 0b10000000

        self.regs.V[X] >>= 1;

        // PC += 2
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_8XY7 =>
      {
        // Extract X and Y register nums (for VX and VY) out of the opcode.  
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;

        // Set the carry flag (VF) if there is a borrow 
        if self.regs.V[X] > self.regs.V[Y]
        {
          self.regs.V[0xF] = 0;
        }
        else
        {
          self.regs.V[0xF] = 1;
        }

        // VX -= VY
        self.regs.V[X] = self.regs.V[Y] - self.regs.V[X];

        // PC += 2
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_8XYE =>
      {
        // VX <<= 1
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        //self.regs.V[0xF] = self.regs.V[X] & 0x01; // 0x80 the lsb 0b00000001
        self.regs.V[0xF] = self.regs.V[X] >> 7;
        self.regs.V[X] <<= 1;

        // PC += 2
        self.regs.PC  += 2;
      },
      

      OpCodeSymbol::_9XY0 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;
        
        if self.regs.V[X] != self.regs.V[Y] 
        {
          self.regs.PC  += 4;
        }
        else
        {
          self.regs.PC  += 2;
        }
      }
      
      OpCodeSymbol::_ANNN =>
      {
        self.regs.I = self.curr_opcode.val & 0x0FFF;
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_BNNN => /* My implementation */
      {
        let NNN = self.curr_opcode.val & 0x0FFF;
        self.regs.PC = NNN + (self.regs.V[0] as u16); // NNN + V0
      },
      
      // Asher - Up to here all the opcodes seem to be implemented correctly
      // ( as in the C++ implementation).
      //
      // TODO: Continue from here
      OpCodeSymbol::_CXNN =>
      {
        let      NN       = self.curr_opcode.val & 0x00FF;
        let      X        = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let mut  rng      = rand::thread_rng();
        let      rnd:u16  = rng.gen(); 

        self.regs.V[X] = ((rnd % 0xFF) & NN) as u8;
        
        // PC += 2
        self.regs.PC  += 2;
      },

      // Display pixel at position(X,Y)
      // Asher: TODO: check this opcode thouhroughly
      OpCodeSymbol::_DXYN =>
      {
        let X       = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        let Y       = ((self.curr_opcode.val & 0x00F0) >> 4) as usize;
        let height  = (self.curr_opcode.val & 0x000F) as u8; 
        let x       = self.regs.V[X];
        let y       = self.regs.V[Y];

        // (x,y) holds the position. height as height

        let mut pixel;

        self.regs.V[0xF] = 0;

        for yline in 0..height
        {
          pixel = self.memory.memory[(self.regs.I + (yline as u16)) as usize];

          for xline in 0..8
          {
            if pixel & (0x80 >> xline) != 0
            {
              let gfx_idx = (((x + xline) as usize) + (( (y + yline) as usize ) * 64)); 

              if self.graphics.gfx[ gfx_idx ] == 1
              {
                self.regs.V[0xF] = 1;
              }
              
              // XOR
              self.graphics.gfx[ gfx_idx ] ^= 1;
            }
          }
        }

        self.draw_flag = true;
        
        // PC += 2
        self.regs.PC  += 2;
      }
  
      OpCodeSymbol::_EX9E =>
      {

        let X  = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;
        
        if self.key[(self.regs.V[X]) as usize] != 0
        {
          self.regs.PC  += 4;
        }
        else
        {
          self.regs.PC  += 2;
        }
      }
  
      OpCodeSymbol::_EXA1 =>
      {
        let X  = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        // loop {};

        if self.key[(self.regs.V[X]) as usize ] == 0
        {
          self.regs.PC  += 4;
        }
        else
        {
          self.regs.PC  += 2;
        }
      }

      OpCodeSymbol::_FX07 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        self.regs.V[X] = self.regs.DELAY_TIMER; 
        
        self.regs.PC  += 2;
      },

      OpCodeSymbol::_FX0A =>
      {
        // get_key()
        let mut key_press = false;

        println!("_FX0A");
        loop {};

        for idx in 0..16
        {
          if self.key[idx] != 0
          {
            let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

            self.regs.V[X] = idx as u8;

            println!("key idx:{} was pushed", idx);

            key_press = true;
          }
        }

        if key_press
        {
          self.regs.PC  += 2;
        }
      },
      
      OpCodeSymbol::_FX15 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        self.regs.DELAY_TIMER = self.regs.V[X]; 
        
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_FX18 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        self.regs.SOUND_TIMER = self.regs.V[X]; 
        
        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_FX1E =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        if self.regs.I + (self.regs.V[X] as u16) > 0xFFF
        {
          self.regs.V[0xF] = 1;
        }
        else
        {
          self.regs.V[0xF] = 0;
        }

        self.regs.I += (self.regs.V[X] as u16);

        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_FX29 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        self.regs.I = (self.regs.V[X] * 0x5) as u16;

        self.regs.PC  += 2;
      },
      
      OpCodeSymbol::_FX33 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8) as usize;

        self.memory.memory[self.regs.I as usize] = self.regs.V[X] / 100;
        self.memory.memory[(self.regs.I+1) as usize] = (self.regs.V[X]/10) % 10;
        self.memory.memory[(self.regs.I+2) as usize] = (self.regs.V[X] % 100) % 10;

        self.regs.PC  += 2;
      },

      OpCodeSymbol::_FX55 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8);

        for idx in 0..X
        {
          self.memory.memory[(self.regs.I + idx) as usize] = self.regs.V[idx as usize];
        }
      
        self.regs.I += (X + 1); 
        
        self.regs.PC  += 2;
      }

      OpCodeSymbol::_FX65 =>
      {
        let X = ((self.curr_opcode.val & 0x0F00) >> 8);

        for idx in 0..X
        {
          self.regs.V[idx as usize] = self.memory.memory[(self.regs.I + idx) as usize];
        }
      
        self.regs.I += (X + 1); 
        
        self.regs.PC  += 2;
      },

      _ =>
      {
        println!("Error parsing opcode !!!!!");
      }
    }
  }

  fn emulate_cycle(&mut self)
  {
    self.fetch_opcode();

    self.execute_opcode();

    // Update timers
    if self.regs.DELAY_TIMER > 0
    {
      self.regs.DELAY_TIMER -= 1;
    }

    if self.regs.SOUND_TIMER > 0
    {
      if self.regs.SOUND_TIMER == 1
      {
        println!("BEEP!");
      }

      self.regs.SOUND_TIMER -= 1;
    }
  }
  
  fn set_keys(&mut self, event:&Event)
  {
    // TODO: Catch Button Events
    if let Some(button) = event.press_args()
    {
      let mut valid_key = true;
      let key;

      match button
      {
        Button::Keyboard(Key::D1) => {self.key[0] = 1; key = 0;},
        Button::Keyboard(Key::D2) => {self.key[1] = 1; key = 1;},
        Button::Keyboard(Key::D3) => {self.key[2] = 1; key = 2;},
        Button::Keyboard(Key::D4) => {self.key[3] = 1; key = 3;},
        Button::Keyboard(Key::Q)  => {self.key[4] = 1; key = 4;},
        Button::Keyboard(Key::W)  => {self.key[5] = 1; key = 5;},
        Button::Keyboard(Key::E)  => {self.key[6] = 1; key = 6;},
        Button::Keyboard(Key::R)  => {self.key[7] = 1; key = 7;},
        Button::Keyboard(Key::A)  => {self.key[8] = 1; key = 8;},
        Button::Keyboard(Key::S)  => {self.key[9] = 1; key = 9;},
        Button::Keyboard(Key::D)  => {self.key[10] = 1; key = 10;},
        Button::Keyboard(Key::F)  => {self.key[11] = 1; key = 11;},
        Button::Keyboard(Key::Z)  => {self.key[12] = 1; key = 12;},
        Button::Keyboard(Key::X)  => {self.key[13] = 1; key = 13;},
        Button::Keyboard(Key::C)  => {self.key[14] = 1; key = 14;},
        Button::Keyboard(Key::V)  => {self.key[15] = 1; key = 15;},
         _ => { valid_key = false; key = 99; },
      }

      if valid_key
      {
        for idx in 0..16
        {
          if idx != key
          {
            self.key[idx] = 0;
          }
        }
      }

      println!("Key:{}", key);
    }
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
    Emulator{ window:WindowSettings::new("CHIP-8 Emulator",
                [(SCREEN_WIDTH_PIXELS*PIXEL_SIZE) as u32,
                 (SCREEN_HEIGHT_PIXELS*PIXEL_SIZE) as u32])
                .exit_on_esc(true).build().unwrap(),
                chip8: Chip8::new() }
  }

  fn setup(&mut self, game_name:&str)
  {
    self.chip8.initialize();
    
    self.chip8.load_game( game_name );
  }
  
  fn display(&mut self, context:&Context, graphics:&mut G2d)
  {
    for x_idx in 0..64
    {
      for y_idx in 0..32
      {
        let color:[f32;4];
          
        if self.chip8.graphics.gfx[y_idx*64 + x_idx] == 0x1
        {
          color = COLOR_RED;
        }
        else
        {
          color = COLOR_BLACK;
        }
          
        rectangle( color,
                   [1.0*((x_idx*PIXEL_SIZE) as f64),
                    1.0*((y_idx*PIXEL_SIZE) as f64),
                    1.0*(((x_idx*PIXEL_SIZE) + PIXEL_SIZE) as f64),
                    1.0*(((y_idx*PIXEL_SIZE) + PIXEL_SIZE) as f64)],
                    context.transform,
                    graphics );
      }
    }
  }

  fn draw_graphics(&mut self, event:&Event)
  {
    // gfx is in chip8, but the piston graphics window
    // is in the emulator.
    // draw_graphics() needs to inquire the chip8 gfx matrix
    // and draw it into the piston window.
    let ref mut _chip8 = &mut self.chip8;
    
    self.window.draw_2d(event,
                        |context, graphics|
                        {  
                          for x_idx in 0..64
                          {
                            for y_idx in 0..32
                            {
                              let color:[f32;4];
          
                              if _chip8.graphics.gfx[y_idx*64 + x_idx] == 0x1
                              {
                                color = COLOR_RED;
                              }
                              else
                              {
                                color = COLOR_BLACK;
                              }
          
                              rectangle( color,
                                         [1.0*((x_idx*PIXEL_SIZE) as f64),
                                          1.0*((y_idx*PIXEL_SIZE) as f64),
                                          1.0*(((x_idx*PIXEL_SIZE) + PIXEL_SIZE) as f64),
                                          1.0*(((y_idx*PIXEL_SIZE) + PIXEL_SIZE) as f64)],
                                         context.transform,
                                         graphics );
                            }
                          }
                        });

    self.chip8.draw_flag = false;
  }

  fn main_loop(&mut self, cycle_limit:u64)
  {
    let mut num_cycle:u64 = 0;

    while let Some(event) = self.window.next()
    {
      self.chip8.set_keys(&event);

      num_cycle += 1;

      if cycle_limit < 1000 && num_cycle > cycle_limit
      {
      }
      else
      {
        self.chip8.emulate_cycle();
      }
      
      // Catch Window CloseEvent 
      if let Some(_) = event.close_args()
      {
        break;
      }
      
      /*self.window.draw_2d( &event,
                           |context, graphics|
                           { 
                             clear([1.0; 4], graphics);
                           }); */

      if self.chip8.draw_flag
      {
        self.draw_graphics(&event);
      }

    }
  }

  fn start(&mut self, game_name:&str)
  {
    self.setup( game_name );

    self.main_loop(1000);
  }
}

fn main()
{ 
  Emulator::new().start("pong.rom");
  // Emulator::new().start("guess.rom");
  //Emulator::new().start("ttto.rom");
  //Emulator::new().start("flipc.rom");

  // thread::sleep(time::Duration::from_millis(3000));
}
