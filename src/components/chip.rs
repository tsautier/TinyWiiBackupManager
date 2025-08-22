// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-2.0-only

use eframe::egui::{self, Color32, RichText};

use crate::game::ConsoleType;

/// Creates a chip component with the specified text and color
pub fn chip(ui: &mut egui::Ui, text: &str, color: Color32) -> egui::Response {
    egui::Frame::new()
        .fill(color)
        .corner_radius(10.0)
        .inner_margin(egui::Margin::symmetric(6, 2))
        .show(ui, |ui| ui.label(RichText::new(text).color(Color32::WHITE)))
        .response
}

/// Creates a console chip with appropriate colors for each console type
pub fn console_chip(ui: &mut egui::Ui, console: &ConsoleType) -> egui::Response {
    match console {
        ConsoleType::GameCube => chip(ui, "🎮 GC", Color32::from_rgb(159, 58, 224)),
        ConsoleType::Wii => chip(ui, "🎾 Wii", Color32::from_rgb(0, 150, 214)),
    }
}
