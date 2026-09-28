use crate::celestial::{Camera3D, CelestialBody};
use crate::cognitive::MemoryTier;
use crate::engine::{CelestialGalaxyState, MazzarothEngine};
use eframe::egui::{self, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use std::time::Instant;

pub struct MazzarothVisualizerApp {
    pub engine: MazzarothEngine,
    pub camera: Camera3D,
    pub auto_rotate: bool,
    pub selected_body: Option<CelestialBody>,
    pub search_query: String,
    pub new_label: String,
    pub new_content: String,
    pub new_tier: MemoryTier,
    last_frame: Instant,
}

impl MazzarothVisualizerApp {
    pub fn new(engine: MazzarothEngine) -> Self {
        Self {
            engine,
            camera: Camera3D {
                rot_x: 0.35,
                rot_y: 0.0,
                zoom: 1.1,
                center_x: 450.0,
                center_y: 350.0,
            },
            auto_rotate: true,
            selected_body: None,
            search_query: String::new(),
            new_label: String::new(),
            new_content: String::new(),
            new_tier: MemoryTier::Episodic,
            last_frame: Instant::now(),
        }
    }
}

impl eframe::App for MazzarothVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f32().min(0.1);
        self.last_frame = now;

        // 1. Celestial camera rotation
        if self.auto_rotate {
            self.camera.rot_y += dt * 0.15;
        }

        // Request continuous repaint for smooth 30-60 FPS celestial orbit rendering
        ctx.request_repaint();

        // Top Control Header
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🌌 MAZZAROTH : CELESTIAL MEMORY GALAXY");
                ui.separator();
                ui.checkbox(&mut self.auto_rotate, "⟳ Auto Orbit");
                ui.separator();
                ui.label(format!("Zoom: {:.1}x", self.camera.zoom));
                if ui.button("Zoom In (+)").clicked() {
                    self.camera.zoom = (self.camera.zoom + 0.15).min(3.0);
                }
                if ui.button("Zoom Out (-)").clicked() {
                    self.camera.zoom = (self.camera.zoom - 0.15).max(0.4);
                }
                if ui.button("Reset Camera").clicked() {
                    self.camera.rot_x = 0.35;
                    self.camera.rot_y = 0.0;
                    self.camera.zoom = 1.1;
                }
            });
        });

        // Left Panel: Ingest & Node Inspector HUD
        egui::SidePanel::left("hud_panel").min_width(340.0).show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Memory Ingestion");
                ui.add_space(4.0);

                ui.group(|ui| {
                    ui.label("Memory Label / Concept");
                    ui.text_edit_singleline(&mut self.new_label);

                    ui.label("Content / Thoughts / Episode");
                    ui.text_edit_multiline(&mut self.new_content);

                    ui.horizontal(|ui| {
                        ui.label("Tier:");
                        ui.selectable_value(&mut self.new_tier, MemoryTier::Working, "Working");
                        ui.selectable_value(&mut self.new_tier, MemoryTier::Episodic, "Episodic");
                        ui.selectable_value(&mut self.new_tier, MemoryTier::Semantic, "Semantic");
                    });

                    ui.add_space(4.0);
                    if ui.button("✨ Form Celestial Memory").clicked() && !self.new_label.is_empty() {
                        let _ = self.engine.ingest(&self.new_label, &self.new_content, self.new_tier, vec!["user".into()]);
                        self.new_label.clear();
                        self.new_content.clear();
                    }
                });

                ui.add_space(10.0);
                ui.heading("Search & Recall");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.search_query);
                    if ui.button("🔍 Recall").clicked() && !self.search_query.is_empty() {
                        if let Ok(recalled) = self.engine.recall(&self.search_query, 5) {
                            if let Some(first) = recalled.first() {
                                self.selected_body = Some(CelestialBody::from_node(first));
                            }
                        }
                    }
                });

                ui.add_space(10.0);
                ui.heading("Selected Star HUD");
                if let Some(body) = &self.selected_body {
                    ui.group(|ui| {
                        ui.colored_label(Color32::from_rgb(body.color.r, body.color.g, body.color.b), format!("✦ {}", body.label));
                        ui.label(format!("Tier: {:?}", body.tier));
                        ui.label(format!("Mass / Retention: {:.2}", body.mass));
                        ui.label(format!("Luminosity: {:.2}", body.luminosity));
                        ui.label(format!("Position: ({:.1}, {:.1}, {:.1})", body.x, body.y, body.z));
                        ui.label(format!("Orbit Radius: {:.1} AU", body.orbit_radius));

                        if !body.tags.is_empty() {
                            ui.add_space(4.0);
                            ui.label("Tags:");
                            ui.horizontal_wrapped(|ui| {
                                for tag in &body.tags {
                                    ui.colored_label(Color32::from_rgb(168, 85, 247), format!("#{}", tag));
                                }
                            });
                        }

                        if !body.content.is_empty() {
                            ui.add_space(4.0);
                            ui.label("Content / Knowledge:");
                            ui.label(egui::RichText::new(&body.content).color(Color32::from_rgb(226, 232, 240)).size(11.0));
                        }

                        if let Ok(links) = self.engine.store.get_links_for_node(&body.id) {
                            if !links.is_empty() {
                                ui.add_space(6.0);
                                ui.label(format!("Constellation Links ({}):", links.len()));
                                for link in links.iter().take(10) {
                                    let other_id = if link.source_id == body.id { &link.target_id } else { &link.source_id };
                                    ui.small(format!("→ {} ({}, w={:.2})", other_id, link.relationship, link.weight));
                                }
                                if links.len() > 10 {
                                    ui.small(format!("... and {} more links", links.len() - 10));
                                }
                            }
                        }
                    });
                } else {
                    ui.label("Click any star in the galaxy to inspect its cognitive neural state.");
                }
            });
        });

        // Central 3D Celestial Canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

            self.camera.center_x = rect.center().x;
            self.camera.center_y = rect.center().y;

            if response.dragged() {
                let delta = response.drag_delta();
                self.camera.rot_y += delta.x * 0.008;
                self.camera.rot_x = (self.camera.rot_x - delta.y * 0.008).clamp(-1.4, 1.4);
            }

            let painter = ui.painter_at(rect);
            // Deep space cosmic background
            painter.rect_filled(rect, 0.0, Color32::from_rgb(5, 7, 15));

            let galaxy_state = self.engine.get_galaxy_state().unwrap_or_else(|_| CelestialGalaxyState {
                bodies: Vec::new(),
                lines: Vec::new(),
                timestamp: 0,
            });

            // Map bodies by ID for O(1) line lookups
            let body_map: std::collections::HashMap<&str, &CelestialBody> =
                galaxy_state.bodies.iter().map(|b| (b.id.as_str(), b)).collect();

            // Draw Galactic Center Black Hole / Core Halo
            let (cx, cy, _) = self.camera.project(0.0, 0.0, 0.0);
            painter.circle_filled(Pos2::new(cx, cy), 18.0 * self.camera.zoom, Color32::from_rgba_unmultiplied(255, 215, 0, 40));
            painter.circle_filled(Pos2::new(cx, cy), 10.0 * self.camera.zoom, Color32::from_rgb(255, 215, 0));

            // Draw Constellation Lines with depth
            for line in &galaxy_state.lines {
                let src = body_map.get(line.source_id.as_str()).copied();
                let tgt = body_map.get(line.target_id.as_str()).copied();

                if let (Some(s), Some(t)) = (src, tgt) {
                    let (sx, sy, _) = self.camera.project(s.x, s.y, s.z);
                    let (tx, ty, _) = self.camera.project(t.x, t.y, t.z);

                    let alpha = (line.weight * 180.0) as u8;
                    painter.line_segment(
                        [Pos2::new(sx, sy), Pos2::new(tx, ty)],
                        Stroke::new(1.2 * self.camera.zoom, Color32::from_rgba_unmultiplied(100, 149, 237, alpha)),
                    );
                }
            }

            // Draw Celestial Bodies (Stars, Planets, Comets) with glow halos
            let mut hovered_body = None;
            let mouse_pos = ctx.input(|i| i.pointer.hover_pos());

            for body in &galaxy_state.bodies {
                let (px, py, depth) = self.camera.project(body.x, body.y, body.z);
                let screen_pos = Pos2::new(px, py);

                if depth > 10.0 && rect.contains(screen_pos) {
                    let star_radius = (body.radius * self.camera.zoom).max(3.0);
                    let base_color = Color32::from_rgb(body.color.r, body.color.g, body.color.b);

                    // Outer glowing corona
                    painter.circle_filled(
                        screen_pos,
                        star_radius * 2.5,
                        Color32::from_rgba_unmultiplied(body.color.r, body.color.g, body.color.b, 40),
                    );

                    // Core star
                    painter.circle_filled(screen_pos, star_radius, base_color);

                    // Star Label
                    painter.text(
                        Pos2::new(px, py + star_radius + 4.0),
                        egui::Align2::CENTER_TOP,
                        &body.label,
                        FontId::proportional(11.0),
                        Color32::from_rgb(226, 232, 240),
                    );

                    // Click detection
                    if let Some(mpos) = mouse_pos {
                        if (mpos.x - px).hypot(mpos.y - py) < (star_radius * 2.0).max(12.0) {
                            hovered_body = Some(body.clone());
                            if ctx.input(|i| i.pointer.primary_clicked()) {
                                self.selected_body = Some(body.clone());
                            }
                        }
                    }
                }
            }

            if let Some(h) = hovered_body {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                let (hx, hy, _) = self.camera.project(h.x, h.y, h.z);
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(hx + 10.0, hy - 20.0), Vec2::new(140.0, 24.0)),
                    4.0,
                    Color32::from_rgba_unmultiplied(15, 23, 42, 220),
                );
                painter.text(
                    Pos2::new(hx + 15.0, hy - 8.0),
                    egui::Align2::LEFT_CENTER,
                    format!("✦ {}", h.label),
                    FontId::proportional(12.0),
                    Color32::WHITE,
                );
            }
        });
    }
}
