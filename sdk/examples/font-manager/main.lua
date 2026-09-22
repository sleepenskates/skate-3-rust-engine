-- Base Font Manager: picks a packaged .ttf/.otf as the game-wide base font.
-- Configure it from the Mods menu (Esc > MODS): Font file = package-relative
-- path under this mod's fonts/ folder (empty restores the engine default),
-- Text size scale = multiplier applied to every default-font HUD text size.
local function format_size(bytes)
    local kb = (bytes or 0) / 1024
    if kb >= 1024 then return string.format('%.1f MiB', bytes / (1024 * 1024)) end
    return string.format('%.0f KiB', kb)
end
local function font_exists(path)
    for _, f in ipairs(sdk.ui.font.list()) do
        if f.path == path then return true end
    end
    return false
end
local function draw()
    local status = sdk.snapshot.font or {}
    local active = status.active and (status.path .. ' x' .. status.scale) or '(engine default)'
    local rows = {
        'BASE FONT MANAGER',
        'Active: ' .. active,
        '',
        'Packaged fonts:',
    }
    local fonts = sdk.ui.font.list()
    if #fonts == 0 then
        rows[#rows + 1] = '  (drop .ttf/.otf into this mod\'s fonts/ folder)'
    else
        for i, f in ipairs(fonts) do
            rows[#rows + 1] = string.format('  %s - %s', f.path, format_size(f.size))
            if i >= 10 then break end
        end
    end
    rows[#rows + 1] = ''
    rows[#rows + 1] = 'Settings in the Mods menu: Font file, Text size scale.'
    rows[#rows + 1] = 'Empty Font file restores the engine default font.'
    for i = 1, 16 do
        sdk.ui.text('font_line' .. i, rows[i] or '')
    end
end
local function refresh()
    local file = sdk.settings.font_file or ''
    local scale = sdk.settings.scale or 1
    if file == '' then
        sdk.ui.font.clear()
    elseif font_exists(file) then
        sdk.ui.font.apply(file, scale)
    else
        sdk.ui.font.clear()
        sdk.log('Font Manager: ' .. file .. ' is not in fonts/; using the default font.')
    end
    draw()
end
return {
    on_load = function()
        refresh()
        sdk.log('Base Font Manager ready. Open the Mods menu (Esc) to pick a font.')
    end,
    on_settings = function()
        refresh()
    end,
    on_event = function(e)
        if e.name == 'world_changed' then
            refresh()
        end
    end,
}