use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct FurnaceNodeComponent;

impl MenuComponent for FurnaceNodeComponent {
    fn setup(_app: &mut App) {}

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building FurnaceNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        };
        world.spawn((FurnaceNodeComponent, node)).set_parent(parent);
    }
}
