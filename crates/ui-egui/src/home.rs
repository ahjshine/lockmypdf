//! Home tab: recommended tools, open card, pinned folders and recent files (local only, never
//! another app's list).

use egui::{Align2, CornerRadius, Rect, Sense, Stroke, vec2};
use lockmypdf_engine::catalog;

use crate::theme::{self, Tokens};
use crate::{LockMyPdfApp, icons, panels::human_size, widgets};

pub mod folders_ui;

const RECOMMENDED: [&str; 5] = ["organize", "comment", "form", "edit", "protect"];

/// A soft shadow that rises on hover and settles when the card is pressed.
pub(crate) fn card_lift(ui: &egui::Ui, t: &Tokens, rect: Rect, press: theme::Press) {
    let lift = press.hover * (1.0 - press.down);
    if lift > 0.04 && t.page_shadow.a() > 0 {
        let shadow = egui::Shadow { offset: [0, 3], blur: (14.0 * lift) as u8, spread: 0, color: t.page_shadow.gamma_multiply(0.85 * lift) };
        ui.painter().add(shadow.as_shape(rect, CornerRadius::same(t.radius)));
    }
}

/// Render a single tool card with the standard lift, styling, icon, title, description, and action.
/// If `allow_unpin` is true, an unpin button is always accessible in the top right corner.
fn tool_card(
    app: &mut LockMyPdfApp,
    ui: &mut egui::Ui,
    t: &Tokens,
    g: &'static catalog::ToolGroup,
    allow_unpin: bool,
    width: f32,
) -> Option<&'static str> {
    let (rect, resp) = ui.allocate_exact_size(vec2(width, 116.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!(g.label)));
    let press = theme::Press::track(ui, &resp);
    card_lift(ui, t, rect, press);
    let fill = press.fill(t.card, t.hover, t.pressed);
    ui.painter().rect(rect, CornerRadius::same(t.radius), fill, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    let body = rect.translate(press.offset());
    let color = egui::Color32::from_rgb(g.hue[0], g.hue[1], g.hue[2]);
    icons::paint(ui, Rect::from_min_size(body.min + vec2(14.0, 14.0), vec2(22.0, 22.0)), g.icon, 21.0, color);

    let mut unpin_clicked = false;
    if allow_unpin {
        let unpin_rect = Rect::from_center_size(body.right_top() + vec2(-18.0, 18.0), vec2(22.0, 22.0));
        let unpin_resp = ui.interact(unpin_rect, ui.scope_id().with(("unpin_card_btn", g.id)), Sense::click());
        unpin_resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("Unpin tool")));
        if unpin_resp.hovered() {
            ui.painter().rect_filled(unpin_rect, CornerRadius::same(t.radius), t.pressed);
        }
        icons::paint(ui, unpin_rect.shrink(4.0), "x", 14.0, t.icon);
        if unpin_resp.on_hover_text(tl!("Unpin from Home")).clicked() {
            unpin_clicked = true;
        }
    }

    let mut title_job = egui::text::LayoutJob::simple_singleline(tl!(g.label).to_owned(), theme::semibold(13.5), t.text);
    title_job.wrap = egui::text::TextWrapping { max_width: body.width() - 58.0, max_rows: 1, break_anywhere: true, ..Default::default() };
    let title = ui.fonts_mut(|f| f.layout_job(title_job));
    let title_elided = title.elided;
    ui.painter().galley(body.min + vec2(44.0, 25.0 - title.size().y * 0.5), title, t.text);
    let blurb = g.sections.first().map(|s| s.items.iter().take(3).map(|i| tl!(i.label)).collect::<Vec<_>>().join(" · ")).unwrap_or_default();
    // Fixed-height cards must leave room for their action in every language.
    let mut job = egui::text::LayoutJob::simple(blurb.clone(), theme::regular(11.5), t.text_muted, body.width() - 28.0);
    job.wrap.max_rows = 2;
    let galley = ui.fonts_mut(|f| f.layout_job(job));
    let blurb_elided = galley.elided;
    ui.painter().galley(body.min + vec2(14.0, 46.0), galley, t.text_muted);
    ui.painter().text(body.left_bottom() + vec2(14.0, -14.0), Align2::LEFT_CENTER, tl!("Use now"), theme::semibold(12.0), t.text);
    let resp = if title_elided || blurb_elided { resp.on_hover_text(format!("{}\n{blurb}", tl!(g.label))) } else { resp };
    if unpin_clicked {
        Some(g.id)
    } else {
        if theme::hand(resp).clicked() {
            app.select_tool(g.id);
        }
        None
    }
}

