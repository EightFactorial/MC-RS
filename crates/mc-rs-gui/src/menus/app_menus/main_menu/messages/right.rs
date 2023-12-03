use bevy::{prelude::*, text::BreakLineOn};

use crate::{
    menus::traits::MenuComponent,
    resources::font::{shadows::TextShadow, DefaultTextStyle},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct RightNodeComponent;

impl MenuComponent for RightNodeComponent {
    fn setup(_app: &mut App) {}

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building LeftNodeComponent");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::End,
                justify_content: JustifyContent::End,
                ..Default::default()
            },
            ..Default::default()
        };

        let node = world
            .spawn((RightNodeComponent, node))
            .set_parent(parent)
            .id();

        let style: TextStyle = world.resource::<DefaultTextStyle>().clone().into();
        let text = Text {
            sections: vec![TextSection {
                value: String::from("TODO: Copyright Notice"),
                style,
            }],
            alignment: TextAlignment::Right,
            linebreak_behavior: BreakLineOn::WordBoundary,
        };

        world
            .spawn(TextBundle {
                style: Style {
                    margin: UiRect {
                        right: Val::Px(2.0),
                        bottom: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: text.clone(),
                z_index: ZIndex::Global(i32::MAX - 128),
                ..Default::default()
            })
            .with_children(|node| {
                node.spawn(TextShadow::create_shadow_text_bundle(text));
            })
            .set_parent(node);
    }
}
