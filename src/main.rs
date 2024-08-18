#![windows_subsystem = "windows"]
use slint::{PlatformError, SharedString};
slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let app = App::new().unwrap();
    let app_handler = app.as_weak().unwrap();
    app.on_generate(move || {
        let raw = app_handler.get_inputs();
        let mut str = SharedString::from("//Made by space.bilibili.com/9832924 \n\nbind n +next\nalias +next \"ang0;+attack;-attack;\"\nalias -next n0\n");
        let raw: Vec<&str> = raw.split_whitespace().collect();
        let len=raw.len();
        for i in 1..len{
            str+=format!("alias n{} \"-ang{} ;alias -next n{}\"\n",i-1,raw[i-1],i).as_str();
        }
        str+=format!("alias n{} \"-ang{} ;alias -next n0\"\n",len-1,raw[len-1]).as_str();
        app_handler.set_outputs(str);
    });
    app.run()
}
