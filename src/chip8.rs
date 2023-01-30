use super::error::Error;
use super::Result;
use lazy_static::lazy_static;
use rand::{self, Rng};
use spin;
use std::{fs::File, io::Read};

type Byte = u8;
type Word = u16;

static mut GAMEMEMEORY: [Byte; 0x1000] = [0; 0x1000];
static mut REGISTER: [Byte; 16] = [0; 16];
static mut ADDRESS_I: Word = 0;
static mut PROGCOUNTER: Word = 0;
lazy_static! {
    static ref STACK: spin::Mutex<Vec<Word>> = spin::Mutex::new(Vec::new());
}
static mut SCREENDATA: [[Byte; 32]; 64] = [[0; 32]; 64];
static mut DELAY_TIMER: Byte = 60;
static mut SOUND_TIMER: Byte = 60;
static mut KEY: [Byte; 0x10] = [0; 0x10];
static mut CLEARFLAG: bool = false;

pub fn cpu_reset(path: String) -> Result<()> {
    unsafe {
        ADDRESS_I = 0;
        PROGCOUNTER = 0x200;
        REGISTER = [0; 16];
        delay_timer();
        sound_timer();
        set_sprite();
    }

    let mut file = File::open(path)?;
    unsafe {
        file.read(&mut GAMEMEMEORY[0x200..])?;
    }
    Ok(())
}

pub unsafe fn execute() -> Result<()> {
    let code1 = GAMEMEMEORY[PROGCOUNTER as usize];
    let code2 = GAMEMEMEORY[PROGCOUNTER as usize + 1];
    let opcode = (code1 as u16) * 256 + code2 as Word;
    PROGCOUNTER += 2;

    match code1 >> 4 {
        0 => match code2 {
            0xE0 => opcode_00e0(),
            0xEE => opcode_00ee()?,
            _ => opcode_0nnn(opcode),
        },

        1 => opcode_1nnn(opcode),
        2 => opcode_2nnn(opcode),
        3 => opcode_3xnn(opcode),
        4 => opcode_4xnn(opcode),
        5 => opcode_5xy0(opcode),
        6 => opcode_6xnn(opcode),
        7 => opcode_7xnn(opcode),
        8 => match code2 & 0x0F {
            0 => opcode_8xy0(opcode),
            1 => opcode_8xy1(opcode),
            2 => opcode_8xy2(opcode),
            3 => opcode_8xy3(opcode),
            4 => opcode_8xy4(opcode),
            5 => opcode_8xy5(opcode),
            6 => opcode_8xy6(opcode),
            7 => opcode_8xy7(opcode),
            0xE => opcode_8xye(opcode),
            _ => unimplemented!(), //todo: Err(e) after implementation of my own Error
        },
        9 => opcode_9xy0(opcode),
        0xA => opcode_annn(opcode),
        0xB => opcode_bnnn(opcode),
        0xC => opcode_cxnn(opcode),
        0xD => opcode_dxyn(opcode),
        0xE => match code2 {
            0x9E => opcode_ex9e(opcode),
            0xA1 => opcode_exa1(opcode),
            _ => unimplemented!(),
        },
        0xF => match code2 {
            0x07 => opcode_fx07(opcode),
            0x0A => opcode_fx0a(opcode),
            0x15 => opcode_fx15(opcode),
            0x18 => opcode_fx18(opcode),
            0x1E => opcode_fx1e(opcode),
            0x29 => opcode_fx29(opcode),
            0x33 => opcode_fx33(opcode),
            0x55 => opcode_fx55(opcode),
            0x65 => opcode_fx65(opcode),
            _ => unimplemented!(),
        },
        _ => (),
    }
    Ok(())
}

