use bevy::prelude::*;

pub trait Asset {
    fn new(asset_server: &Res<AssetServer>) -> Self;
    fn get_meshes(&self) -> Vec<Handle<Mesh>>;
    fn get_align(&self) -> Vec3;
}

macro_rules!
create_asset {
    ($name: ident, $align: expr, $($asset: literal),+) => {
        pub struct $name {
            meshes: Vec<Handle<Mesh>>,
            align: Vec3,
        }
        impl Asset for $name {
            fn new(asset_server: &Res<AssetServer>) -> Self {
                $name {
                    meshes: vec![
                        $(asset_server.load($asset)),*
                    ],
                    align: $align
                }
            }
            fn get_meshes(&self) -> Vec<Handle<Mesh>> {
                self.meshes.clone()
            }
            fn get_align(&self) -> Vec3 {
                self.align
            }
        }
    }
}

create_asset!(
    King,
    Vec3::new(-0.2, 0., -1.9),
    "models/pieces.glb#Mesh0/Primitive0",
    "models/pieces.glb#Mesh1/Primitive0"
);

create_asset!(
    Queen,
    Vec3::new(-0.2, 0., -0.95),
    "models/pieces.glb#Mesh7/Primitive0"
);
create_asset!(
    Rook,
    Vec3::new(-0.1, 0., 1.8),
    "models/pieces.glb#Mesh5/Primitive0"
);
create_asset!(
    Knight,
    Vec3::new(-0.2, 0., 0.9),
    "models/pieces.glb#Mesh3/Primitive0",
    "models/pieces.glb#Mesh4/Primitive0"
);
create_asset!(
    Bishop,
    Vec3::new(-0.1, 0., 0.),
    "models/pieces.glb#Mesh6/Primitive0"
);
create_asset!(
    Pawn,
    Vec3::new(-0.2, 0., 2.6),
    "models/pieces.glb#Mesh2/Primitive0"
);
