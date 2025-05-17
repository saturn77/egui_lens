use eframe::egui;
use egui_lens::{ReactiveEventLogger, ReactiveEventLoggerState, LogColors};
use egui_mobius_reactive::Dynamic;

// Import platform modules
mod platform;
use platform::{banner, details, parameters::gui};

/// This example demonstrates how to use the custom log types with string identifiers.
/// It shows creating logs with different custom types and configuring their colors.
fn main() -> Result<(), eframe::Error> {
    // Set up the environment for the example
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_titlebar_buttons_shown(true)
            .with_inner_size([gui::VIEWPORT_X*1.25, gui::VIEWPORT_Y*1.25])
            .with_min_inner_size([gui::VIEWPORT_X, gui::VIEWPORT_Y])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Custom Log Types Example",
        native_options,
        Box::new(|cc| Ok(Box::new(ExampleApp::new(cc)))),
    )
}

/// Main application struct
struct ExampleApp {
    // Dynamic shared state
    logger_state: Dynamic<ReactiveEventLoggerState>,
    log_colors: Dynamic<LogColors>,
    banner: banner::Banner,
    details: details::Details,
}

impl ExampleApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Create shared state for the logger
        let logger_state = Dynamic::new(ReactiveEventLoggerState::new());
        
        // Create shared colors with custom configurations
        let mut log_colors = Dynamic::new(LogColors::default());
        
        // Configure colors for various custom log types
        configure_custom_log_colors(&mut log_colors);
        
        // Create banner and details instances
        let mut banner = banner::Banner::new();
        let mut details = details::Details::new();
        
        // Format banner and get system info
        banner.format();
        details.get_os();
        
        // Create the app
        let app = Self {
            logger_state,
            log_colors,
            banner,
            details,
        };
        
        // Add some example logs including system info
        app.add_example_logs();
        
        app
    }
    
    /// Add example logs to demonstrate different log types
    fn add_example_logs(&self) {
        // Create a logger using references to our Dynamic state
        let logger = ReactiveEventLogger::with_colors(&self.logger_state, &self.log_colors);
        
        // Log banner message (welcome message)
        logger.log_info(&self.banner.message);
        
        // Log system details
        let details_text = self.details.clone().format_os();
        logger.log_info(&details_text);
        
        // Log standard log types to demonstrate different colors
        logger.log_info("This is a standard info message");
        logger.log_warning("This is a standard warning message");
        logger.log_error("This is a standard error message");
        logger.log_debug("This is a standard debug message");
        
        // Log with custom types to demonstrate custom type colors
        logger.log_custom("network", "Connected to server on port 8080");
        logger.log_custom("database", "Executed query in 42ms");
        logger.log_custom("security", "User authentication successful");
        logger.log_custom("performance", "Rendering took 16ms");
        logger.log_custom("analytics", "Page view recorded for /dashboard");
        logger.log_custom("http", "GET /api/users - 200 OK - 12ms");
        logger.log_custom("websocket", "Client connected: user_123");
        logger.log_custom("auth", "JWT token issued");
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for "show system info" request from the logger
        let show_system_info = ctx.memory(|mem| {
            mem.data.get_temp::<bool>(egui::Id::new("show_system_info")).unwrap_or(false)
        });
        
        if show_system_info {
            // Clear the flag
            ctx.memory_mut(|mem| {
                mem.data.remove::<bool>(egui::Id::new("show_system_info"));
            });
            
            // Create a logger to display system info
            let logger = ReactiveEventLogger::with_colors(&self.logger_state, &self.log_colors);
            
            // Log system details
            let details_text = self.details.format_os();
            logger.log_info(&details_text);
            
            // Then log banner message
            logger.log_info(&self.banner.message);
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Custom Log Types Example");
            ui.add_space(8.0);
            
            ui.label("This example demonstrates the flexible custom log types feature.");
            ui.label("Each custom log type has its own specific color and identifier.");
            ui.add_space(16.0);
            
            // Create a logger for this frame using references to our Dynamic state
            let logger = ReactiveEventLogger::with_colors(&self.logger_state, &self.log_colors);
            
            // Display the logger
            logger.show(ui);
            
            // Add buttons to insert more logs
            ui.add_space(16.0);
            ui.heading("Add more logs");
            
            ui.horizontal(|ui| {
                if ui.button("System Info").clicked() {
                    // Get fresh system info
                    let details_text = self.details.format_os();
                    logger.log_info(&details_text);
                }
                
                if ui.button("Add Network Log").clicked() {
                    logger.log_custom("network", "New client connected from 192.168.1.5");
                }
                
                if ui.button("Add Database Log").clicked() {
                    logger.log_custom("database", "Inserted 5 records in 18ms");
                }
            });
            
            ui.horizontal(|ui| {
                if ui.button("Add Security Log").clicked() {
                    logger.log_custom("security", "Failed login attempt: incorrect password");
                }
                
                if ui.button("Add Custom HTTP Log").clicked() {
                    logger.log_custom("http", "POST /api/data - 201 Created - 45ms");
                }
                
                if ui.button("Add Standard Info Log").clicked() {
                    logger.log_info("This is a standard info message");
                }
            });
            
            ui.horizontal(|ui| {
                if ui.button("Add Standard Warning").clicked() {
                    logger.log_warning("This is a standard warning message");
                }
                
                if ui.button("Add Standard Error").clicked() {
                    logger.log_error("This is a standard error message");
                }
                
                if ui.button("Add Standard Debug").clicked() {
                    logger.log_debug("This is a standard debug message");
                }
            });
        });
    }
}

/// Configure colors for the custom log types
fn configure_custom_log_colors(colors: &mut Dynamic<LogColors>) {
    // Get mutable access to the colors
    let mut colors_value = colors.get();
    
    // Set simple colors (same for level and message)
    colors_value.set_custom_color("network", egui::Color32::from_rgb(100, 149, 237)); // Cornflower blue
    colors_value.set_custom_color("database", egui::Color32::from_rgb(106, 90, 205)); // Slate blue
    colors_value.set_custom_color("security", egui::Color32::from_rgb(60, 179, 113)); // Medium sea green
    
    // Set different colors for level and message
    colors_value.set_custom_colors(
        "performance", 
        egui::Color32::from_rgb(255, 165, 0),    // Bright orange for level
        egui::Color32::from_rgb(255, 215, 140)   // Lighter orange for message
    );
    
    colors_value.set_custom_colors(
        "analytics", 
        egui::Color32::from_rgb(218, 112, 214),  // Orchid for level
        egui::Color32::from_rgb(230, 175, 228)   // Lighter orchid for message
    );
    
    colors_value.set_custom_colors(
        "http", 
        egui::Color32::from_rgb(70, 130, 180),   // Steel blue for level
        egui::Color32::from_rgb(150, 190, 220)   // Lighter blue for message
    );
    
    colors_value.set_custom_colors(
        "websocket", 
        egui::Color32::from_rgb(0, 139, 139),    // Dark cyan for level
        egui::Color32::from_rgb(100, 200, 200)   // Lighter cyan for message
    );
    
    colors_value.set_custom_colors(
        "auth", 
        egui::Color32::from_rgb(85, 107, 47),    // Dark olive green for level
        egui::Color32::from_rgb(160, 200, 120)   // Lighter olive green for message
    );
    
    // Update the dynamic value
    colors.set(colors_value);
}