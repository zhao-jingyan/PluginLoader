// 自定义 UI 组件
// Phase 3 后期会在这里添加自定义控件

use eframe::egui;

/// 电平表组件
pub struct LevelMeter {
    pub level: f32,
    pub peak: f32,
}

impl LevelMeter {
    pub fn new() -> Self {
        Self {
            level: 0.0,
            peak: 0.0,
        }
    }
    
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // TODO: 实现漂亮的电平表可视化
        ui.add(egui::ProgressBar::new(self.level).show_percentage());
    }
}

impl Default for LevelMeter {
    fn default() -> Self {
        Self::new()
    }
}

