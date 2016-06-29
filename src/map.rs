use error::{Error,Reason,Result};
use pos::Position;
use std::vec::Vec ;

pub trait PositionAccessor {
    fn set_position(&mut self,new_position:Position);
    fn get_position(&self) -> Position ;
}

pub struct Map<T : PositionAccessor> {
    internal_vec : Vec<T> ,
    length: (i32,i32),
    offset: (i32,i32)
}

impl<T> Map<T> where T : PositionAccessor {
    pub fn new(length:(i32,i32),offset:(i32,i32)) -> Result<Map<T>> {
        if length.0 <= 0 || length.1 <= 0 {
            Err(Error::new(Reason::NegativeMapLength))
        } else {
            let vec : Vec<T> = Vec::with_capacity(length.0 as usize * length.1 as usize);
            Ok(Map::<T> {
                internal_vec:vec,
                length:length,
                offset:offset
            })
        }
    }

    pub fn get(&self,pos:Position) -> Option<&T> {
        None
    }

    pub fn swap(&mut self,pos_1:Position,pos_2:Position) -> Result<()> {
        Ok(())
    }
}
