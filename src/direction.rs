
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Rotation {
    Clockwise,
    CounterClockwise
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Direction {
    MainDirection(MainDirection),
    SubDirection(SubDirection)
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum MainDirection {
    NE,
    E,
    SE,
    SW,
    W,
    NW,
}

impl MainDirection {
    pub fn rotate(self,rotation:Rotation) -> MainDirection {
        use Rotation::*;
        match (self,rotation) {
            (MainDirection::NE,Clockwise) | (MainDirection::SE, CounterClockwise) => MainDirection::E,
            (MainDirection::E,Clockwise)  | (MainDirection::SW, CounterClockwise) => MainDirection::SE,
            (MainDirection::SE,Clockwise) | (MainDirection::W , CounterClockwise) => MainDirection::SW,
            (MainDirection::SW,Clockwise) | (MainDirection::NW, CounterClockwise) => MainDirection::W,
            (MainDirection::W,Clockwise)  | (MainDirection::NE, CounterClockwise) => MainDirection::NW,
            (MainDirection::NW,Clockwise) | (MainDirection::E , CounterClockwise) => MainDirection::NE,
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum SubDirection {
    /// North
    N,
    /// North East East
    NEE,
    /// South East East
    SEE,
    /// South
    S,
    /// South West West
    SWW,
    /// North West West
    NWW
}

impl SubDirection {
    pub fn rotate(self,rotation:Rotation) -> SubDirection {
        use Rotation::*;
        match (self,rotation) {
            (SubDirection::NWW,Clockwise) | (SubDirection::NEE, CounterClockwise) => SubDirection::N,
            (SubDirection::N,Clockwise)   | (SubDirection::SEE, CounterClockwise) => SubDirection::NEE,
            (SubDirection::NEE,Clockwise) | (SubDirection::S, CounterClockwise) => SubDirection::SEE,
            (SubDirection::SEE,Clockwise) | (SubDirection::SWW , CounterClockwise) => SubDirection::S,
            (SubDirection::S,Clockwise)   | (SubDirection::NWW, CounterClockwise) => SubDirection::SWW,
            (SubDirection::SWW,Clockwise) | (SubDirection::N , CounterClockwise) => SubDirection::NWW,
        }
    }
}
