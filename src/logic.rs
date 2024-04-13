

pub struct Player {
    name: String,
    card: Card,

}

pub struct Card {
    role: Role,
    
}

pub enum Role {
    Werwolf,
    Dorfbewohner,
    Hexe,
    Amor,
    Dorfmatratze,
    Jaeger,

}