unsafe fn set_sprite() {
    //0
    GAMEMEMEORY[0x50] = 0xF0;
    GAMEMEMEORY[0x51] = 0x90;
    GAMEMEMEORY[0x52] = 0x90;
    GAMEMEMEORY[0x53] = 0x90;
    GAMEMEMEORY[0x54] = 0xF0;
    //1
    GAMEMEMEORY[0x55] = 0x20;
    GAMEMEMEORY[0x56] = 0x60;
    GAMEMEMEORY[0x57] = 0x20;
    GAMEMEMEORY[0x58] = 0x20;
    GAMEMEMEORY[0x59] = 0x70;
    //2
    GAMEMEMEORY[0x5A] = 0xF0;
    GAMEMEMEORY[0x5B] = 0x10;
    GAMEMEMEORY[0x5C] = 0xF0;
    GAMEMEMEORY[0x5D] = 0x80;
    GAMEMEMEORY[0x5E] = 0xF0;
    //3
    GAMEMEMEORY[0x5F] = 0xF0;
    GAMEMEMEORY[0x60] = 0x10;
    GAMEMEMEORY[0x61] = 0xF0;
    GAMEMEMEORY[0x62] = 0x10;
    GAMEMEMEORY[0x63] = 0xF0;
    //4
    GAMEMEMEORY[0x64] = 0x90;
    GAMEMEMEORY[0x65] = 0x90;
    GAMEMEMEORY[0x66] = 0xF0;
    GAMEMEMEORY[0x67] = 0x10;
    GAMEMEMEORY[0x68] = 0x10;
    //5
    GAMEMEMEORY[0x69] = 0xF0;
    GAMEMEMEORY[0x6A] = 0x80;
    GAMEMEMEORY[0x6B] = 0xF0;
    GAMEMEMEORY[0x6C] = 0x10;
    GAMEMEMEORY[0x6D] = 0xF0;
    //6
    GAMEMEMEORY[0x6E] = 0xF0;
    GAMEMEMEORY[0x6F] = 0x80;
    GAMEMEMEORY[0x70] = 0xF0;
    GAMEMEMEORY[0x71] = 0x90;
    GAMEMEMEORY[0x72] = 0xF0;
    //7
    GAMEMEMEORY[0x73] = 0xF0;
    GAMEMEMEORY[0x74] = 0x10;
    GAMEMEMEORY[0x75] = 0x22;
    GAMEMEMEORY[0x76] = 0x40;
    GAMEMEMEORY[0x77] = 0x40;
    //8
    GAMEMEMEORY[0x78] = 0xF0;
    GAMEMEMEORY[0x79] = 0x90;
    GAMEMEMEORY[0x7A] = 0xF0;
    GAMEMEMEORY[0x7B] = 0x90;
    GAMEMEMEORY[0x7C] = 0xF0;
    //9
    GAMEMEMEORY[0x7D] = 0xF0;
    GAMEMEMEORY[0x7E] = 0x90;
    GAMEMEMEORY[0x7F] = 0xF0;
    GAMEMEMEORY[0x80] = 0x10;
    GAMEMEMEORY[0x81] = 0xF0;
    //A
    GAMEMEMEORY[0x82] = 0xF0;
    GAMEMEMEORY[0x83] = 0x90;
    GAMEMEMEORY[0x84] = 0xF0;
    GAMEMEMEORY[0x85] = 0x90;
    GAMEMEMEORY[0x86] = 0x90;
    //B
    GAMEMEMEORY[0x87] = 0xE0;
    GAMEMEMEORY[0x88] = 0x90;
    GAMEMEMEORY[0x89] = 0xE0;
    GAMEMEMEORY[0x8A] = 0x90;
    GAMEMEMEORY[0x8B] = 0xE0;
    //C
    GAMEMEMEORY[0x8C] = 0xE0;
    GAMEMEMEORY[0x8D] = 0x90;
    GAMEMEMEORY[0x8E] = 0xE0;
    GAMEMEMEORY[0x8F] = 0x90;
    GAMEMEMEORY[0x90] = 0xE0;
    //D
    GAMEMEMEORY[0x91] = 0xE0;
    GAMEMEMEORY[0x92] = 0x90;
    GAMEMEMEORY[0x93] = 0x90;
    GAMEMEMEORY[0x94] = 0x90;
    GAMEMEMEORY[0x95] = 0xE0;
    //E
    GAMEMEMEORY[0x96] = 0xF0;
    GAMEMEMEORY[0x97] = 0x80;
    GAMEMEMEORY[0x98] = 0xF0;
    GAMEMEMEORY[0x99] = 0x80;
    GAMEMEMEORY[0x9A] = 0xF0;
    //F
    GAMEMEMEORY[0x9B] = 0xF0;
    GAMEMEMEORY[0x9C] = 0x80;
    GAMEMEMEORY[0x9D] = 0xF0;
    GAMEMEMEORY[0x9E] = 0x80;
    GAMEMEMEORY[0x9F] = 0x80;
}

