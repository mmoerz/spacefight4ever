use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::ui::button::UiButtonType;
use crate::ui::titlebar::ui_titlebar_bundle;

use super::titlebar::UiTitleBar;
use super::button::{UiWindowState, UiWindowType};
use super::assets::{windowsskin::WindowSkin, theme::UiTheme, atlasbuttonskin::ButtonSkin};

// state components for last used window size-index
//
//
#[derive(Resource, Default, Debug)]
pub struct UiWindowZCounter(i32);

impl UiWindowZCounter {
    pub fn inc(&mut self) -> i32 {
        self.0 += 1;
        self.0
    }
    pub fn get(&self) -> i32 {
        self.0
    }
}

/// resource to track the focused window
#[derive(Resource, Debug)]
pub struct UiWindowFocused(Entity);

impl Default for UiWindowFocused {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

impl UiWindowFocused {
    pub fn set(&mut self, entity: Entity) {
        self.0 = entity;
    }
    pub fn get(&self) -> Entity {
        self.0
    }
}

#[derive(Component, Clone)]
pub struct UiWindowCurrentState {
    pub state: UiWindowState,
    pub normal_size: Vec2,
    pub normal_pos: Vec2,
    pub focused: bool,
}

impl UiWindowCurrentState {
    pub fn save_window_size(&mut self, node: &Node) {
        self.normal_size = Vec2::new(
            match node.width {
                Val::Px(v) => v,
                _ => return, // ignore non-pixel layouts
            },
            match node.height {
                Val::Px(v) => v,
                _ => return,
            },
        );

        self.normal_pos = Vec2::new(
            match node.left {
                Val::Px(v) => v,
                _ => 0.0,
            },
            match node.top {
                Val::Px(v) => v,
                _ => 0.0,
            },
        );
    }

