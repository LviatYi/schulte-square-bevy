use crate::plugin::lynx::lynx_plugin::same_val;
use bevy::log::warn;
use bevy::prelude::{
    Added, Changed, ChildOf, Children, Component, ComputedNode, Entity, Local, Node, Or, Query,
    Ref, Val, With,
};
use std::collections::HashSet;

/// Fits a UI node into the bounds of its direct parent slot while preserving a fixed aspect ratio.
///
/// This component is designed for the `slot -> content` pattern:
///
/// - The parent slot participates in normal Bevy UI layout and owns the final available area.
/// - The child content carries `AspectFitInSlot` and becomes as large as possible inside that slot.
/// - The child keeps the requested `aspect_ratio` and will not intentionally overflow the slot.
///
/// Best practices:
///
/// - Put `AspectFitInSlot` on the only direct child of a dedicated slot node.
/// - Let the slot decide its size through ordinary flex/grid sizing such as `width: 100%` or `height: 80%`.
/// - Keep the fitted content node focused on aspect fitting; do not also make siblings compete for the slot space.
/// - If the slot has more than one direct child, the resulting size may be surprising because the slot no longer
///   represents a single content area. In debug builds, `LynxPlugin` logs a warning for this case.
///
/// Typical structure:
///
/// ```ignore
/// commands.spawn(Node {
///     width: Val::Percent(100.0),
///     height: Val::Percent(100.0),
///     align_items: AlignItems::Center,
///     justify_content: JustifyContent::Center,
///     ..default()
/// }).with_children(|slot| {
///     slot.spawn((
///         Node {
///             width: Val::Auto,
///             height: Val::Auto,
///             ..default()
///         },
///         AspectFitInSlot::new(1.0),
///     ));
/// });
/// ```
///
/// `aspect_ratio` follows the same meaning as Bevy UI: `width / height`.
#[derive(Component, Debug, Clone, Copy)]
pub struct AspectFitInSlot {
    aspect_ratio: f32,
}

impl AspectFitInSlot {
    pub const fn new(aspect_ratio: f32) -> Self {
        Self { aspect_ratio }
    }

    pub const fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}

pub fn update_max_aspect_fit_nodes(
    mut warned_slots: Local<HashSet<Entity>>,
    changed_slots: Query<
        (Entity, Ref<ComputedNode>, &Children),
        (With<Node>, Changed<ComputedNode>),
    >,
    aspect_fit_nodes: Query<&AspectFitInSlot>,
    mut fit_nodes: Query<&mut Node, With<AspectFitInSlot>>,
    mut changed_fit_nodes: Query<
        (Entity, &ChildOf, Ref<AspectFitInSlot>),
        Or<(
            Added<AspectFitInSlot>,
            Changed<AspectFitInSlot>,
            Changed<ChildOf>,
        )>,
    >,
    slot_computed_nodes: Query<&ComputedNode, With<Node>>,
    children_query: Query<&Children>,
) {
    let mut pending_updates = Vec::new();

    for (slot_entity, parent_slot_computed, children) in &changed_slots {
        if parent_slot_computed.is_empty() {
            continue;
        }

        if children.is_empty() {
            continue;
        }

        for child_entity in children.iter() {
            let Ok(fit_comp) = aspect_fit_nodes.get(*child_entity) else {
                continue;
            };

            let aspect_ratio = fit_comp.aspect_ratio();
            if aspect_ratio <= 0.0 {
                continue;
            }

            #[cfg(debug_assertions)]
            if children.len() > 1 && warned_slots.insert(slot_entity) {
                warn!(
                    "AspectFitInSlot expects its parent slot to have exactly one direct child, but {:?} has {} children. Result may be unexpected.",
                    slot_entity,
                    children.len()
                );
            }

            pending_updates.push((
                *child_entity,
                aspect_ratio,
                *parent_slot_computed,
            ));
        }
    }

    for (entity, parent, fit_comp) in &mut changed_fit_nodes {
        let Ok(parent_slot_computed) = slot_computed_nodes.get(parent.0) else {
            continue;
        };

        if parent_slot_computed.is_empty() {
            continue;
        }

        let aspect_ratio = fit_comp.aspect_ratio();
        if aspect_ratio <= 0.0 {
            continue;
        }

        #[cfg(debug_assertions)]
        if let Ok(children) = children_query.get(parent.0) {
            if children.len() > 1 && warned_slots.insert(parent.0) {
                warn!(
                    "AspectFitInSlot expects its parent slot to have exactly one direct child, but {:?} has {} children. Result may be unexpected.",
                    parent.0,
                    children.len()
                );
            }
        }

        pending_updates.push((entity, aspect_ratio, *parent_slot_computed));
    }

    for (entity, aspect_ratio, parent_slot_computed) in pending_updates {
        let Ok(mut node) = fit_nodes.get_mut(entity) else {
            continue;
        };
        let available_width = parent_slot_computed.size.x;
        let available_height = parent_slot_computed.size.y;
        apply_aspect_fit(&mut node, aspect_ratio, available_width, available_height);
    }
}

fn apply_aspect_fit(
    node: &mut Node,
    aspect_ratio: f32,
    available_width: f32,
    available_height: f32,
) {
    let available_ratio = available_width / available_height;
    let (target_width, target_height) = if available_ratio > aspect_ratio {
        (Val::Auto, Val::Percent(100.0))
    } else {
        (Val::Percent(100.0), Val::Auto)
    };

    let target_aspect_ratio = Some(aspect_ratio);
    if same_val(&node.width, &target_width)
        && same_val(&node.height, &target_height)
        && node.aspect_ratio == target_aspect_ratio
    {
        return;
    }

    node.width = target_width;
    node.height = target_height;
    node.aspect_ratio = target_aspect_ratio;
}