pub unsafe fn delay_timer() {
    std::thread::spawn(|| loop {
        if DELAY_TIMER == 0 {
            DELAY_TIMER = 60;
        } else {
            DELAY_TIMER -= 1;
        }
        let dura = std::time::Duration::from_millis(16);
        std::thread::sleep(dura);
    });
}

pub unsafe fn sound_timer() {
    std::thread::spawn(|| loop {
        if SOUND_TIMER == 0 {
            SOUND_TIMER = 60;
        } else {
            SOUND_TIMER -= 1;
        }
        let dura = std::time::Duration::from_millis(16);
        std::thread::sleep(dura);
    });
}

//Unlike 8086, chip8 store higher bits in lower address
//and store lower ibts in higher address
//like 0x1234, the 0x12 will store in 0x200
// and the 0x34 will store in 0x201
// (It called Big-endian?)
pub fn get_nexe_opcode() -> Word {
    let mut res = unsafe { GAMEMEMEORY[PROGCOUNTER as usize] as Word };
    // shifit lower bits into higer bits => 0x12->0x1200
    res <<= 8;
    unsafe {
        res |= GAMEMEMEORY[PROGCOUNTER as usize + 1] as u16;
        PROGCOUNTER += 2;
    }
    res
}
pub fn opcode_0nnn(opcode: Word) {
    unsafe { PROGCOUNTER = opcode }
}

//00e0 - clear the screen
pub fn opcode_00e0() {
    unsafe {
        for i in 0..64 {
            for j in 0..32 {
                SCREENDATA[i][j] = 0;
            }
        }
        CLEARFLAG = true;
    }
}

//00EE - return from a subroutine
pub fn opcode_00ee() -> Result<()> {
    unsafe {
        PROGCOUNTER = STACK
            .lock()
            .pop()
            .ok_or(Error::Opcode("00EE".to_string()))?;
    }
    Ok(())
}

//1NNN is the opcode for jump instruction.
pub fn opcode_1nnn(opcode: Word) {
    unsafe {
        PROGCOUNTER = opcode & 0x0FFF;
    }
}

//2NNN - Call subroutine at NNN
pub fn opcode_2nnn(opcode: Word) {
    unsafe {
        STACK.lock().push(PROGCOUNTER);
        PROGCOUNTER = opcode & 0xFFF;
    }
}

//3XNN - if Vx == NN, then skip the next instruction.
pub fn opcode_3xnn(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let nn = (opcode & 0x00FF) as u8;
    unsafe {
        if REGISTER[regx as usize] == nn {
            PROGCOUNTER += 2;
        }
    }
}

//4XNN - if Vx != NN, then skip the next instruction
pub fn opcode_4xnn(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let nn = (opcode & 0x00FF) as u8;
    unsafe {
        if REGISTER[regx as usize] != nn {
            PROGCOUNTER += 2;
        }
    }
}

//5xy0 - if x==y,then skip the next instruction.
pub fn opcode_5xy0(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        if REGISTER[regx as usize] == REGISTER[regy as usize] {
            PROGCOUNTER += 2;
        }
    }
}

// 6XNN - Set Vx = NN
pub fn opcode_6xnn(opcode: Word) {
    let regx = (opcode & 0xF00) >> 8;
    let nn = (opcode & 0x00FF) as Byte;
    unsafe {
        REGISTER[regx as usize] = nn;
    }
}

// 7XNN -  Vx += NN, carry flag is not changed
pub fn opcode_7xnn(opcode: Word) {
    let regx = (opcode & 0xF00) >> 8;
    let nn = (opcode & 0x00FF) as Byte;
    unsafe {
        REGISTER[regx as usize] = REGISTER[regx as usize].wrapping_add(nn);
    }
}

//8XY0 - Vx = Vy
pub fn opcode_8xy0(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        REGISTER[regx as usize] = REGISTER[regy as usize];
    }
}

