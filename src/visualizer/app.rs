use crate::celestial::{Camera3D, CelestialBody};
use crate::cognitive::MemoryTier;
use crate::engine::{CelestialGalaxyState, MazzarothEngine};
use eframe::egui::{self, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct DustPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct MazzarothVisualizerApp {
    pub engine: MazzarothEngine,
    pub camera: Camera3D,
    pub auto_rotate: bool,
    pub selected_body: Option<CelestialBody>,
    pub search_query: String,
    pub new_label: String,
    pub new_content: String,
    pub new_tier: MemoryTier,
    pub dust_particles: Vec<DustPoint>,
    pub start_time: Instant,
    last_frame: Instant,
}

fn dust_palette(r: f32) -> (f32, f32, f32) {
    if r < 300.0 {
        let t = (r / 300.0).clamp(0.0, 1.0);
        (1.0 * (1.0 - t) + 0.75 * t, 0.94 * (1.0 - t) + 0.52 * t, 0.54 * (1.0 - t) + 0.99 * t)
    } else {
        let t = ((r - 300.0) / 800.0).clamp(0.0, 1.0);
        (0.75 * (1.0 - t) + 0.22 * t, 0.52 * (1.0 - t) + 0.74 * t, 0.99 * (1.0 - t) + 0.97 * t)
    }
}

fn mix_color(c1: (f32, f32, f32), c2: (f32, f32, f32), factor: f32) -> (f32, f32, f32) {
    (
        c1.0 * (1.0 - factor) + c2.0 * factor,
        c1.1 * (1.0 - factor) + c2.1 * factor,
        c1.2 * (1.0 - factor) + c2.2 * factor,
    )
}

impl MazzarothVisualizerApp {
    pub fn new(engine: MazzarothEngine) -> Self {
        // 1. Pre-generate 6,000 cosmic dust particles matching visual 4-arm spiral law
        let mut dust_particles = Vec::with_capacity(6000);
        let mut seed: u64 = 1337;
        let mut rand = || -> f32 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((seed >> 33) as f32) / 2147483648.0
        };
        let num_arms = 4.0;
        for i in 0..6000 {
            let arm = (i as f32) % num_arms;
            let arm_angle = (arm / num_arms) * std::f32::consts::PI * 2.0;
            let r = rand().powf(1.5) * 1100.0 + 20.0;
            let spin = r * 0.003;
            let rx = (rand().powf(3.0) * if rand() < 0.5 { 1.0 } else { -1.0 } * 0.28) * r;
            let ry = (rand().powf(3.0) * if rand() < 0.5 { 1.0 } else { -1.0 } * 0.12) * r;
            let rz = (rand().powf(3.0) * if rand() < 0.5 { 1.0 } else { -1.0 } * 0.28) * r;

            let x = (arm_angle + spin).cos() * r + rx;
            let y = ry + (r * 0.01).sin() * 20.0;
            let z = (arm_angle + spin).sin() * r + rz;

            let (cr, cg, cb) = dust_palette(r);
            dust_particles.push(DustPoint {
                x,
                y,
                z,
                r: (cr * 255.0) as u8,
                g: (cg * 255.0) as u8,
                b: (cb * 255.0) as u8,
                a: 35,
            });
        }

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
            dust_particles,
            start_time: Instant::now(),
            last_frame: Instant::now(),
        }
    }
}

