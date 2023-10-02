#[derive(PartialEq, Clone,Hash, Eq, Debug, Copy)]
pub enum Direction {
    NorthRight,
    EastRight,
    WestRight,
    SouthRight,
    North,
    South,
    West,
    East,
    NorthLeft,
    EastLeft,
    WestLeft,
    SouthLeft,
}
