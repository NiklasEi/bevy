//! Shows how to return to the calling function after a windowed Bevy app has exited.

use bevy::{prelude::*, winit::WinitSettings};

fn main() {
    println!("Running Bevy App");
    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Close window to return to main function".to_owned(),
                ..default()
            },
            ..default()
        }))
        .add_system(system1)
        .run();
    println!("Bevy App has exited. We are back in our main function.");
}

fn system1() {
    info!("Logging from Bevy App");
}