impl eframe::App for MazzarothVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f32().min(0.1);
        self.last_frame = now;
        let elapsed = (now - self.start_time).as_secs_f32();

        // 1. Celestial camera rotation
        if self.auto_rotate {
            self.camera.rot_y += dt * 0.15;
        }

        // Continuous repaint for smooth celestial orbit rendering
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

        // Left Panel: Ingest & Node Inspector HUD (Image 2 Parity)
        egui::SidePanel::left("hud_panel").min_width(360.0).show(ctx, |ui| {
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

                        // Image 2: Planetary Schematic & Cognitive Harmonic Resonance Waveform Widget
                        ui.add_space(6.0);
                        let (hud_rect, _) = ui.allocate_exact_size(Vec2::new(320.0, 110.0), egui::Sense::hover());
                        let p = ui.painter_at(hud_rect);
                        p.rect_filled(hud_rect, 8.0, Color32::from_rgb(14, 6, 32));
                        p.rect_stroke(hud_rect, 8.0, Stroke::new(1.0_f32, Color32::from_rgb(168, 85, 247)));

                        // 1. Saturnian ringed planet schematic on the left
                        let planet_c = Pos2::new(hud_rect.min.x + 50.0, hud_rect.center().y);
                        // Back ring
                        p.circle_filled(planet_c, 24.0, Color32::from_rgba_unmultiplied(250, 204, 21, 50));
                        // Planet sphere
                        p.circle_filled(planet_c, 14.0, Color32::from_rgb(234, 88, 12));
                        p.circle_filled(planet_c, 10.0, Color32::from_rgb(250, 204, 21));
                        // Front ring stroke
                        p.line_segment(
                            [Pos2::new(planet_c.x - 26.0, planet_c.y + 4.0), Pos2::new(planet_c.x + 26.0, planet_c.y - 4.0)],
                            Stroke::new(3.0_f32, Color32::from_rgb(250, 204, 21)),
                        );

                        // Orbiting Moon
                        let m_angle = elapsed * 1.5;
                        let mx = planet_c.x + m_angle.cos() * 32.0;
                        let my = planet_c.y + m_angle.sin() * 12.0;
                        p.circle_filled(Pos2::new(mx, my), 3.0, Color32::from_rgb(56, 189, 248));

                        // 2. Cognitive Harmonic Waveform on the right
                        let wave_rect = Rect::from_min_size(Pos2::new(hud_rect.min.x + 110.0, hud_rect.min.y + 10.0), Vec2::new(200.0, 90.0));
                        p.rect_filled(wave_rect, 4.0, Color32::from_rgb(8, 2, 18));
                        p.rect_stroke(wave_rect, 4.0, Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(56, 189, 248, 50)));

                        let wave_mid_y = wave_rect.center().y;
                        let lum = body.luminosity.clamp(0.2, 1.0);
                        for x_step in (0..190).step_by(3) {
                            let fx0 = x_step as f32;
                            let fx1 = (x_step + 3) as f32;
                            let nx0 = fx0 / 190.0;
                            let nx1 = fx1 / 190.0;

                            let y0 = wave_mid_y + (nx0 * 12.0 - elapsed * 4.0).sin() * (16.0 * lum) * (-nx0 * 0.8).exp();
                            let y1 = wave_mid_y + (nx1 * 12.0 - elapsed * 4.0).sin() * (16.0 * lum) * (-nx1 * 0.8).exp();

                            p.line_segment(
                                [Pos2::new(wave_rect.min.x + fx0, y0), Pos2::new(wave_rect.min.x + fx1, y1)],
                                Stroke::new(1.5_f32, Color32::from_rgb(250, 204, 21)),
                            );
                        }

                        p.text(
                            Pos2::new(wave_rect.min.x + 6.0, wave_rect.min.y + 4.0),
                            egui::Align2::LEFT_TOP,
                            format!("SYNC: {:.1}%", lum * 99.4),
                            FontId::monospace(9.0),
                            Color32::from_rgb(56, 189, 248),
                        );

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
            painter.rect_filled(rect, 0.0, Color32::from_rgb(4, 1, 10));

            // Draw 6k Procedural Dust Backdrop
            for d in &self.dust_particles {
                let (dx, dy, d_depth) = self.camera.project(d.x, d.y, d.z);
                let dust_pos = Pos2::new(dx, dy);
                if d_depth > 10.0 && rect.contains(dust_pos) {
                    painter.circle_filled(dust_pos, 1.0, Color32::from_rgba_unmultiplied(d.r, d.g, d.b, d.a));
                }
            }

            let galaxy_state = self.engine.get_galaxy_state().unwrap_or_else(|_| CelestialGalaxyState {
                bodies: Vec::new(),
                lines: Vec::new(),
                timestamp: 0,
            });

            // Map bodies by ID for O(1) line lookups
            let body_map: std::collections::HashMap<&str, &CelestialBody> =
                galaxy_state.bodies.iter().map(|b| (b.id.as_str(), b)).collect();

            // Draw Galactic Center Black Hole / Core Halo
            let (cx, cy, c_depth) = self.camera.project(0.0, 0.0, 0.0);
            let proj_scale_core = if c_depth > 1.0 { (400.0 / c_depth) * self.camera.zoom } else { self.camera.zoom };
            painter.circle_filled(Pos2::new(cx, cy), (12.0 * proj_scale_core).clamp(4.0, 14.0), Color32::from_rgba_unmultiplied(255, 215, 0, 35));
            painter.circle_filled(Pos2::new(cx, cy), (4.5 * proj_scale_core).clamp(2.0, 5.0), Color32::from_rgb(255, 215, 0));

            // Draw Constellation Lines with depth & relationship weighting
            for line in &galaxy_state.lines {
                let src = body_map.get(line.source_id.as_str()).copied();
                let tgt = body_map.get(line.target_id.as_str()).copied();

                if let (Some(s), Some(t)) = (src, tgt) {
                    let (sx, sy, s_depth) = self.camera.project(s.x, s.y, s.z);
                    let (tx, ty, t_depth) = self.camera.project(t.x, t.y, t.z);

                    if s_depth > 10.0 || t_depth > 10.0 {
                        let avg_depth = (s_depth + t_depth) * 0.5;
                        let proj_scale = if avg_depth > 1.0 { (400.0 / avg_depth) * self.camera.zoom } else { self.camera.zoom };
                        let stroke_w = (0.8 * proj_scale).clamp(0.4, 1.8);

                        let (r, g, b, alpha) = match line.relationship.as_str() {
                            "core_gravitational_ray" => (250, 217, 77, 20), // 0.08 alpha
                            "interstellar_bridge" => {
                                let dist = ((s.x - t.x).powi(2) + (s.y - t.y).powi(2) + (s.z - t.z).powi(2)).sqrt();
                                let len_fade = (1.0 - dist / 200.0).max(0.2);
                                (192, 132, 252, (30.0 * len_fade) as u8) // 0.12 * lenFade
                            }
                            "version_orbit" => (56, 189, 248, 75), // 0.30 alpha
                            "contains_chapter" => (168, 85, 247, 60), // 0.25 alpha
                            _ => (168, 85, 247, 50),
                        };

                        painter.line_segment(
                            [Pos2::new(sx, sy), Pos2::new(tx, ty)],
                            Stroke::new(stroke_w, Color32::from_rgba_unmultiplied(r, g, b, alpha)),
                        );
                    }
                }
            }

            // Draw Celestial Bodies with scaled perspective, radial glow & nearest hit detection
            let mut best_hit_body = None;
            let mut best_hit_dist = f32::MAX;
            let mouse_pos = ctx.input(|i| i.pointer.hover_pos());

            for body in &galaxy_state.bodies {
                let (px, py, depth) = self.camera.project(body.x, body.y, body.z);
                let screen_pos = Pos2::new(px, py);

                if depth > 10.0 && rect.contains(screen_pos) {
                    let proj_scale = if depth > 1.0 { (400.0 / depth) * self.camera.zoom } else { self.camera.zoom };
                    let star_radius = (body.radius * proj_scale * 0.35).clamp(1.5, 7.0);

                    // Radial glow & hue blend
                    let r = (body.x * body.x + body.z * body.z).sqrt();
                    let falloff = (1.0 - r / 900.0).max(0.0);
                    let radius_fade = 0.35 + 0.65 * falloff;
                    let lum = body.luminosity * radius_fade;
                    let tier_col = (body.color.r as f32 / 255.0, body.color.g as f32 / 255.0, body.color.b as f32 / 255.0);
                    let dp = dust_palette(r);
                    let blended = mix_color(tier_col, dp, 0.45);
                    let final_r = ((blended.0 * lum * 255.0).clamp(0.0, 255.0)) as u8;
                    let final_g = ((blended.1 * lum * 255.0).clamp(0.0, 255.0)) as u8;
                    let final_b = ((blended.2 * lum * 255.0).clamp(0.0, 255.0)) as u8;
                    let star_color = Color32::from_rgb(final_r, final_g, final_b);

                    // Outer glowing corona
                    painter.circle_filled(
                        screen_pos,
                        star_radius * 1.8,
                        Color32::from_rgba_unmultiplied(final_r, final_g, final_b, 25),
                    );

                    // Core star point
                    painter.circle_filled(screen_pos, star_radius, star_color);

                    // Star Label: Only for core or selected stars to eliminate clutter
                    let is_selected = self.selected_body.as_ref().is_some_and(|s| s.id == body.id);
                    let is_core = body.id.contains(":core:");
                    if is_core || is_selected {
                        painter.text(
                            Pos2::new(px, py + star_radius + 4.0),
                            egui::Align2::CENTER_TOP,
                            &body.label,
                            FontId::proportional(11.0),
                            Color32::from_rgb(226, 232, 240),
                        );
                    }

                    // Nearest Click / Hover detection
                    if let Some(mpos) = mouse_pos {
                        let dist = (mpos.x - px).hypot(mpos.y - py);
                        let hit_radius = (star_radius * 1.5).max(6.0);
                        if dist < hit_radius && dist < best_hit_dist {
                            best_hit_dist = dist;
                            best_hit_body = Some(body.clone());
                        }
                    }
                }
            }

            if let Some(h) = best_hit_body {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                let (hx, hy, _) = self.camera.project(h.x, h.y, h.z);
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(hx + 10.0, hy - 20.0), Vec2::new(160.0, 24.0)),
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

                if ctx.input(|i| i.pointer.primary_clicked()) {
                    self.selected_body = Some(h.clone());
                }
            }
        });
    }
}
