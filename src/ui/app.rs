use eframe::egui;
use std::sync::{Arc, Mutex};
use log::{info, error};

use crate::plugin::{PluginScanner, PluginInfo};
use crate::audio::AudioProcessorEngine;

/// Plugin Loader 主应用
pub struct PluginLoaderApp {
    /// 插件扫描器
    scanner: PluginScanner,
    
    /// 已扫描的插件列表
    plugins: Vec<PluginInfo>,
    
    /// 音频处理引擎
    audio_engine: Arc<Mutex<Option<AudioProcessorEngine>>>,
    
    /// 已加载的插件（用于显示）
    loaded_plugins: Vec<String>,
    
    /// 选中的插件索引
    selected_plugin: Option<usize>,
    
    /// 搜索过滤文本
    search_filter: String,
    
    /// 是否显示扫描窗口
    show_scan_window: bool,
    
    /// 扫描状态消息
    scan_status: String,
}

impl Default for PluginLoaderApp {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginLoaderApp {
    pub fn new() -> Self {
        info!("初始化 Plugin Loader 应用");
        
        let scanner = PluginScanner::new();
        
        // 尝试从缓存加载插件
        let plugins = scanner.load_cache().unwrap_or_default();
        info!("从缓存加载了 {} 个插件", plugins.len());
        
        Self {
            scanner,
            plugins,
            audio_engine: Arc::new(Mutex::new(None)),
            loaded_plugins: Vec::new(),
            selected_plugin: None,
            search_filter: String::new(),
            show_scan_window: false,
            scan_status: String::new(),
        }
    }
    
    /// 扫描插件
    fn scan_plugins(&mut self) {
        info!("开始扫描插件...");
        self.scan_status = "正在扫描插件...".to_string();
        
        match self.scanner.scan_all() {
            Ok(plugins) => {
                self.plugins = plugins;
                self.scan_status = format!("扫描完成！找到 {} 个插件", self.plugins.len());
                info!("{}", self.scan_status);
            }
            Err(e) => {
                self.scan_status = format!("扫描失败: {}", e);
                error!("{}", self.scan_status);
            }
        }
    }
    
    /// 获取过滤后的插件列表
    fn filtered_plugins(&self) -> Vec<&PluginInfo> {
        if self.search_filter.is_empty() {
            self.plugins.iter().collect()
        } else {
            let filter = self.search_filter.to_lowercase();
            self.plugins
                .iter()
                .filter(|p| {
                    p.metadata.name.to_lowercase().contains(&filter)
                        || p.metadata.vendor.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }
}

impl eframe::App for PluginLoaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("🔍 扫描插件").clicked() {
                        self.show_scan_window = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📂 打开工程").clicked() {
                        // TODO: 实现打开工程
                        ui.close_menu();
                    }
                    if ui.button("💾 保存工程").clicked() {
                        // TODO: 实现保存工程
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("❌ 退出").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("音频", |ui| {
                    if ui.button("⚙️ 音频设置").clicked() {
                        // TODO: 打开音频设置
                        ui.close_menu();
                    }
                    if ui.button("🎸 启动音频引擎").clicked() {
                        // TODO: 启动音频引擎
                        info!("启动音频引擎");
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("帮助", |ui| {
                    if ui.button("📖 使用指南").clicked() {
                        // TODO: 打开帮助
                        ui.close_menu();
                    }
                    if ui.button("ℹ️ 关于").clicked() {
                        // TODO: 显示关于对话框
                        ui.close_menu();
                    }
                });
            });
        });
        
        // 左侧面板 - 插件库
        egui::SidePanel::left("plugin_library")
            .default_width(300.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("🎸 插件库");
                ui.separator();
                
                // 搜索框
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    ui.text_edit_singleline(&mut self.search_filter);
                    if ui.button("❌").clicked() {
                        self.search_filter.clear();
                    }
                });
                
                ui.separator();
                
                // 插件统计
                let total = self.plugins.len();
                let valid = self.plugins.iter().filter(|p| p.valid).count();
                ui.label(format!("总计: {} 个插件 ({} 有效)", total, valid));
                
                ui.separator();
                
                // 插件列表
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let filtered = self.filtered_plugins();
                    
                    for (idx, plugin) in filtered.iter().enumerate() {
                        let is_selected = self.selected_plugin == Some(idx);
                        
                        let response = ui.selectable_label(
                            is_selected,
                            format!("{} {}", 
                                if plugin.valid { "✅" } else { "❌" },
                                plugin.metadata.name
                            )
                        );
                        
                        if response.clicked() {
                            self.selected_plugin = Some(idx);
                        }
                        
                        // 显示详细信息
                        if is_selected {
                            ui.indent("plugin_details", |ui| {
                                ui.small(format!("厂商: {}", plugin.metadata.vendor));
                                ui.small(format!("版本: {}", plugin.metadata.version));
                                if let Some(err) = &plugin.error {
                                    ui.colored_label(
                                        egui::Color32::from_rgb(255, 100, 100),
                                        format!("错误: {}", err)
                                    );
                                }
                            });
                        }
                    }
                });
            });
        
        // 中央面板 - 插件链
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔗 插件链");
            ui.separator();
            
            if self.loaded_plugins.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.label("暂无插件");
                    ui.label("从左侧插件库拖拽插件到此处");
                });
            } else {
                for (idx, plugin_name) in self.loaded_plugins.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}.", idx + 1));
                        ui.label(plugin_name);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("❌").clicked() {
                                // TODO: 移除插件
                            }
                            if ui.button("⚙️").clicked() {
                                // TODO: 打开插件 UI
                            }
                        });
                    });
                    ui.separator();
                }
            }
        });
        
        // 右侧面板 - 电平表和控制
        egui::SidePanel::right("meters")
            .default_width(150.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("🎚️ 电平表");
                ui.separator();
                
                // TODO: 实现实时电平表
                ui.label("输入:");
                ui.add(egui::ProgressBar::new(0.5).show_percentage());
                
                ui.add_space(10.0);
                
                ui.label("输出:");
                ui.add(egui::ProgressBar::new(0.3).show_percentage());
                
                ui.separator();
                
                // 控制按钮
                ui.vertical_centered(|ui| {
                    if ui.button("▶️ 播放").clicked() {
                        // TODO: 启动音频
                    }
                    if ui.button("⏸️ 暂停").clicked() {
                        // TODO: 暂停音频
                    }
                    if ui.button("⏹️ 停止").clicked() {
                        // TODO: 停止音频
                    }
                });
            });
        
        // 底部状态栏
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("就绪");
                ui.separator();
                ui.label(format!("插件: {}", self.loaded_plugins.len()));
                ui.separator();
                ui.label("采样率: 48000 Hz");
                ui.separator();
                ui.label("延迟: 5.3 ms");
            });
        });
        
        // 扫描窗口
        if self.show_scan_window {
            egui::Window::new("插件扫描")
                .default_width(400.0)
                .show(ctx, |ui| {
                    ui.label(&self.scan_status);
                    ui.separator();
                    
                    if ui.button("开始扫描").clicked() {
                        self.scan_plugins();
                    }
                    
                    if ui.button("关闭").clicked() {
                        self.show_scan_window = false;
                    }
                });
        }
    }
}

