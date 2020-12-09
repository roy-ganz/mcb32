#[derive(Debug)]
pub struct Shifter<'a> {
    bit_pos: usize,
    bytes: &'a mut [u8]
}
    
impl<'a> Shifter<'a>{
    pub fn new(slice: &'a mut [u8]) -> Shifter<'a> {
        
        Shifter {
            bit_pos: 0,
            bytes: slice
            
        }
    }

    pub fn clear(&mut self) {
        self.bit_pos = 0;
    }
    pub fn capacity(&self) -> usize {
        self.bytes.len()
    }
    pub fn len(&self) ->usize {
        let (index, bits) = self.positions();
        index + if bits == 0 {0} else {1}
    }
    
    pub fn is_aligned(&self) ->bool {
        self.bit_pos & 7 == 0
    }
    pub fn as_slice(&'a mut self) -> &'a[u8] {
        // delete remaining bits
        let (index, bits) = self.positions();
        if bits != 0 {
            let mask = 0xff  << (7 - bits);
            self.bytes[index] &= mask;
            &self.bytes[0..=index]
        } else {
            &self.bytes[0..index]
        }
    }
   
    pub fn shift_bool(&mut self, value: bool) {
        if value {
            let (index, bits) = self.positions();
            self.bytes[index] |= 1 << (7 - bits);    
        } 
       self.bit_pos += 1;
     }
     pub fn shift_u8(&mut self, value: u8) {
        let (index, bits) = self.positions();
        let hi = value >> bits;
        self.bytes[index] |= hi;
        
        if bits != 0 {
            let lo = value << (8 - bits) ;
            self.bytes[index+1 ] |= lo;    
        }
        self.bit_pos += 8;
     }
     
     fn positions(&self) -> (usize, usize) {
         ( self.bit_pos >> 3, self.bit_pos & 7)
     }
}
