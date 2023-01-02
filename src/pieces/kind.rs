use std::slice::Iter;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl Kind {
    pub fn iter() -> Iter<'static, Kind> {
        static KIND: [Kind; 6] = [
            Kind::King,
            Kind::Queen,
            Kind::Rook,
            Kind::Knight,
            Kind::Bishop,
            Kind::Pawn,
        ];
        KIND.iter()
    }
}
