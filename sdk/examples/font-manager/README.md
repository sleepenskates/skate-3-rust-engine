# Base Font Manager (Lua SDK 1)

Example mod that swaps the game-wide base font for the HUD and menus.

## What it does

- Scans `.ttf`/`.otf` files bundled under this package's `fonts/` folder.
- Replaces the engine's default font asset, so every text node that does not
  set an explicit font handle renders with the selected face (HUD, menus,
  overlays).
- Applies a global **Text size scale** (0.25x..4x) so the picked face reads at
  a comfortable size even if it renders larger or smaller than the stock font.
- Restores the engine default automatically on unload, disable, failure, or when
  another mod grabs the base font slot.

## Usage

1. Drop the mod package into `mods/` and enable it in the game's Mods menu.
2. Put `.ttf`/`.otf` files in this mod's `fonts/` folder (or repack them into
   the ZIP). The in-game overlay lists the available `fonts/` paths.
3. In the Mods menu, open **Base Font Manager** and set:
   - **Font file**: package-relative path like `fonts/MyFont.ttf`.
     Leave empty to keep / restore the engine default font.
   - **Text size scale**: multiplier for every default-font text size
     (e.g. `1.0` unchanged, `1.5` larger, `0.7` smaller).

Only one mod can own the base font at a time; a second attempt fails with a
clear error.

## Packaged font

`fonts/FiraMono-subset.ttf` is the same face the engine ships, so enabling the
mod with the default settings changes nothing visually. Replace it with any
other `.ttf`/`.otf` and set **Font file** to its path.

`FiraMono-subset.ttf` is derived from Fira Mono, licensed under the SIL Open
Font License 1.1 (see the OFL text in the upstream Fira Mono distribution; the
subset is taken unmodified from the Bevy engine source).