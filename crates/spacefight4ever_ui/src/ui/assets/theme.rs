use std::collections::HashMap;
use bevy::prelude::*;

use crate::ui::button::{UiButtonType, UiWindowType};
use super::atlasbuttonskin::ButtonSkin;
use super::windowsskin::WindowSkin;
use super::atlasbuttonskin::UiButtonTypesAllHandles;

/// Theme mode (light or dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    /// Light theme
    Light,
    /// Dark theme (default for game applications)
    #[default]
    Dark,
}

/// Color scheme variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorScheme {
    /// Default Material You purple/violet scheme
    #[default]
    Default,
    /// Custom scheme (use with `MaterialTheme::from_seed`)
    Custom,
}

/// Theme Resource
///
/// Contains all color tokens for the a 3 color system.
/// Use this resource to style your UI components consistently.
///
/// # Example
///
/// ```rust,no_run
/// use bevy::prelude::*;
/// use bevy_material_ui::theme::UiTheme;
///
/// fn setup_ui(theme: Res<UiTheme>, mut commands: Commands) {
///     commands.spawn((
///         Node {
///             width: Val::Percent(100.0),
///             height: Val::Percent(100.0),
///             ..default()
///         },
///         BackgroundColor(theme.surface),
///     ));
/// }
/// ```
#[derive(TypePath, Asset, Debug, Clone)]
pub struct UiTheme {
    /// Current theme mode
    pub mode: ThemeMode,
    pub color_scheme: ColorScheme,

    /// theme stuff for themeable entities
    pub button_skins: UiButtonTypesAllHandles,
    //pub button_skins: HashMap<UiButtonType, Handle<ButtonSkin>>,
    pub window_skins: HashMap<UiWindowType, Handle<WindowSkin>>,

    // Primary colors
    /// Primary brand color
    pub primary: Color,
    /// Color for content on primary
    pub on_primary: Color,
    /// Primary container color
    pub primary_container: Color,
    /// Color for content on primary container
    pub on_primary_container: Color,

    // Secondary colors
    /// Secondary brand color
    pub secondary: Color,
    /// Color for content on secondary
    pub on_secondary: Color,
    /// Secondary container color
    pub secondary_container: Color,
    /// Color for content on secondary container
    pub on_secondary_container: Color,

    // Tertiary colors
    /// Tertiary accent color
    pub tertiary: Color,
    /// Color for content on tertiary
    pub on_tertiary: Color,
    /// Tertiary container color
    pub tertiary_container: Color,
    /// Color for content on tertiary container
    pub on_tertiary_container: Color,

    // Error colors
    /// Error state color
    pub error: Color,
    /// Color for content on error
    pub on_error: Color,
    /// Error container color
    pub error_container: Color,
    /// Color for content on error container
    pub on_error_container: Color,

    // Surface colors
    /// Base surface color
    pub surface: Color,
    /// Color for content on surface
    pub on_surface: Color,
    /// Variant of on_surface for less emphasis
    pub on_surface_variant: Color,
    /// Lowest surface container
    pub surface_container_lowest: Color,
    /// Low surface container
    pub surface_container_low: Color,
    /// Default surface container
    pub surface_container: Color,
    /// High surface container
    pub surface_container_high: Color,
    /// Highest surface container
    pub surface_container_highest: Color,

    // Other colors
    /// Outline color for borders
    pub outline: Color,
    /// Variant outline for subtle borders
    pub outline_variant: Color,
    /// Inverse surface for contrast
    pub inverse_surface: Color,
    /// Content on inverse surface
    pub inverse_on_surface: Color,
    /// Inverse primary for contrast
    pub inverse_primary: Color,
    /// Scrim overlay color
    pub scrim: Color,
    /// Shadow color
    pub shadow: Color,

    // Custom game-specific colors
    /// Color for selected/active states
    pub selected: Color,
    /// Color for unselected/inactive states
    pub unselected: Color,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self::dark()
    }
}

#[inline]
fn hex(hex: &str) -> Color {
    Color::from(Srgba::hex(hex).unwrap())
}

impl UiTheme {
    /// Create a new theme with the specified mode and color scheme
    pub fn new(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }

