use bevy::ecs::component::Component;


#[derive(Default, PartialEq, Clone, Component)] pub enum Role {
    Werwolf,
    Dorfbewohner,
    Hexe,
    Amor,
    Dorfmatratze,
    Jaeger,
    #[default] None,
}
