use bevy::prelude::*;

// #[derive(Component)]
// struct Vec3 {
//     x: f32,
//     y: f32,
//     z: f32
// }

// impl Vec3 {
//     fn new() -> Self {
//         Self {
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//         }
//     }
// }

#[derive(Component)]
struct Node(f32);

#[derive(Component)]
struct Edge;

fn init_nodes(mut commands: Commands) {
    println!("Init nodes...");
    commands.spawn()
        .insert_bundle((Node(1.0), Edge));
}

fn main() {
    // Task: create a linear graph like N - N - N with the values 1 - 2 - 3
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_nodes)
        .run();
}
