use bevy::{	
	prelude::*,
	ui::FocusPolicy
};
use iyes_loopless::prelude::*;

use crate::{
	GameState,
};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App){
        app.add_enter_system(GameState::MainMenu, setup_menu)
        .add_system(handle_start_button.run_in_state(GameState::MainMenu))
		//.add_system_set(SystemSet::on_pause(GameState::MainMenu).with_system(despawn_menu));
        .add_enter_system(GameState::Playing, despawn_menu);
    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>){
    let ui_assets = UiAssets{
		font: assets.load("quattrocentosans-bold.ttf"),
		button: assets.load("button.png"),
		button_pressed: assets.load("button_pressed.png")
	};

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(ButtonBundle{
        style: Style{
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
            margin: Rect::all(Val::Auto),
            ..Default::default()
        },
		color: Color::NONE.into(),
        ..Default::default()
    })
	.with_children(|parent|{
		parent.spawn_bundle(ImageBundle{
			style:Style{
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			image: ui_assets.button.clone().into(),
			..Default::default()
		})
			.insert(FocusPolicy::Pass)
			.with_children(|parent|{
				parent.spawn_bundle(TextBundle{
					text: Text::with_section(
						"Start Game",
						TextStyle{
							font: ui_assets.font.clone(),
							font_size: 40.0,
							color: Color::rgb(0.9, 0.9, 0.9),
						},
						Default::default(),
					),
					focus_policy: FocusPolicy::Pass,
					..Default::default()
				});
			});
			
	}); 
	commands.insert_resource(ui_assets);
}

struct UiAssets{
	font: Handle<Font>,
	button: Handle<Image>,
	button_pressed: Handle<Image>
}
fn handle_start_button(
	mut commands: Commands,
	interaction_query: Query<(&Children, &Interaction), Changed<Interaction>>,
	mut image_query: Query<&mut UiImage>,
	ui_assets: Res<UiAssets>,
	//ascii: Rec<AsciiSheet>
){
	for(children, interaction) in interaction_query.iter(){
		let child = children.iter().next().unwrap();
		let mut image = image_query.get_mut(*child).unwrap();

		match interaction{
			Interaction:: Clicked => {
				image.0 = ui_assets.button_pressed.clone();
				info!("Start Menu");
				commands.insert_resource(NextState(GameState::Playing));
			}
			Interaction::Hovered | Interaction:: None=>{
				image.0 = ui_assets.button.clone();
			}
		}
	}
}

fn despawn_menu(mut commands: Commands, button_query: Query<Entity, With<Button>>) 
{
    for ent in button_query.iter(){
		commands.entity(ent).despawn_recursive();
	}
}
