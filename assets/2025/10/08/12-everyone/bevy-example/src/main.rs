use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, setup).run();
}

#[derive(Resource)]
struct Timer(i32);

fn setup(mut commands: Commands, mut meshes: Assets<Mesh>) {
    commands.insert_resource(Timer(0));
    meshes.add(Sphere::new(1.).mesh());
}