fn home_card_width(width: f32) -> f32 {
    let gap = 12.0;
    let cols = if width < 480.0 {
        1.0
    } else if width < 740.0 {
        2.0
    } else if width < 980.0 {
        3.0
    } else {
        5.0
    };
    ((width - gap * (cols - 1.0)) / cols).floor().max(120.0)
}

/// Draw the Home page with a local-first hero, recommended tools and recent documents.
pub fn show(app: &mut LockMyPdfApp, ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let compact = ui.available_width() < 640.0;
    egui::ScrollArea::vertical().auto_shrink([false, false]).animated(true).show(ui, |ui| {
        let avail_w = ui.available_width();
        let max_w: f32 = 1080.0;
        let content_w = (avail_w - 32.0).min(max_w).max(260.0);
        let side_padding = ((avail_w - content_w) / 2.0).max(16.0);
        let card_inner_w = (content_w - (if compact { 24.0 } else { 32.0 })).max(120.0);
        let card_width = home_card_width(card_inner_w);

        egui::Frame::NONE.inner_margin(egui::Margin { left: side_padding as i8, right: side_padding as i8, top: 24, bottom: 32 }).show(ui, |ui| {
            ui.label(egui::RichText::new(tl!("01 / PRIVATE PDF WORKSPACE")).font(egui::FontId::monospace(11.0)).color(t.text));
            ui.add_space(14.0);
            ui.label(egui::RichText::new(tl!("Welcome to lockmypdf")).font(theme::medium(14.0)).color(t.text_muted));
            ui.add_space(8.0);
            ui.label(egui::RichText::new(tl!("PDF work. Kept local.")).font(theme::semibold(if compact { 32.0 } else { 50.0 })).color(t.text));
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(tl!("Open, organize, sign, and protect your files without sending them anywhere."))
                    .font(theme::regular(if compact { 14.0 } else { 16.0 }))
                    .color(t.text_muted),
            );
            ui.add_space(20.0);
            if compact {
                ui.vertical_centered(|ui| {
                    let btn_w = (content_w - 32.0).clamp(240.0, 360.0);
                    if widgets::pill_button_sized(ui, tl!("Open a PDF ↗"), true, Some(btn_w), 46.0, 15.0).clicked() {
                        app.open_dialog();
                    }
                    ui.add_space(14.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = vec2(8.0, 4.0);
                        let badge_font = egui::FontId::monospace(11.0);
                        let label_w = ui.fonts_mut(|f| {
                            f.layout_no_wrap(tl!("LOCAL-FIRST").into(), badge_font.clone(), t.text_faint).size().x
                                + f.layout_no_wrap("·".into(), badge_font.clone(), t.badge_new).size().x
                                + f.layout_no_wrap(tl!("NO ACCOUNT").into(), badge_font.clone(), t.text_faint).size().x
                                + f.layout_no_wrap("·".into(), badge_font.clone(), t.badge_new).size().x
                                + f.layout_no_wrap(tl!("WASM").into(), badge_font.clone(), t.text_faint).size().x
                                + 32.0
                        });
                        let pad = ((ui.available_width() - label_w) / 2.0).max(0.0);
                        ui.add_space(pad);
                        ui.label(egui::RichText::new(tl!("LOCAL-FIRST")).font(badge_font.clone()).color(t.text_faint));
                        ui.label(egui::RichText::new("·").font(badge_font.clone()).color(t.badge_new));
                        ui.label(egui::RichText::new(tl!("NO ACCOUNT")).font(badge_font.clone()).color(t.text_faint));
                        ui.label(egui::RichText::new("·").font(badge_font.clone()).color(t.badge_new));
                        ui.label(egui::RichText::new(tl!("WASM")).font(badge_font).color(t.text_faint));
                    });
                });
            } else {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(10.0, 8.0);
                    if widgets::pill_button(ui, tl!("Open a PDF ↗"), true).clicked() {
                        app.open_dialog();
                    }
                    ui.label(egui::RichText::new(tl!("LOCAL-FIRST")).font(egui::FontId::monospace(11.0)).color(t.text_faint));
                    ui.label(egui::RichText::new("·").color(t.badge_new));
                    ui.label(egui::RichText::new(tl!("NO ACCOUNT")).font(egui::FontId::monospace(11.0)).color(t.text_faint));
                    ui.label(egui::RichText::new("·").color(t.badge_new));
                    ui.label(egui::RichText::new(tl!("WASM")).font(egui::FontId::monospace(11.0)).color(t.text_faint));
                });
            }
            ui.add_space(34.0);

            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new(tl!("START WITH A TOOL")).font(egui::FontId::monospace(11.0)).color(t.text_muted));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(tl!("5 focused workflows")).font(egui::FontId::monospace(11.0)).color(t.text_faint));
                });
            });
            ui.add_space(8.0);
            let mut unpin_tool = None;
            egui::Frame::NONE
                .fill(t.card)
                .stroke(Stroke::new(1.0, t.border))
                .corner_radius(CornerRadius::same(t.radius))
                .inner_margin(egui::Margin::same(if compact { 12 } else { 16 }))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = vec2(12.0, 12.0);
                        for id in RECOMMENDED {
                            if app.is_tool_pinned(id) {
                                continue;
                            }
                            let Some(g) = catalog::group(id) else { continue };
                            tool_card(app, ui, &t, g, false, card_width);
                        }
                    });
                });

            ui.add_space(26.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new(tl!("PINNED TOOLS")).font(egui::FontId::monospace(11.0)).color(t.text_muted));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if widgets::ghost_button(ui, "pin", tl!("Pin a tool…")).clicked() {
                        app.dialog = Some(crate::Dialog::PinTool);
                    }
                });
            });
            ui.add_space(6.0);
            if app.pinned_tools.is_empty() {
                ui.label(egui::RichText::new(tl!("Pin your favorite tools here for quick access.")).color(t.text_muted));
            } else {
                let pinned = app.pinned_tools.clone();
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(12.0, 12.0);
                    for id in &pinned {
                        let Some(g) = catalog::group(id) else { continue };
                        if let Some(to_unpin) = tool_card(app, ui, &t, g, true, card_width) {
                            unpin_tool = Some(to_unpin);
                        }
                    }
                });
            }
            if let Some(id) = unpin_tool {
                app.unpin_tool(id);
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                ui.add_space(26.0);
                crate::folders_ui::section(app, ui);
            }

            let mut clear = false;
            if app.recent_limit > 0 {
                ui.add_space(28.0);
                ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new(tl!("RECENT")).font(egui::FontId::monospace(11.0)).color(t.text_muted));
                    if !app.recent.is_empty() {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let resp = widgets::ghost_button(ui, "trash-2", tl!("Clear"));
                            resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("Clear Recent Files")));
                            clear = resp.on_hover_text(tl!("Clear Recent Files")).clicked();
                        });
                    }
                });
                ui.add_space(8.0);
                if app.recent.is_empty() {
                    ui.label(
                        egui::RichText::new(tl!("Files you open in lockmypdf appear here. Drop a PDF anywhere to open it.")).color(t.text_muted),
                    );
                }
            }
            let mut open = None;
            let mut remove = None;
            for r in app.recent.iter().take(app.recent_limit) {
                let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 46.0), Sense::click());
                resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, &r.name));
                let press = theme::Press::track(ui, &resp);
                let hovered = ui.rect_contains_pointer(rect);
                if hovered && press.hover < 0.04 && press.down < 0.04 {
                    ui.painter().rect_filled(rect, CornerRadius::same(t.radius), t.hover);
                } else {
                    press.wash(ui, rect, t.radius, false);
                }
                if hovered {
                    let x_rect = Rect::from_center_size(rect.right_center() - vec2(26.0, 0.0), vec2(28.0, 28.0));
                    let x = ui.interact(x_rect, ui.scope_id().with(("remove-recent", &r.path)), Sense::click());
                    let label = crate::i18n::fmt(tl!("Remove {name} from Recent"), &[("name", &r.name)]);
                    x.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, &label));
                    if x.hovered() {
                        ui.painter().rect_filled(x_rect, CornerRadius::same(t.radius), t.pressed);
                    }
                    icons::paint(ui, x_rect.shrink(6.0), "x", 16.0, t.icon);
                    if x.on_hover_text(tl!("Remove from Recent")).clicked() {
                        remove = Some(r.path.clone());
                    }
                }
                let body = rect.translate(press.offset());
                icons::paint(
                    ui,
                    Rect::from_min_size(body.min + vec2(10.0, 11.0), vec2(24.0, 24.0)),
                    "file-text",
                    22.0,
                    t.icon,
                );
                let detail = ui.painter().text(
                    body.right_center() - vec2(48.0, 0.0),
                    Align2::RIGHT_CENTER,
                    format!("{}  ·  {}", crate::i18n::trn(crate::i18n::current(), r.pages as u64, "{n} page", "{n} pages"), human_size(r.size)),
                    theme::regular(12.0),
                    t.text_muted,
                );
                let text_width = detail.left() - 16.0 - (body.left() + 46.0);
                widgets::row_text(ui, body.min + vec2(46.0, 15.0), r.name.as_str(), theme::medium(13.5), t.text, text_width);
                widgets::row_text(ui, body.min + vec2(46.0, 32.0), r.path.as_str(), theme::regular(11.0), t.text_faint, text_width);
                if theme::hand(resp).on_hover_text(&r.path).clicked() {
                    open = Some(r.path.clone());
                }
            }
            if let Some(p) = open {
                app.open_recent(&p);
            }
            if let Some(p) = remove {
                app.recent.retain(|r| r.path != p);
            }
            if clear {
                app.execute("file.clear_recent");
            }

            ui.add_space(28.0);
            egui::Frame::NONE
                .fill(t.card)
                .stroke(Stroke::new(1.0, t.border))
                .corner_radius(CornerRadius::same(t.radius))
                .inner_margin(egui::Margin::same(if compact { 14 } else { 18 }))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(egui::RichText::new(tl!("YOUR FILES STAY HERE")).font(egui::FontId::monospace(11.0)).color(t.accent));
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(tl!("lockmypdf works in your browser. No account, no telemetry, no cloud processing."))
                            .color(t.text_muted),
                    );
                });
            ui.add_space(16.0);
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn review_home_harness() -> egui_kittest::Harness<'static, LockMyPdfApp> {
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let mut app = LockMyPdfApp::new();
        app.pinned_tools = vec!["edit".into()];
        let mut fonts = false;
        egui_kittest::Harness::builder().with_size(vec2(1500.0, 1200.0)).build_ui_state(
            move |ui, app| {
                if !fonts {
                    ui.ctx().set_fonts(theme::font_definitions());
                    fonts = true;
                }
                show(app, ui);
            },
            app,
        )
    }

    #[test]
    fn review_pinned_recommended_tools_appear_once() {
        use egui_kittest::kittest::Queryable;
        let mut h = review_home_harness();
        h.run_steps(3);
        assert_eq!(h.get_all_by_label("Edit a PDF").count(), 1);
    }

    #[test]
    fn review_a_pinned_tool_can_be_unpinned_without_hover() {
        use egui_kittest::kittest::Queryable;
        let mut h = review_home_harness();
        h.run_steps(3);
        h.get_by_label("Unpin tool").click();
        h.run_steps(2);
        assert!(h.state().pinned_tools.is_empty());
    }

    #[test]
    fn translated_card_description_leaves_room_for_the_action() {
        let de = crate::i18n::Lang::from_code("de").unwrap();
        crate::i18n::set_current(de);
        let ctx = egui::Context::default();
        ctx.set_fonts(theme::font_definitions());
        let mut app = LockMyPdfApp::new();
        let blurb = catalog::group("form")
            .unwrap()
            .sections
            .first()
            .unwrap()
            .items
            .iter()
            .take(3)
            .map(|item| crate::i18n::tr(de, item.label))
            .collect::<Vec<_>>()
            .join(" · ");
        let output = ctx
            .run_ui(egui::RawInput { screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1440.0, 900.0))), ..Default::default() }, |ui| {
                show(&mut app, ui)
            });
        crate::i18n::set_current(crate::i18n::Lang::EN);
        let text_shapes: Vec<_> =
            output.shapes.iter().filter_map(|shape| if let egui::Shape::Text(text) = &shape.shape { Some(text) } else { None }).collect();
        let description = text_shapes.iter().find(|text| text.galley.job.text == blurb).unwrap();
        let action = text_shapes
            .iter()
            .find(|text| text.galley.job.text == crate::i18n::tr(de, "Use now") && (text.pos.x - description.pos.x).abs() < 0.1)
            .unwrap();
        assert!(description.galley.rows.len() <= 2);
        assert!(
            description.pos.y + description.galley.size().y + 4.0 <= action.pos.y,
            "the translated description must leave a visible gap before its action"
        );
        output.drop_without_applying_deltas();
    }

    #[test]
    fn home_uses_narrow_insets_and_full_width_cards_on_phone_sizes() {
        assert_eq!(home_inset(320.0), 20.0);
        assert_eq!(home_inset(768.0), 32.0);
        assert_eq!(home_inset(1440.0), 48.0);
        assert_eq!(home_card_width(280.0), 280.0);
        assert_eq!(home_card_width(640.0), 206.0);
    }

    #[test]
    fn dark_home_action_accent_reads_on_cards() {
        let dark = Tokens::for_kind(crate::theme::ThemeKind::Dark);
        assert!(theme::contrast(dark.accent, dark.card) >= 3.0);
    }
}
