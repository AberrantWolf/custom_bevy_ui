use bevy::prelude::*;

mod ui_system;

/// This example illustrates how to create a button that changes color and text based on its interaction state.
fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<ui_system::ButtonMaterials>()
        .add_startup_system(setup.system())
        .add_system(ui_system::button_system.system())
        .add_system_to_stage(
            bevy::render::stage::DRAW,
            ui_system::draw_widget_system.system(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ui_system::ButtonMaterials>,
) {
    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        .spawn(ui_system::UiWidgetComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // material: button_materials.normal,
            ..Default::default()
        });

    // commands
    //     // ui camera
    //     .spawn(UiCameraComponents::default())
    //     .spawn(ButtonComponents {
    //         style: Style {
    //             size: Size::new(Val::Px(150.0), Val::Px(65.0)),
    //             // center button
    //             margin: Rect::all(Val::Auto),
    //             // horizontally center child text
    //             justify_content: JustifyContent::Center,
    //             // vertically center child text
    //             align_items: AlignItems::Center,
    //             ..Default::default()
    //         },
    //         material: button_materials.normal,
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn(TextComponents {
    //             text: Text {
    //                 value: "Button".to_string(),
    //                 font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
    //                 style: TextStyle {
    //                     font_size: 40.0,
    //                     color: Color::rgb(0.8, 0.8, 0.8),
    //                 },
    //             },
    //             ..Default::default()
    //         });
    //     });
}
