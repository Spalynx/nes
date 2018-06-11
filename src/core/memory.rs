//For now, memory is a 2kb sized 8 bit array.
//This might not ever change, but it seems a tad too simplistic.
pub struct MEM {
    values:	[u8; 0x800], //2kb internal RAM.
}

#[allow(dead_code)]
impl MEM {
//Initializes an empty memory struct.
    pub fn new() -> MEM {
        return MEM {
            values:	[0; 0x800],
        }
    }

    //Obtains values from full memory map.
    pub fn get(&self, address: u16) -> u8 {
        if address >= 0x00 && address <= 0x800{
            //2kb internal ram
            self.values[address as usize]
        }
        else {
            panic!("Other values in the memory map not implemented yet!");
        }
    }

    //Much faster, only has to access the first page of memory.
    pub fn get_zp(&self, address: u8) -> u8 {
        if address > 255 {
            panic!("Went over!");
        }

        self.values[address as usize]
    }

    // block any illegal storing.
    pub fn set(&mut self, address: u16, val: u8){
        if address >= 0x00 && address <= 0x800 {
            //2kb internal ram
            self.values[address as usize] = val;
        }
        else {
            panic!("Other values in the memory map not implemented yet!");
        }
    }
    //Sets a value in the zero page.
    //Much faster, only has to access the first page of memory.
    pub fn set_zp(&mut self, address: u8, val: u8) {
        if address > 255 {
            panic!("Went over!");
        }

        self.values[address as usize] = val;
    }
}


/*Memory locations $200 to $5ff map to the screen pixels. Different values will
draw different colour pixels. The colours are:

$0: Black
$1: White
$2: Red
$3: Cyan
$4: Purple
$5: Green
$6: Blue
$7: Yellow
$8: Orange
$9: Brown
$a: Light red
$b: Dark grey
$c: Grey
$d: Light green
$e: Light blue
$f: Light grey

I think this might be easy 6502 specific...
 */
//JESUS CHRIST, why did they have to make it all confusing and call the High Order Bytes pages?
//Now Zero Paging seems really fukken obvious!
