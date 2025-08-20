// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 在开发模式下启用日志
    #[cfg(debug_assertions)]
    {
        // 启用日志记录
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();
    }
    
    // 使用panic hook捕获任何panic
    std::panic::set_hook(Box::new(|_panic_info| {
        // 在实际应用中，你可能想要将这个信息保存到日志文件中
    }));
    
    match cadremanagementsystem_lib::run() {
        Ok(_) => (),
        Err(_e) => {
            // 保持控制台窗口打开，以便查看错误信息
            #[cfg(debug_assertions)]
            {
                use std::io::Read;
                let _ = std::io::stdin().read(&mut [0u8]).unwrap();
            }
        }
    }
}
