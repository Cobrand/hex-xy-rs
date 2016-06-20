use std::vec::Vec ;
use pos::* ;
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

    pub fn in_cone(self,range:i32,direction:SubDirection) -> Vec<Position> {
        let n = (range + 1) as usize;
        let mut vec : Vec<Position> = Vec::with_capacity((n + 1) * n / 2 );
        //match direction {
        // SubDirection::
        // }
        vec
    }

    pub fn in_line(self,direction:MainDirection,range:i32) -> Vec<Position> {
        let mut vec : Vec<Position> = Vec::with_capacity(range.abs() as usize + 1);
        vec
    }
}

#[cfg(test)]
mod tests {
    use pos::* ;
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
    fn test_in_star(){
        let v = Position::new(0,0).in_star(2);
        assert_eq!(v.len(),13);
    }
}
