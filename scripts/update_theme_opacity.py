import re

# 1. src/app/state.rs
with open("src/app/state.rs", "r") as f:
    state_content = f.read()

state_content = state_content.replace(
    "Theme(ThemeSource),",
    "Theme(ThemeSource, f32),"
)
state_content = state_content.replace(
    'Self::Theme(t) => format!("Theme Grade: {}", t.display_name()),',
    'Self::Theme(t, op) => format!("Theme Grade: {} (Opacity: {:.0}%)", t.display_name(), op * 100.0),'
)
state_content = state_content.replace(
    'Self::Theme(ts) => match ts {',
    'Self::Theme(ts, op) => match ts {'
)
state_content = state_content.replace(
    'ThemeSource::Preset(name) => format!("theme:Preset:{}", name),',
    'ThemeSource::Preset(name) => format!("theme:Preset:{}:{}", name, op),'
)
state_content = state_content.replace(
    'ThemeSource::Custom(path) => format!("theme:Custom:{}", path.to_string_lossy()),',
    'ThemeSource::Custom(path) => format!("theme:Custom:{}:{}", path.to_string_lossy(), op),'
)
state_content = state_content.replace(
    'ThemeSource::CustomPalette(name, _) => format!("theme:Preset:{}", name),',
    'ThemeSource::CustomPalette(name, _) => format!("theme:Preset:{}:{}", name, op),'
)
# Update parser
parser_old = """            "theme" => {
                if parts.len() == 3 {
                    if parts[1] == "Preset" {
                        return Some(Self::Theme(ThemeSource::Preset(parts[2].to_string())));
                    } else if parts[1] == "Custom" {
                        return Some(Self::Theme(ThemeSource::Custom(PathBuf::from(parts[2]))));
                    }
                }
                None
            },"""
parser_new = """            "theme" => {
                let p: Vec<&str> = code.splitn(4, ':').collect();
                if p.len() >= 3 {
                    let op = if p.len() == 4 { p[3].parse().unwrap_or(1.0) } else { 1.0 };
                    if p[1] == "Preset" {
                        return Some(Self::Theme(ThemeSource::Preset(p[2].to_string()), op));
                    } else if p[1] == "Custom" {
                        return Some(Self::Theme(ThemeSource::Custom(PathBuf::from(p[2])), op));
                    }
                }
                None
            },"""
state_content = state_content.replace(parser_old, parser_new)
with open("src/app/state.rs", "w") as f:
    f.write(state_content)


# 2. src/app/mod.rs
with open("src/app/mod.rs", "r") as f:
    mod_content = f.read()

mod_content = mod_content.replace(
    "op: crate::app::state::PipelineOp::Theme(initial_theme.clone()),",
    "op: crate::app::state::PipelineOp::Theme(initial_theme.clone(), 1.0),"
).replace(
    "op: crate::app::state::PipelineOp::Theme(theme.clone()),",
    "op: crate::app::state::PipelineOp::Theme(theme.clone(), 1.0),"
)

process_image_old = """                    crate::app::state::PipelineOp::Theme(theme) => {
                        let node_shades = theme.get_shades();
                        if !node_shades.is_empty() {
                            let mut buf = current_dyn.to_rgba8();"""
process_image_new = """                    crate::app::state::PipelineOp::Theme(theme, opacity) => {
                        let node_shades = theme.get_shades();
                        if !node_shades.is_empty() {
                            let mut buf = current_dyn.to_rgba8();
                            let original_buf = if *opacity < 1.0 { Some(buf.clone()) } else { None };"""
mod_content = mod_content.replace(process_image_old, process_image_new)

# Find par_correct_image calls and add blending if opacity < 1.0
end_old = """                            }
                            current_dyn = image::DynamicImage::ImageRgba8(buf);
                        }
                    },"""
end_new = """                            }
                            if let Some(orig) = original_buf {
                                let op = *opacity;
                                for (px_out, px_in) in buf.pixels_mut().zip(orig.pixels()) {
                                    px_out[0] = ((px_out[0] as f32) * op + (px_in[0] as f32) * (1.0 - op)) as u8;
                                    px_out[1] = ((px_out[1] as f32) * op + (px_in[1] as f32) * (1.0 - op)) as u8;
                                    px_out[2] = ((px_out[2] as f32) * op + (px_in[2] as f32) * (1.0 - op)) as u8;
                                }
                            }
                            current_dyn = image::DynamicImage::ImageRgba8(buf);
                        }
                    },"""
mod_content = mod_content.replace(end_old, end_new)

with open("src/app/mod.rs", "w") as f:
    f.write(mod_content)


# 3. src/ui/workspace.rs
with open("src/ui/workspace.rs", "r") as f:
    ws_content = f.read()

ws_content = ws_content.replace(
    "op: crate::app::state::PipelineOp::Theme(init.clone()),",
    "op: crate::app::state::PipelineOp::Theme(init.clone(), 1.0),"
).replace(
    "op: crate::app::state::PipelineOp::Theme(theme.clone()),",
    "op: crate::app::state::PipelineOp::Theme(theme.clone(), 1.0),"
)

with open("src/ui/workspace.rs", "w") as f:
    f.write(ws_content)
