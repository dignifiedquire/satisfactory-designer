#[cfg(not(target_arch = "wasm32"))]
pub fn load_img(name: &str) -> String {
    format!("file://assets/img/{name}")
}

#[cfg(target_arch = "wasm32")]
pub fn load_img(name: &str) -> String {
    let root = std::env!("TRUNK_PUBLIC_URL");

    format!("{root}/img/{name}")
}