//8XY1 - Vx |= Vy
pub fn opcode_8xy1(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        REGISTER[regx as usize] |= REGISTER[regy as usize];
    }
}

//8XY2 - Vx &= Vy
pub fn opcode_8xy2(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        REGISTER[regx as usize] &= REGISTER[regy as usize];
    }
}

//8XY3 - Vx ^= Vy
pub fn opcode_8xy3(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        REGISTER[regx as usize] ^= REGISTER[regy as usize];
    }
}

//8XY4 - Vx += Vy, set carry flag to 1 if overflow ,otherwise 0.
pub fn opcode_8xy4(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        let (result, carry) = REGISTER[regx as usize].overflowing_add(REGISTER[regy as usize]);
        REGISTER[regx as usize] = result;
        REGISTER[0xF] = carry.into();
    }
}

//8XY5 - Vx -= Vy, set carry flag to 0 if borrow, otherwise 1.
pub fn opcode_8xy5(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        let tmp = 1u8 - (REGISTER[regx as usize] < REGISTER[regy as usize]) as Byte;
        REGISTER[regx as usize] = REGISTER[regx as usize].wrapping_sub(REGISTER[regy as usize]);
        REGISTER[0xF] = tmp;
    }
}

//8XY6 - Vx >>=1, store the least significant bit into VF.
pub fn opcode_8xy6(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    unsafe {
        REGISTER[0xF] = REGISTER[regx as usize] & 0x01;
        REGISTER[regx as usize] >>= 1
    }
}

//8XY7 - Vx = Vy - Vx, set carry flag to 0 if borrowed, otherwise 1.
pub fn opcode_8xy7(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        let tmp = 1u8 - (REGISTER[regx as usize] > REGISTER[regy as usize]) as Byte;
        REGISTER[regx as usize] = REGISTER[regy as usize].wrapping_sub(REGISTER[regx as usize]);
        REGISTER[0xF] = tmp;
    }
}

//8XYE - Vx =  Vx<<1, store the most significant bit to VF.
pub fn opcode_8xye(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    unsafe {
        REGISTER[0xF] = (REGISTER[regx as usize] & 0x80) >> 7;
        REGISTER[regx as usize] <<= 1
    }
}

//9xy0 - if Vx!=Vy,then skip the next instruction.
pub fn opcode_9xy0(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    unsafe {
        if REGISTER[regx as usize] != REGISTER[regy as usize] {
            PROGCOUNTER += 2;
        }
    }
}

//ANNN - Set address I to the address NNN
pub fn opcode_annn(opcode: Word) {
    let address = opcode & 0xFFF;
    unsafe {
        ADDRESS_I = address;
    }
}

//BNNN - Jump to the address NN plus V0
pub fn opcode_bnnn(opcode: Word) {
    let nnn = opcode & 0xFFF;
    unsafe {
        PROGCOUNTER = REGISTER[0] as Word + nnn;
    }
}

//CXNN - Vx - rand()&NN
pub fn opcode_cxnn(opcode: Word) {
    let mut rng = rand::thread_rng();
    let r: Byte = rng.gen::<Byte>();
    let regx = (opcode & 0x0F00) >> 8;
    let nn = (opcode & 0x00FF) as Byte;
    unsafe { REGISTER[regx as usize] = r & nn }
}

//DXYN - draw(Vx,Vy,N)
pub fn opcode_dxyn(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let regy = (opcode & 0x00F0) >> 4;
    let height = (opcode & 0x000F) as Byte;
    let coord_x = unsafe { REGISTER[regx as usize] };
    let coord_y = unsafe { REGISTER[regy as usize] };
    unsafe {
        REGISTER[0xF] = 0;
    }
    for yline in 0..height {
        let data: Byte = unsafe { GAMEMEMEORY[(ADDRESS_I + yline as Word) as usize] };
        for xpix in 0..8 {
            let mask = 1 << (7 - xpix);
            if (data & mask) != 0 {
                let mut x = coord_x + xpix;
                let mut y = coord_y + yline;
                if y > 31 {
                    y = 31;
                }
                if x > 63 {
                    x = 63;
                }

                unsafe {
                    if SCREENDATA[x as usize][y as usize] == 1 {
                        REGISTER[0xF] = 1;
                    } else {
                    }

                    SCREENDATA[x as usize][y as usize] ^= 1;
                }
            }
        }
    }
}

