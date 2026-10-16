//! The supplied Lunola Luma app icon and product lockup, shown without redraw or recolouring.

use egui::{Color32, Context, Id, Rect, TextureHandle, TextureOptions, Ui, pos2};
use photocraft_codecs::{ChannelLayout, SampleType};

/// 128 px source icon for the compact custom title bar.
const ICON_PNG: &[u8] = include_bytes!("../../../assets/app-icon/hicolor/128x128/apps/ai.storyteller.photocraft.png");
#[cfg(not(target_arch = "wasm32"))]
const LOCKUP_PNG: &[u8] = include_bytes!("../../../brand/lunola-luma/assets/lunola-luma-lockup.png");
const LOCKUP_ID: &str = "lunola-luma-official-lockup";

/// The icon's pixels (one transparent pixel if it couldn't be decoded, which a test rules out).
fn decode() -> egui::ColorImage {
    let rgba = photocraft_codecs::decode(ICON_PNG).ok().map(|img| img.convert(ChannelLayout::Rgba, SampleType::U8));
    match rgba {
        Some(img) if img.data().len() == img.width() as usize * img.height() as usize * 4 => {
            egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.data())
        }
        _ => egui::ColorImage::filled([1, 1], Color32::TRANSPARENT),
    }
}

/// The icon texture, uploaded on first use and kept in the context.
fn texture(ctx: &Context) -> TextureHandle {
    let id = Id::new("photocraft-brand-mark");
    if let Some(tex) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return tex;
    }
    let options = TextureOptions { mipmap_mode: Some(egui::TextureFilter::Linear), ..TextureOptions::LINEAR };
    let tex = ctx.load_texture("photocraft-brand-mark", decode(), options);
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    tex
}

/// Where the mark was painted this frame (tests and screenshots).
fn rect_id() -> Id {
    Id::new("photocraft-brand-mark-rect")
}

/// Paint the mark into `r` (square).
pub fn paint_mark(ui: &Ui, r: Rect) {
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    ui.painter().image(texture(ui.ctx()).id(), r, uv, Color32::WHITE);
    ui.ctx().data_mut(|d| d.insert_temp(rect_id(), r));
}

fn decode_lockup(bytes: &[u8]) -> egui::ColorImage {
    let rgba = photocraft_codecs::decode(bytes).ok().map(|img| img.convert(ChannelLayout::Rgba, SampleType::U8));
    match rgba {
        Some(img) if img.data().len() == img.width() as usize * img.height() as usize * 4 => {
            egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.data())
        }
        _ => egui::ColorImage::filled([1, 1], Color32::TRANSPARENT),
    }
}

fn lockup_texture(ctx: &Context) -> TextureHandle {
    let id = Id::new(LOCKUP_ID);
    if let Some(tex) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return tex;
    }
    #[cfg(not(target_arch = "wasm32"))]
    let image = decode_lockup(LOCKUP_PNG);
    #[cfg(target_arch = "wasm32")]
    let image = egui::ColorImage::filled([1, 1], Color32::TRANSPARENT);
    let tex = ctx.load_texture(LOCKUP_ID, image, TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    tex
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn install_lockup(ctx: &Context, bytes: &[u8]) {
    let tex = ctx.load_texture(LOCKUP_ID, decode_lockup(bytes), TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(Id::new(LOCKUP_ID), tex));
}

/// The supplied Lunola Luma product lockup, rendered at its original aspect ratio.
pub fn paint_lockup(ui: &Ui, r: Rect) {
    let tex = lockup_texture(ui.ctx());
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    ui.painter().image(tex.id(), r, uv, Color32::WHITE);
}

/// Source raster aspect: 1448×1086. Keep the supplied lockup undistorted at any width.
pub fn lockup_size(width: f32) -> egui::Vec2 {
    egui::vec2(width, width * (1086.0 / 1448.0))
}

/// The rect the mark was last painted into.
#[cfg(test)]
pub fn mark_rect(ctx: &Context) -> Option<Rect> {
    ctx.data(|d| d.get_temp(rect_id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_official_app_icon_decodes_to_a_transparent_128_px_square() {
        let img = decode();
        assert_eq!(img.size, [128, 128]);
        assert_eq!(img.pixels[0].a(), 0, "transparent background");
        assert!(img.pixels[64 * 128 + 64].a() > 0, "the supplied icon is visible");
    }

    #[test]
    fn supplied_product_lockup_keeps_its_aspect_ratio() {
        assert_eq!(lockup_size(288.0), egui::vec2(288.0, 216.0));
    }

    #[test]
    fn the_title_bar_paints_the_icon_from_one_cached_texture() {
        let ctx = Context::default();
        crate::PhotocraftApp::setup_context(&ctx, crate::theme::ThemeKind::ALL[0]);
        let mut app = crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
        // The system title bar hides the in-app mark (the OS draws its own icon), so paint the
        // app-drawn bar here.
        app.custom_titlebar = true;
        let mut frame = || {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| crate::panels::title_bar(&mut app, ui));
            let uploads = out.textures_delta.set.values().flat_map(|d| d.iter()).filter(|d| d.image.size() == [128, 128]).count();
            out.textures_delta.clear();
            uploads
        };
        assert_eq!(frame(), 1, "uploaded on first use");
        assert_eq!(frame(), 0, "then reused");
        assert!(mark_rect(&ctx).is_some());
    }
}
