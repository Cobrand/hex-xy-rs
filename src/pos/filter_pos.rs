use std::vec::Vec ;
use pos::*;
use std::cmp::{max,min};
impl Position {
    /// Positions from top-left are added first, to top-right, to finish by bottom-right
    pub fn in_range(self,range:i32) -> Vec<Position> {
        let range = range.abs() ;
        let n : usize = range.abs() as usize + 1 ;
        let mut vec : Vec<Position> = Vec::with_capacity(n*(n-1)*3 + 1);
        // formula is (n/2)*(n-1)*6 + 1
        for dy in -range .. range + 1 {
            for dx in (max(-range,-range-dy) .. min(range,range-dy) + 1).rev() {
                vec.push(Position::new(self.x + dx,self.y + dy));
            }
        };
        vec
    }

    pub fn in_star_edges(self,range:i32) -> Vec<Position> {
        let mut vec : Vec<Position> = Vec::with_capacity(6);
        vec.push(self + (NE * range));
        vec.push(self + (E  * range));
        vec.push(self + (SE * range));
        vec.push(self + (NW * range));
        vec.push(self + (W  * range));
        vec.push(self + (SW * range));
        vec
    }

    /// Center of the star is added first, and
    pub fn in_star(self,range:i32) -> Vec<Position> {
        let range = range.abs() ;
        let mut vec : Vec<Position> = Vec::with_capacity(6 * (range as usize) + 1);
        vec.push(self);
        for r in 1..range + 1 {
            vec.extend_from_slice(self.in_star_edges(r).as_slice());
        }
        vec
    }

    pub fn in_cone(self,direction:MainDirection,range:i32) -> Vec<Position> {
        let (direction,range) = BaseVec(direction,range).normalize().raw();
        let n = (range + 1) as usize;
        let mut vec : Vec<Position> = Vec::with_capacity((n+1) * (n+1));
        for r in 0..range+1 { // r = current_range
            let temp_pos = self + direction.to_pos() * r ;
            vec.push(temp_pos);
            if r != 0 {
                use pos::Rotation::{CounterClockwise as CCW, Clockwise as CW};
                let (direction_1,direction_2) = (direction.rotate(CW).rotate(CW),direction.rotate(CCW).rotate(CCW));
                for i in 1..r+1 {
                    vec.push(temp_pos + direction_1.to_pos() * i);
                    vec.push(temp_pos + direction_2.to_pos() * i);
                }
            }
        }
        vec
    }

    pub fn in_line(self,direction:MainDirection,range:i32) -> Vec<Position> {
        let mut vec : Vec<Position> = Vec::with_capacity(range.abs() as usize + 1);
        let (direction,range) = BaseVec(direction,range).normalize().raw();
        for i in 0..range+1 {
            vec.push(self + direction.to_pos() * i);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use position::* ;

    #[test]
    fn test_in_cone(){
        let v = Position::new(0,0).in_cone(MainDirection::E,1);
        assert!(v.contains(&Position::new(0,0)));
        assert!(v.contains(&Position::new(1,0)));
        assert!(v.contains(&Position::new(0,1)));
        assert!(v.contains(&Position::new(1,-1)));
        assert_eq!(v.len(),4);
    }

    #[test]
    fn test_in_range(){
        let v = Position::new(0,0).in_range(2);
        assert_eq!(v.len(),19);
    }

    #[test]
    fn test_in_star(){
        let v = Position::new(0,0).in_star(2);
        assert_eq!(v.len(),13);
    }

    #[test]
    fn test_in_line(){
        let v = Position::new(0,0).in_line(MainDirection::NNE,3);
        assert_eq!(v.len(),4);
    }
}