    pub fn restore_window_size(&mut self, node: &mut Node) {
        node.width = Val::Px(self.normal_size.x);
        node.height = Val::Px(self.normal_size.y);
        node.left = Val::Px(self.normal_pos.x);
        node.top = Val::Px(self.normal_pos.y);
    }
}

#[derive(Component, Clone)]
pub struct UiAtlasWindow {
    pub window_type: UiWindowType,
    pub skin: Handle<WindowSkin>,
    pub min_width: f32,
    pub min_height: f32,
}

pub fn on_window_click_focus(
    on_down: On<Pointer<Press>>,
    mut z_query: Query<&mut GlobalZIndex, With<UiAtlasWindow>>,
    mut counter: ResMut<UiWindowZCounter>,
    parents: Query<&ChildOf>,
    mut focus: ResMut<UiWindowFocused>,
) {
    let mut current = on_down.event_target();

    // climb hierarchy until we find UiWindow
    loop {
        if z_query.contains(current) {
            let mut z = z_query.get_mut(current).unwrap();
            z.0 = counter.inc();
            focus.set(current);
            return;
        }

        current = match parents.get(current) {
            Ok(p) => p.get(),
            Err(_) => return,
        };
    }
}

#[derive(Component)]
pub struct UiWindowDragging(Entity);

pub fn on_window_titlebar_drag_start(
    on_drag_start: On<Pointer<DragStart>>,
    parents: Query<&ChildOf, With<UiTitleBar>>,
    mut commands: Commands,
) {
    if let Ok(parent) = parents.get(on_drag_start.event_target()) {
        commands.entity(on_drag_start.event_target())
            .insert(UiWindowDragging(parent.get()));
    }
}

pub fn on_window_titlebar_drag(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut UiTransform, With<UiAtlasWindow>>,
    dragging: Query<&UiWindowDragging>,
) {
    if let Ok(drag) = dragging.get(on_drag.event_target()) {
        //println!("name: {:?}", name);
        if let Ok(mut transform) = query.get_mut(drag.0) {
            // Extract the current values as f32
            let Val::Px(x) = transform.translation.x else { return };
            let Val::Px(y) = transform.translation.y else { return };

            transform.translation = Val2::px(
                x + on_drag.delta.x,
                y + on_drag.delta.y
            );
        }
    }
}

pub fn on_window_titlebar_drag_end(
    on_drag_end: On<Pointer<DragEnd>>,
    mut query: Query<(&mut Node, &mut UiTransform)>,
    dragging: Query<&UiWindowDragging>,
    mut commands: Commands,
) {
    if let Ok(drag) = dragging.get(on_drag_end.event_target()) {
        if let Ok((mut node, mut transform)) = query.get_mut(drag.0) {
            let Val::Px(dx) = transform.translation.x else { return };
            let Val::Px(dy) = transform.translation.y else { return };

            if let Val::Px(left) = node.left {
                node.left = Val::Px(left + dx);
            }

            if let Val::Px(top) = node.top {
                node.top = Val::Px(top + dy);
            }
            transform.translation = Val2::ZERO;
        }
        commands.entity(on_drag_end.event_target()).remove::<UiWindowDragging>();
    }
}

pub fn close_windows(
    mut commands: Commands,
    interaction_query: Query<(Entity, &UiButtonType, &Interaction), (Changed<Interaction>, With<UiButtonType>)>,
    parents: Query<&ChildOf>,
    windows: Query<Entity, With<UiAtlasWindow>>,
) {
    for (button_entity, button_type, interaction) in &interaction_query {
        if *interaction == Interaction::Pressed && *button_type == UiButtonType::Close {
            // find the parent window
            if let Some(window_entity) = get_window_node(&windows, button_entity, &parents) {
                // despawn window and all children
                commands.entity(window_entity).despawn();
                println!("Closed window {:?}", window_entity);
            }
        }
    }
}

#[derive(Message)]
pub struct UiWindowsStatusChangeRequest {
    window: Entity,
    status: UiWindowState,
}


// TODO: implement making the titlebar smaller and moving it to the bottom corner, but
// this will need some sort of tracking what windows have already been minimized
pub fn minimize_windows(
    q: Query<(&UiButtonType, &Interaction, &ChildOf), (Changed<Interaction>, With<UiButtonType>)>,
    parent_query: Query<&ChildOf>,
    windows: Query<Entity, With<UiAtlasWindow>>,
    mut change_message: MessageWriter<UiWindowsStatusChangeRequest>,
) {
    for (button_type, interaction, button_container) in q {
        if *interaction == Interaction::Pressed && *button_type == UiButtonType::Minimize {
            if let Some(window_entity) = get_window_node(&windows, button_container.get(), &parent_query) {
                change_message.write(UiWindowsStatusChangeRequest {
                    window: window_entity,
                    status: UiWindowState::Minimized,
                });
            }
        }
    }
}

/// separate system, so that windows can be minimized by other means as well
/// TODO: implement hide as well?
pub fn apply_window_state_change(
    mut change_message: MessageReader<UiWindowsStatusChangeRequest>,
    mut q: Query<(&UiAtlasWindow, &mut UiWindowCurrentState, &mut Node), With<UiAtlasWindow>>,
    skin_query: Res<Assets<WindowSkin>>,
) {
    for message in change_message.read() {
        if let Ok((window, mut current, mut node)) = q.get_mut(message.window) {
            match (current.state, message.status) {
                (UiWindowState::Normal, UiWindowState::Minimized) => {
                    let skin: &WindowSkin = skin_query.get(&window.skin).unwrap();
                    current.save_window_size(&node);
                    node.height = Val::Px(skin.titlebar.height);
                    current.state = UiWindowState::Minimized;
                    current.focused = false;
                },
                (UiWindowState::Normal, UiWindowState::Maximized) => {
                    current.save_window_size(&node);
                    node.left = Val::Px(0.);
                    node.top = Val::Px(0.);
                    node.width = Val::Percent(100.);
                    node.height = Val::Percent(100.);
                    current.state = UiWindowState::Maximized;
                    current.focused = true;
                },
                (UiWindowState::Minimized, UiWindowState::Normal) | 
                (UiWindowState::Minimized, UiWindowState::Minimized)=> {
                    current.restore_window_size(&mut node);
                    current.state = UiWindowState::Normal;
                    current.focused = true;
                },
                (UiWindowState::Minimized, UiWindowState::Maximized) => {
                    node.left = Val::Px(0.);
                    node.top = Val::Px(0.);
                    node.width = Val::Percent(100.);
                    node.height = Val::Percent(100.);
                    current.state = UiWindowState::Maximized;
                    current.focused = true;
                }
                (UiWindowState::Maximized, UiWindowState::Normal) |
                (UiWindowState::Maximized, UiWindowState::Maximized) => {
                    current.restore_window_size(&mut node);
                    current.state = UiWindowState::Normal;
                    current.focused = true;
                },
                (UiWindowState::Maximized, UiWindowState::Minimized) => {
                    let skin: &WindowSkin = skin_query.get(&window.skin).unwrap();
                    node.height = Val::Px(skin.titlebar.height);
                    current.state = UiWindowState::Minimized;
                    current.focused = false;
                }
                _ => {}
            }
        }
    }
}

pub fn maximize_windows(
    mut q: Query<(&UiButtonType, &Interaction, &ChildOf), (Changed<Interaction>, With<UiButtonType>)>,
    parent_query: Query<&ChildOf>,
    mut windows: Query<Entity, With<UiAtlasWindow>>,
    mut change_message: MessageWriter<UiWindowsStatusChangeRequest>,
) {
    for (button_type, interaction, button_container) in &mut q {
        if *interaction == Interaction::Pressed && *button_type == UiButtonType::Maximize {
             if let Ok(titlebar) = parent_query.get(button_container.get()) {
                println!("parent_tbar: {:?}", titlebar);
                if let Ok(parent_window) = parent_query.get(titlebar.get()) {
                    if let Ok(window) = windows.get_mut(parent_window.get()) {
                        change_message.write(UiWindowsStatusChangeRequest {
                            window: window,
                            status: UiWindowState::Maximized,
                        });
                    }
                }
            }
        }
    }
}

// pub fn window_button_interaction_system(
//     mut interaction_query: Query<
//         (&Interaction, &UiAtlasButtonIndex, &mut ImageNode),
//         (Changed<Interaction>, With<UiButtonType>,
//         )>,
//     texture_atlases: Res<UiWindowAtlas>,
// ){
//     for (interaction, button_index, mut image_node  ) in &mut interaction_query {
//         if let Some(atlas) = &mut image_node.texture_atlas {
//             atlas.index = match *interaction {
//                 Interaction::Pressed => button_index.0 + texture_atlases.button_offset * 2,
//                 Interaction::Hovered => button_index.0 + texture_atlases.button_offset,
//                 Interaction::None => button_index.0,
//             };
//         }
//     }    
// }

/// enum for expressing the side of the window that gets resized
#[derive(Clone, Copy)]
pub enum ResizeSide {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// what side of the window gets resized
#[derive(Component)]
pub struct UiWindowResizeHandle {
    pub side: ResizeSide,
}

pub fn window_resize_system(
    on_drag: On<Pointer<Drag>>,
    resize_query: Query<(&UiWindowResizeHandle, &ChildOf)>,
    windows: Query<Entity, With<UiAtlasWindow>>,
    atlas_windows: Query<&UiAtlasWindow>,
    parents: Query<&ChildOf>,
    mut node_query: Query<&mut Node>,
) {
    // only process if the drag target is a resize handle
    let target = on_drag.event_target();

    if let Ok((handle, child_parent)) = resize_query.get(target) {
        if let Some(window_entity) = get_window_node(&windows, child_parent.get(), &parents) {
            if let Ok(mut node) = node_query.get_mut(window_entity) {
                if let Ok(window) = atlas_windows.get(window_entity) {
                    match handle.side {
                        ResizeSide::BottomRight => {
                            let w = match node.width { Val::Px(v) => v, _ => window.min_width };
                            let h = match node.height { Val::Px(v) => v, _ => window.min_height };
                            node.width = Val::Px((w + on_drag.delta.x).max(window.min_width));
                            node.height = Val::Px((h + on_drag.delta.y).max(window.min_height));
                        },
                        ResizeSide::Right => {
                            let w = match node.width { Val::Px(v) => v, _ => window.min_width };
                            node.width = Val::Px((w + on_drag.delta.x).max(window.min_width));
                        },
                        ResizeSide::Bottom => {
                            let h = match node.height { Val::Px(v) => v, _ => window.min_height };
                            node.height = Val::Px((h + on_drag.delta.y).max(window.min_height));
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

pub struct UiAtlasWindowPlugin;

impl Plugin for UiAtlasWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiWindowFocused>()
            .init_resource::<UiWindowZCounter>()
            .add_message::<UiWindowsStatusChangeRequest>()
            .add_observer(on_window_click_focus)
            .add_observer(on_window_titlebar_drag)
            .add_observer(on_window_titlebar_drag_end)
            .add_observer(on_window_titlebar_drag_start)
            .add_observer(window_resize_system)
            .add_systems(Update, (
                minimize_windows,
                apply_window_state_change,
                maximize_windows,
                close_windows,
            ));
    }
}

#[derive(Component)]
pub struct UiWindowMain;

/// status bar marker
#[derive(Default, Component)]
pub struct UiWindowStatusBar;

pub struct UiAtlasWindowBuilder {
    pub window_type: UiWindowType,
    pub title: String,
    pub skin: Handle<WindowSkin>,
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub left: f32,
    pub min_width: f32,
    pub min_height: f32,
    pub initial_image_node: ImageNode,
    pub z_index: i32,
}

impl UiAtlasWindowBuilder {
    pub fn new(title: String, window_type: UiWindowType, theme: &UiTheme, skins: &Assets<WindowSkin>) -> Self {
        //let image_node = ImageNode::default();
        let skin_handle = theme.get_window_skin(window_type).unwrap().clone();
        let window_skin  = skins.get(&skin_handle).unwrap();

        let initial_image_node  = 
            ImageNode::from_atlas_image(
                    window_skin.image.clone(),
                    TextureAtlas {
                        index: window_skin.atlas_index,
                        layout: window_skin.atlas.clone(),
                    },
            )
            .with_mode(NodeImageMode::Sliced(window_skin.atlas_slicer.clone()));

        Self {
            window_type: UiWindowType::Standard,
            title,
            skin: skin_handle,
            width: window_skin.default_size.x as f32,
            height: window_skin.default_size.y as f32,
            top: window_skin.default_pos.y as f32,
            left: window_skin.default_pos.x as f32,
            min_width: window_skin.min_size.x as f32,
            min_height: window_skin.min_size.y as f32,
            initial_image_node,
            z_index: 0,
        }
    }

    pub fn set_skin(mut self, skin: Handle<WindowSkin>) -> Self {
        self.skin = skin;
        self
    }

    pub fn with_z_index(mut self, z_index: &mut UiWindowZCounter) -> Self {
        self.z_index = z_index.inc();
        self
    }

    pub fn build_with_theme(
        self,
        theme: &UiTheme,
        button_skins: &Assets<ButtonSkin>,
        window_skins: &Assets<WindowSkin>,
    ) -> impl Bundle {
        (
            UiAtlasWindow {
                window_type: self.window_type,
                skin: self.skin.clone(),
                min_width: self.min_width,
                min_height: self.min_height,
            },
            UiWindowCurrentState {
                state: UiWindowState::Normal,
                normal_size: Vec2::new(self.width, self.height),
                normal_pos: Vec2::new(self.left, self.top),
                focused: false,
            },
            Node {
                width: Val::Px(self.width),
                height: Val::Px(self.height),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                position_type: PositionType::Absolute,
                left: Val::Px(self.left),
                top: Val::Px(self.top),
                ..default()
            },
            self.initial_image_node,
            GlobalZIndex(self.z_index),

            Transform::default(),
            GlobalTransform::default(),
            Visibility::Inherited,
            children![(
                ui_titlebar_bundle(
                    self.title,
                    theme,
                    self.window_type,
                    button_skins,
                    window_skins)
            ),
            // Window body + resize handles
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        UiWindowMain,
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                    ),
                    (
                        Node {
                            width: px(5.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Right },
                    )
                ]
            ),
            (
                Node {
                    width: Val::Percent(100.),
                    height: px(5.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        Node {
                            width: Val::Percent(100.),
                            height: px(5.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Bottom },
                        UiWindowStatusBar,
                    ),
                    (
                        Node {
                            width: px(5.),
                            height: px(5.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::BottomRight },
                    )
                ]
            )
            ]
        )
    }
}

pub fn get_window_node(
    windows: &Query<Entity, With<UiAtlasWindow>>,
    mut current: Entity,
    parents: &Query<&ChildOf>,
) -> Option<Entity> {
    while let Ok(parent) = parents.get(current) {
        if windows.contains(current) {
            return Some(current);
        }
        current = parent.get();
    }

    windows.contains(current).then_some(current)
}

/// Helper for spawning a titlebar
pub fn spawn_ui_window(
    commands: &mut Commands,
    title: String,
    window_type: UiWindowType,
    theme: &UiTheme,
    button_skins: &Assets<ButtonSkin>,
    window_skins: &Assets<WindowSkin>,
) -> Entity {
    commands.spawn(
        UiAtlasWindowBuilder::new(title, window_type, theme, window_skins)
            .build_with_theme(theme, button_skins, window_skins)
    ).id()
}

// Spawn a titlebar using a `UiTheme`
// pub fn ui_thematic_titlebar_bundle(
//     theme: &UiTheme,
//     width: f32,
//     height: f32,
//     margin: UiRect,
//     skins: &Assets<TitlebarSkin>,
// ) -> Option<impl Bundle> {
//     theme.titlebar_skins.get(&theme.default_titlebar).map(|skin| {
//         UiTitleBarBuilder::new(skin.clone(), width, height, margin, skins).build()
//     })
// }

pub fn ui_window_bundle(
    title: String,
    window_type: UiWindowType,
    theme: &UiTheme,
    button_skins: &Assets<ButtonSkin>,
    window_skins: &Assets<WindowSkin>, // <- pass assets
) -> impl Bundle {
    UiAtlasWindowBuilder::new(title, window_type, theme, window_skins)
            .build_with_theme(theme, button_skins, window_skins)
}