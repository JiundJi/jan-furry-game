use bevy::ecs::component::Component;



#[derive(Component, PartialEq, Clone)] pub struct Player {
    pub name: String,
    pub role: Role,

}

#[derive(Default, PartialEq, Clone)] pub enum Role {
    Werwolf,
    Dorfbewohner,
    Hexe,
    Amor,
    Dorfmatratze,
    Jaeger,
    #[default] None,
}
