//! Base-font manager. Replaces the font asset behind the default font handle so
//! every text node using `TextFont::default()` renders with the mod-supplied
//! font, and rescales each visible size by a global multiplier (0.25..=4).
use super::Mods;
use bevy::prelude::*;
use skate_mods::read_bounded;

/// The default font asset lives at this id (see bevy_text::TextPlugin).
fn default_font() -> AssetId<Font> {
    AssetId::<Font>::default()
}
const FONT_BYTES_LIMIT: u64 = 16 * 1024 * 1024;

pub(crate) struct FontState {
    pub path: String,
    pub scale: f32,
}

/// Remember the untouched font size of a scaled text node so it can be restored.
#[derive(Component)]
pub(crate) struct ModFontScaled {
    pub original: f32,
}

pub(crate) fn header(app: &mut App) -> Option<Font> {
    let font = app
        .world()
        .resource::<Assets<Font>>()
        .get(default_font())
        .cloned();
    if font.is_none() {
        warn!("Default font asset is unavailable; base font mods are disabled");
    }
    font
}

pub(crate) fn apply(
    world: &mut World,
    mods: &mut Mods,
    id: &str,
    path: &str,
    scale: f32,
) -> Result<(), String> {
    if let Some((owner, _)) = &mods.font {
        if owner != id {
            return Err("The base font is already set by another mod".into());
        }
    }
    let root = &mods.manager.packages.get(id).ok_or("Unknown mod")?.root;
    let bytes = read_bounded(root, path, FONT_BYTES_LIMIT)?;
    let font = Font::try_from_bytes(bytes)
        .map_err(|e| format!("{path} is not a valid .ttf/.otf font: {e}"))?;
    world
        .resource_mut::<Assets<Font>>()
        .insert(default_font(), font)
        .map_err(|_| "The default font slot is unavailable".to_string())?;
    mods.font = Some((id.to_owned(), FontState { path: path.to_owned(), scale }));
    Ok(())
}

/// Restore the engine default font and forget the active owner.
pub(crate) fn clear(world: &mut World, mods: &mut Mods) {
    mods.font = None;
    if let Some(default) = &mods.default_font {
        let _ = world
            .resource_mut::<Assets<Font>>()
            .insert(default_font(), default.clone());
    }
}

/// Clear only if this mod currently owns the base font (used on retire/failure).
pub(crate) fn forget(world: &mut World, mods: &mut Mods, id: &str) {
    if mods.font.as_ref().is_some_and(|(owner, _)| owner == id) {
        clear(world, mods);
    }
}

/// Keep every default-font size multiplied by the active scale. Runs after the
/// mod command batch so a font apply refreshes text the same frame it lands.
pub(crate) fn mirror(world: &mut World) {
    let scale = world.resource::<Mods>().font.as_ref().map(|(_, f)| f.scale);
    let mut first_scale: Vec<(Entity, f32)> = Vec::new();
    let mut restore: Vec<Entity> = Vec::new();
    {
        let mut text = world.query::<(Entity, &mut TextFont, Option<&ModFontScaled>)>();
        for (entity, mut font, scaled) in text.iter_mut(world) {
            if font.font.id() != default_font() {
                continue;
            }
            match scale {
                Some(s) if s != 1.0 => {
                    let original = match scaled {
                        Some(m) => m.original,
                        None => {
                            first_scale.push((entity, font.font_size));
                            font.font_size
                        }
                    };
                    let target = original * s;
                    if (font.font_size - target).abs() > 1e-3 {
                        font.font_size = target;
                    }
                }
                _ => {
                    if let Some(m) = scaled {
                        if (font.font_size - m.original).abs() > 1e-3 {
                            font.font_size = m.original;
                        }
                        restore.push(entity);
                    }
                }
            }
        }
    }
    for (entity, original) in first_scale {
        if let Some(mut entity) = world.get_entity_mut(entity) {
            let _ = entity.insert(ModFontScaled { original });
        }
    }
    for entity in restore {
        if let Some(mut entity) = world.get_entity_mut(entity) {
            let _ = entity.remove::<ModFontScaled>();
        }
    }
}