//EX9E - if key() == Vx, skip the next instruction.
pub fn opcode_ex9e(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let key = get_key_pressed();
    unsafe {
        if key == REGISTER[regx as usize] {
            PROGCOUNTER += 2;
        }
    }
}

//EXA1 - if key() != Vx, skip the next instruction.
pub fn opcode_exa1(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let key = get_key_pressed();
    unsafe {
        if key != REGISTER[regx as usize] {
            PROGCOUNTER += 2;
        }
    }
}

//FX07 - Vx = get_delay()
pub fn opcode_fx07(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    unsafe {
        REGISTER[regx as usize] = DELAY_TIMER;
    }
}

//FX0A - Vx = get_key()
pub fn opcode_fx0a(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    unsafe {
        let key = get_key_pressed();
        if key == 0xFF {
            PROGCOUNTER -= 2;
        } else {
            REGISTER[regx as usize] = key;
        }
    }
}

//FX15 - set the delay timer to vx
pub fn opcode_fx15(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    unsafe {
        DELAY_TIMER = REGISTER[regx as usize];
    }
}

//FX18 - set the delay timer to vx
pub fn opcode_fx18(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    unsafe {
        SOUND_TIMER = REGISTER[regx as usize];
    }
}

//FX1E - ADDRESS_I += Vx
pub fn opcode_fx1e(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    unsafe {
        ADDRESS_I += REGISTER[regx as usize] as Word;
    }
}

//FX29 - I = sprtie_addr[Vx]
pub fn opcode_fx29(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    unsafe {
        ADDRESS_I = (REGISTER[regx as usize] * 5 + 0x50) as Word;
    }
}

//FX33 - Binary-coded decimal
pub fn opcode_fx33(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    let value = unsafe { REGISTER[regx as usize] };
    let hundreds = value / 100;
    let tens = (value / 10) % 10;
    let units = value % 10;

    unsafe {
        GAMEMEMEORY[ADDRESS_I as usize] = hundreds;
        GAMEMEMEORY[ADDRESS_I as usize + 1] = tens;
        GAMEMEMEORY[ADDRESS_I as usize + 2] = units;
    }
}

// FX55 - MEMORY[I..] = REGISTER[0..Vx]
pub fn opcode_fx55(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    for i in 0..=regx {
        unsafe {
            GAMEMEMEORY[ADDRESS_I as usize] = REGISTER[i as usize];
            ADDRESS_I += 1;
        }
    }
}

// FX65 -   REGISTER[0..Vx] = MEMORY[I..]
pub fn opcode_fx65(opcode: Word) {
    let regx = (opcode & 0x0F00) >> 8;
    for i in 0..=regx {
        unsafe {
            REGISTER[i as usize] = GAMEMEMEORY[ADDRESS_I as usize];
            ADDRESS_I += 1;
        }
    }
}

fn get_key_pressed() -> Byte {
    let mut key = 0xFF;
    unsafe {
        for i in 0..=0xF {
            if KEY[i] == 1 {
                key = i as Byte;
                break;
            }
        }
    }
    key
}

// exposed api for keys.
pub fn key_pressed(key_code: Byte) {
    unsafe {
        KEY[key_code as usize] = 1;
    }
}

pub fn key_released(key_code: Byte) {
    unsafe {
        KEY[key_code as usize] = 0;
    }
}

// exposed api for viewing SCREENDATA
pub fn view(x: usize, y: usize) -> Byte {
    unsafe { SCREENDATA[x][y] }
}

pub fn get_clear_flag() -> bool {
    unsafe {
        let a = CLEARFLAG;
        CLEARFLAG = false;
        a
    }
}

#[cfg(test)]
mod test {
    use super::{Byte, SCREENDATA};

    #[test]
    fn test_00e0() {
        unsafe {
            for i in 0..64 {
                for j in 0..32 {
                    SCREENDATA[i][j] = (i * j) as Byte;
                }
            }
        }
        super::opcode_00e0();
        unsafe {
            for i in 0..64 {
                for j in 0..32 {
                    assert_eq!(SCREENDATA[i][j], 0);
                }
            }
        }
    }

