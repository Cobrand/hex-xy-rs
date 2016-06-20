//            +y
//         ^
//        | | +x
//         V
//
pub struct Position {
    pub x : i32,
    pub y : i32
}

impl Position {
    pub fn new(x:i32,y:i32) -> Position {
        Position {x:x,y:y}
    }

    pub fn get_z(&self) -> i32 {
        -self.y - self.x
    }
}

#[test]
fn get_z(){
    assert_eq!(Position::new(1,0).get_z(),-1);
    assert_eq!(Position::new(0,0).get_z(),0);
    assert_eq!(Position::new(5,-2).get_z(),0);
}
