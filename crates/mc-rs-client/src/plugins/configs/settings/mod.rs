use bevy::{app::AppExit, prelude::*};
use mc_rs_core::{schedule::state::ApplicationState, sounds::SoundEvent};
use mc_rs_gui::resources::scale::GuiScale;
use serde::{Deserialize, Serialize};

pub mod audio;
use audio::AudioSettings;

pub mod camera;
use camera::CameraSettings;

pub mod resourcepack;
use resourcepack::ResourcePackSettings;

pub mod window;
use window::{GuiScaleSettings, WindowSettings};

use super::traits::{ConfigFile, ResourceConfig};

#[derive(Debug, Default, Clone, PartialEq, Resource, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub window: WindowSettings,
    #[serde(default)]
    pub resourcepacks: ResourcePackSettings,
    #[serde(default)]
    pub camera: CameraSettings,
    #[serde(default)]
    pub audio: AudioSettings,
}

impl ConfigFile for Settings {
    const FILE_PATH: &'static str = "settings.toml";
}
impl ResourceConfig for Settings {
    fn add_systems(app: &mut App) {
        app.add_systems(Startup, ResourcePackSettings::update_resourcepacks);

        app.add_systems(
            PreUpdate,
            (
                CameraSettings::update_camera.run_if(in_state(ApplicationState::InGame)),
                WindowSettings::update_window,
                GuiScaleSettings::update_scale,
                ResourcePackSettings::update_resourcepacks,
                AudioSettings::update_volume,
            )
                .run_if(resource_exists_and_changed::<Settings>()),
        );

        app.add_systems(
            Update,
            (
                AudioSettings::sound_events.run_if(on_event::<SoundEvent>()),
                (GuiScaleSettings::update_settings, Self::save_config)
                    .chain()
                    .run_if(on_event::<AppExit>()),
            ),
        );

        app.add_systems(
            OnEnter(ApplicationState::InGame),
            CameraSettings::update_camera,
        );
    }
}

impl Settings {
    /// Insert resources derived from [Settings] into the app
    pub(crate) fn insert_resources(&self, app: &mut App) {
        // Insert GuiScaleSettings
        {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!(
                "Inserting GuiScaleSettings: {:?}",
                self.window.resolution.gui_scale
            );

            app.insert_resource(self.window.resolution.gui_scale);
        }

        // Insert GuiScale
        {
            let scale: GuiScale = (&self.window.resolution).into();

            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Inserting GuiScale: {scale:?}");

            app.insert_resource(scale);
        }

        // Insert GlobalVolume
        {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Inserting GlobalVolume: {}", self.audio.global);

            app.insert_resource(GlobalVolume::new(self.audio.global));
        }
    }
}
