use bevy::prelude::*;

use crate::player::{Bolts, CharacterController};

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update);
            
    }
}

#[derive(Component)]
pub struct BoltText;

fn setup(
    mut commands: Commands, 
    
) {
    // bolts text
    commands.spawn((
        BoltText,
        
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Tua madre",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font_size: 40.0,
                color: Color::BLUE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(5.0),
            right: Val::Percent(3.0),
            ..default()
        }),
    ));
    
}


fn update(
    mut text: Query<&mut Text, With<BoltText>>,
    bolts: Query<&Bolts, With<CharacterController>>
) {
    let Ok(mut text) = text.get_single_mut() else {return;};
    let Ok(bolts) = bolts.get_single() else {return;};

    *text = Text::from_section(bolts.0.to_string(), TextStyle {
        font_size: 40.0,
        color: Color::BLUE,
        ..default()
    });
}