    /// retrieve the button skin for a button type
    pub fn get_button_skin(&self, button_type: UiButtonType) -> Handle<ButtonSkin> {
        self.button_skins[button_type].clone()
    }

    /// retrieve the window skin for a window type
    pub fn get_window_skin(&self, window_type: UiWindowType) -> Option<&Handle<WindowSkin>> {
        self.window_skins.get(&window_type)
    }

    /// Predefined light theme with default color scheme
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            color_scheme: ColorScheme::Default,
            button_skins: UiButtonTypesAllHandles{ types: [Handle::default(),  Handle::default(), Handle::default(), Handle::default()] },
            window_skins: HashMap::new(),

            primary: Color::from(Srgba::hex("#6750A4").unwrap()),
            on_primary: hex("#FFFFFF"),
            primary_container: hex("#EADDFF"),
            on_primary_container: hex("#21005D"),

            secondary: hex("#625B71"),
            on_secondary: hex("#FFFFFF"),
            secondary_container: hex("#E8DEF8"),
            on_secondary_container: hex("#1D192B"),

            tertiary: hex("#7D5260"),
            on_tertiary: hex("#FFFFFF"),
            tertiary_container: hex("#FFD8E4"),
            on_tertiary_container: hex("#31111D"),

            error: hex("#B3261E"),
            on_error: hex("#FFFFFF"),
            error_container: hex("#F9DEDC"),
            on_error_container: hex("#410E0B"),

            surface: hex("#FFFBFE"),
            on_surface: hex("#1C1B1F"),
            on_surface_variant: hex("#49454F"),
            surface_container_lowest: hex("#FFFBFE"),
            surface_container_low: hex("#FFFBFE"),
            surface_container: hex("#FFFBFE"),
            surface_container_high: hex("#FFFBFE"),
            surface_container_highest: hex("#FFFBFE"),

            outline: hex("#79747E"),
            outline_variant: hex("#C4C4C8"),
            inverse_surface: hex("#313033"),
            inverse_on_surface: hex("#F4EFF4"),
            inverse_primary: hex("#D0BCFF"),
            scrim: Color::srgba(0., 0., 0., 0.5),
            shadow: Color::srgba(0., 0., 0., 0.2),

            selected: hex("#6750A4"),
            unselected: hex("#625B71"),
        }
    }

    /// Predefined dark theme with default color scheme
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            color_scheme: ColorScheme::Default,
            button_skins: UiButtonTypesAllHandles{ types: [Handle::default(),  Handle::default(), Handle::default(), Handle::default()] },
            window_skins: HashMap::new(),

            primary: hex("#D0BCFF"),
            on_primary: hex("#381E72"),
            primary_container: hex("#4F378B"),
            on_primary_container: hex("#EADDFF"),

            secondary: hex("#CCC2DC"),
            on_secondary: hex("#332D41"),
            secondary_container: hex("#4A4458"),
            on_secondary_container: hex("#E8DEF8"),

            tertiary: hex("#EFB8C8"),
            on_tertiary: hex("#492532"),
            tertiary_container: hex("#633B48"),
            on_tertiary_container: hex("#FFD8E4"),

            error: hex("#F2B8B5"),
            on_error: hex("#601410"),
            error_container: hex("#8C1D18"),
            on_error_container: hex("#F9DEDC"),  

            surface: hex("#121212"),
            on_surface: hex("#E6E1E5"),
            on_surface_variant: hex("#CAC4D0"),
            surface_container_lowest: hex("#121212"),
            surface_container_low: hex("#121212"),
            surface_container: hex("#121212"),
            surface_container_high: hex("#121212"),
            surface_container_highest: hex("#121212"),

            outline: hex("#938F99"),
            outline_variant: hex("#49454F"),
            inverse_surface: hex("#E6E1E5"),
            inverse_on_surface: hex("#121212"),
            inverse_primary: hex("#6750A4"),
            scrim: Color::srgba(0., 0., 0., 0.5),
            shadow: Color::srgba(0., 0., 0., 0.2),

            selected: hex("#D0BCFF"),
            unselected: hex("#CCC2DC"),
        }
    }
}