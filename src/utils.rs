use dioxus:: prelude::*;
// use dioxus::web::window;

#[cfg(target_os = "android")]
pub fn initial_width() -> Signal<u32> {
    let mut width = use_signal(|| 0);
    use_effect(move || {
        spawn(async move {
            let listener_script =
                r#"dioxus.send(window.innerWidth);"#;
            let mut eval = document::eval(listener_script);            
                if let Ok(x) = eval.recv::<u32>().await {                   
                    width.set(x)
                }
            
        });
    });
    use_effect(move || {
        spawn(async move {
            let listener_script =
                r#"addEventListener("resize", ()=>dioxus.send(window.innerWidth));"#;
            let mut eval = document::eval(listener_script);
            loop {
                
                if let Ok(x) = eval.recv::<u32>().await {
                    width.set(x)
                }
            }
        });
    });
    width 
}

#[cfg(not(target_os = "android"))]
pub fn initial_width() -> Signal<u32> {
    let width = use_signal(|| 0);
    width
}


// fix temporaire juste pour testing de version web