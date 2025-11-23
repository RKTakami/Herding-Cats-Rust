use anyhow::Result;
use slint::{ComponentHandle, Model, VecModel, SharedString};
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use crate::font_manager::{FontManager, FontInfo, init_font_manager, get_font_manager};

slint::include_modules!();



pub struct FontManagerWindowManager {
    window: FontManagerWindow,
}

impl FontManagerWindowManager {
    pub fn new() -> Result<Self> {
        let window = FontManagerWindow::new()?;
        let instance = Self { window };
        instance.init();
        Ok(instance)
    }

    fn init(&self) {
        let window_weak = self.window.as_weak();
        
        // Initial load
        self.refresh_fonts();

        // Install callback
        let window_weak_install = self.window.as_weak();
        self.window.on_install_font(move |family| {
             let family = family.to_string();
             let window_weak = window_weak_install.clone();
             
             // Spawn on a thread to avoid blocking UI
             std::thread::spawn(move || {
                 let _ = match get_font_manager() {
                     Ok(mut guard) => {
                         if let Some(fm) = guard.as_mut() {
                             let _ = fm.download_and_install_font(&family);
                         }
                     },
                     Err(_) => {},
                 };
                 
                 // Refresh list after install attempt
                 // We need to invoke back to UI thread to update
                 let _ = slint::invoke_from_event_loop(move || {
                     if let Some(window) = window_weak.upgrade() {
                         if let Ok(guard) = get_font_manager() {
                            if let Some(fm) = guard.as_ref() {
                                let fonts: Vec<FontInfo> = fm.get_available_fonts().into_iter().cloned().collect();
                                let font_items: Vec<FontItem> = fonts.into_iter().map(|f| FontItem {
                                    name: f.name.into(),
                                    family: f.family.into(),
                                    category: f.category.into(),
                                    installed: f.installed,
                                }).collect();
                                let model = VecModel::from(font_items);
                                window.set_fonts(Rc::new(model).into());
                            }
                         }
                     }
                 });
             });
        });

        let window_weak_close = self.window.as_weak();
        self.window.on_close_requested(move || {
            if let Some(window) = window_weak_close.upgrade() {
                let _ = window.hide();
            }
        });
    }
    
    pub fn show(&self) -> Result<()> {
        self.window.show().map_err(|e| anyhow::anyhow!("Failed to show window: {}", e))
    }
    
    fn refresh_fonts(&self) {
        let window_weak = self.window.as_weak();
        // Run in background to avoid locking UI if lock is contended
        std::thread::spawn(move || {
            let fonts = if let Ok(guard) = get_font_manager() {
                if let Some(fm) = guard.as_ref() {
                    fm.get_available_fonts().into_iter().cloned().collect()
                } else {
                    vec![]
                }
            } else {
                vec![]
            };

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(window) = window_weak.upgrade() {
                    let font_items: Vec<FontItem> = fonts.into_iter().map(|f| FontItem {
                        name: f.name.into(),
                        family: f.family.into(),
                        category: f.category.into(),
                        installed: f.installed,
                    }).collect();
                    let model = VecModel::from(font_items);
                    window.set_fonts(Rc::new(model).into());
                }
            });
        });
    }
}
