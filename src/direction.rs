use std::ops::Neg;
use pos;

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
    NNE,
    E,
    SSE,
    SSW,
    W,
    NNW,
}

impl MainDirection {
    pub fn rotate(self,rotation:Rotation) -> MainDirection {
        use Rotation::*;
        use MainDirection as M;
        match (self,rotation) {
            (M::NNE,Clockwise) | (M::SSE, CounterClockwise) => M::E,
            (M::E,Clockwise)   | (M::SSW, CounterClockwise) => M::SSE,
            (M::SSE,Clockwise) | (M::W , CounterClockwise) => M::SSW,
            (M::SSW,Clockwise) | (M::NNW, CounterClockwise) => M::W,
            (M::W,Clockwise)   | (M::NNE, CounterClockwise) => M::NNW,
            (M::NNW,Clockwise) | (M::E , CounterClockwise) => M::NNE,
        }
    }

    pub fn to_pos(self) -> pos::Position {
        use MainDirection as M;
        match self {
            M::NNE => pos::NE,
            M::E   => pos::E ,
            M::SSE => pos::SE,
            M::SSW => pos::SW,
            M::W   => pos::W ,
            M::NNW => pos::NW,
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum SubDirection {
    N,
    ENE,
    ESE,
    S,
    WSW,
    WNW
}

impl SubDirection {
    pub fn rotate(self,rotation:Rotation) -> SubDirection {
        use Rotation::*;
        use SubDirection as S;
        match (self,rotation) {
            (S::WNW, Clockwise) | (S::ENE, CounterClockwise) => S::N,
            (S::N  , Clockwise) | (S::ESE, CounterClockwise) => S::ENE,
            (S::ENE, Clockwise) | (S::S  , CounterClockwise) => S::ESE,
            (S::ESE, Clockwise) | (S::WSW, CounterClockwise) => S::S,
            (S::S  , Clockwise) | (S::WNW, CounterClockwise) => S::WSW,
            (S::WSW, Clockwise) | (S::N  , CounterClockwise) => S::WNW,
        }
    }

    pub fn sides(self) -> (MainDirection,MainDirection) {
        use SubDirection as S;
        use MainDirection as M;
        match self {
            S::N =>   (M::NNW,M::NNE),
            S::ENE => (M::NNE,M::E  ),
            S::ESE => (M::E  ,M::SSE),
            S::S =>   (M::SSE,M::SSW),
            S::WSW => (M::SSW,M::W  ),
            S::WNW => (M::W  ,M::NNW)
        }
    }
}

impl Neg for MainDirection {
    type Output = MainDirection ;
    fn neg(self) -> MainDirection {
        match self {
            MainDirection::NNE => MainDirection::SSW,
            MainDirection::E   => MainDirection::W,
            MainDirection::SSE => MainDirection::NNW,
            MainDirection::SSW => MainDirection::NNE,
            MainDirection::W   => MainDirection::E,
            MainDirection::NNW => MainDirection::SSE,
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub struct BaseVec(pub MainDirection,pub i32);

/// ```
/// // use hex-xy::MainDirection::{E,W};
/// // assert_eq!((E,-3).normalize().raw(),(W,3));
/// // assert_eq!((W,2).normalize().raw(),(W,2));
/// ```
impl BaseVec {
    pub fn normalize(self) -> BaseVec {
        if self.1 < 0 {
            BaseVec(-self.0,-self.1)
        } else {
            BaseVec(self.0,self.1)
        }
    }

    pub fn raw(self) -> (MainDirection,i32) {
        (self.0,self.1)
    }
}