    #[test]
    fn test_call_and_return() {
        super::opcode_1nnn(0x200);
        super::opcode_2nnn(0x202);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x202);
            assert_eq!(*super::STACK.lock().last().unwrap(), 0x200);
        }
        super::opcode_00ee().unwrap();
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x200);
        }
    }

    #[test]
    fn test_cond_xnn() {
        super::opcode_1nnn(0x200);

        unsafe {
            super::REGISTER[0] = 0x11;
        }
        super::opcode_3xnn(0x3011);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x202);
        }
        super::opcode_4xnn(0x4011);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x202);
        }

        super::opcode_3xnn(0x3001);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x202);
        }
        super::opcode_4xnn(0x4001);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x204);
        }
    }

    #[test]
    fn test_cond_xy() {
        super::opcode_1nnn(0x200);

        unsafe {
            super::REGISTER[0] = 0x11;
            super::REGISTER[1] = 0x11;
            super::REGISTER[2] = 0x01;
        }
        super::opcode_5xy0(0x5010);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x202);
        }
        super::opcode_9xy0(0x9010);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x202);
        }

        super::opcode_5xy0(0x5020);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x202);
        }
        super::opcode_9xy0(0x9020);
        unsafe {
            assert_eq!(super::PROGCOUNTER, 0x204);
        }
    }

    #[test]
    fn test_const() {
        unsafe {
            super::REGISTER[0] = 0x11;
            super::opcode_6xnn(0x6022);
            assert_eq!(super::REGISTER[0], 0x22);

            super::opcode_7xnn(0x7011);
            assert_eq!(super::REGISTER[0], 0x33);
        }
    }

    #[test]
    fn test_assign() {
        unsafe {
            super::REGISTER[0] = 0x11;
            super::REGISTER[2] = 0x23;
            super::opcode_8xy0(0x8020);
            assert_eq!(super::REGISTER[0], 0x23);
        }
    }

    #[test]
    fn test_bitop() {
        unsafe {
            super::REGISTER[0] = 0xAA;
            super::REGISTER[1] = 0xA5;
            super::opcode_8xy1(0x8011);
            assert_eq!(super::REGISTER[0], 0xAA | 0xA5); //0xAF
            super::opcode_8xy2(0x8012);
            assert_eq!(super::REGISTER[0], 0xAF & 0xA5); //0xA5
            super::opcode_8xy3(0x8013);
            assert_eq!(super::REGISTER[0], 0xA5 ^ 0xA5); //0x00
            super::opcode_8xy6(0x8016);
            assert_eq!(super::REGISTER[0], 0x00);

            super::REGISTER[0] = 0xA5;
            super::opcode_8xy6(0x8016);
            assert_eq!(super::REGISTER[0], 0x52);
            assert_eq!(super::REGISTER[0xF], 0x1);
            super::opcode_8xye(0x801e);
            assert_eq!(super::REGISTER[0], 0xA4);
            assert_eq!(super::REGISTER[0xF], 0x0);
        }
    }

    #[test]
    fn test_math() {
        unsafe {
            super::REGISTER[0] = 0xFF;
            super::REGISTER[1] = 0x11;

            super::opcode_8xy4(0x8014);
            assert_eq!(super::REGISTER[0], 0x10);
            assert_eq!(super::REGISTER[0xF], 0x1);
            super::opcode_8xy4(0x8014);
            assert_eq!(super::REGISTER[0], 0x21);
            assert_eq!(super::REGISTER[0xF], 0x0);

            super::opcode_8xy5(0x8015);
            assert_eq!(super::REGISTER[0], 0x10);
            assert_eq!(super::REGISTER[0xF], 0x1);
            super::opcode_8xy5(0x8015);
            assert_eq!(super::REGISTER[0], 0xFF);
            assert_eq!(super::REGISTER[0xF], 0x0);

            super::opcode_8xy7(0x8017);
            assert_eq!(super::REGISTER[0], 0x12);
            assert_eq!(super::REGISTER[0xF], 0x0);
        }
    }
}
