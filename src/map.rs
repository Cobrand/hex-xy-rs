use error::{Error,Reason,Result};
use pos::Position;
use std::vec::Vec ;

pub trait PositionAccessor {
    fn set_position(&mut self,new_position:Position);
    fn get_position(&self) -> Position ;
}

pub trait AllowContent {
    fn is_content_allowed() -> bool ;
}

pub struct Map<T : PositionAccessor,Bg : Clone + Default> {
    contents_slice : Box<[Option<T>]>,
    bg_slice : Box<[Bg]>,
    length: (i32,i32),
    offset: (i32,i32)
}

impl<T,Bg> Map<T,Bg> where T : PositionAccessor, Bg : Clone + Default {
    pub fn new(length:(i32,i32),offset:(i32,i32)) -> Result<Map<T,Bg>> {
        if length.0 <= 0 || length.1 <= 0 {
            Err(Error::new(Reason::NegativeMapLength))
        } else {
            let total_len : usize = length.0 as usize * length.1 as usize ;
            let mut contents_vec : Vec<Option<T>> = Vec::with_capacity(total_len);
            let mut bg_vec : Vec<Bg> = Vec::with_capacity(total_len);
            for i in 0 .. total_len {
                contents_vec.push(None);
                bg_vec.push(Bg::default());
            };
            Ok(Map::<T,Bg> {
                contents_slice:contents_vec.into_boxed_slice(),
                bg_slice:bg_vec.into_boxed_slice(),
                length:length,
                offset:offset
            })
        }
    }

    fn pos_to_index(&self,pos:Position) -> Result<usize> {
        debug_assert!(self.length.0 > 0 && self.length.1 > 0);
        let tmp_pos = pos - Position::from(self.offset) ;
        if tmp_pos.x < 0 || tmp_pos.x >= self.length.0
        || tmp_pos.y < 0 || tmp_pos.y >= self.length.1 {
            Err(Error::new(Reason::OutOfRange))
        } else {
            Ok((tmp_pos.x + self.length.0 * tmp_pos.y) as usize)
        }
    }

    fn index_to_pos(&self,index:usize) -> Result<Position> {
        debug_assert!(self.length.0 > 0 && self.length.1 > 0);
        if index > (self.length.0 * self.length.1) as usize {
            Err(Error::new(Reason::OutOfRange))
        } else {
            let y = index as i32 / self.length.0 ;
            let x = index as i32 % self.length.0 ;
            Ok(Position::new(x,y) + Position::from(self.offset))
        }
    }

    pub fn get(&self,pos:Position) -> Result<(Option<&T>,&Bg)> {
        unimplemented!()
    }

    pub fn get_contents(&self,pos:Position) -> Result<Option<&T>> {
        unimplemented!()
    }

    pub fn get_bg(&self,pos:Position) -> Result<Bg> {
        unimplemented!()
    }

    pub fn change_bg(&mut self,position:Position,bg:Bg){

    }

    pub fn swap_contents(&mut self,pos_1:Position,pos_2:Position) -> Result<()> {
        Ok(())
    }
}


#[test]
pub fn test_index_to_pos(){
    let m = self::tests::sample_map();
    assert_eq!(m.pos_to_index(Position::new(-5,-5)).unwrap(),0);
    assert_eq!(m.pos_to_index(Position::new(-5,-4)).unwrap(),10);
    assert_eq!(m.pos_to_index(Position::new(-4,-4)).unwrap(),11);
}

#[test]
pub fn test_pos_to_index(){
    let m = self::tests::sample_map();
    assert_eq!(m.index_to_pos(0).unwrap(),Position::new(-5,-5));
    assert_eq!(m.index_to_pos(11).unwrap(),Position::new(-4,-4));
}

#[cfg(test)]
mod tests {
    use super::*;
    use pos::Position;
    use std::string::String;
    #[derive(Debug,Clone)]
    pub struct Dummy {
        pos:Position,
        name:String
    }

    #[derive(Debug,Clone,Default)]
    pub struct Bg {
        kind:String
    }

    impl PositionAccessor for Dummy {
        fn set_position(&mut self,new_position:Position) {
            self.pos = new_position;
        }
        fn get_position(&self) -> Position {
            self.pos
        }
    }

    pub fn sample_map() -> Map<Dummy,Bg> {
        Map::new((10,10),(-5,-5)).unwrap()
    }

    #[test]
    fn routine_test(){
        let map : Map<Dummy,Bg> = sample_map();
    }
}
