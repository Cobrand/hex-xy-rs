use error::{Error,Reason};
use std::vec::Vec ;

pub struct Map<Object> {
    internal_vec : Vec<Object> ,
    length: (i32,i32),
    offset: (i32,i32)
}

impl<Object> Map<Object> {
    pub fn new(length:(i32,i32),offset:(i32,i32)) -> Result<Map<Object>,Error> {
        if length.0 <= 0 || length.1 <= 0 {
            Err(Error::new(Reason::NegativeMapLength))
        } else {
            let vec  : Vec<Object> = Vec::with_capacity(length.0 as usize * length.1 as usize);
            Ok(Map::<Object> {
                internal_vec:vec,
                length:length,
                offset:offset
            })
        }
    }
}
