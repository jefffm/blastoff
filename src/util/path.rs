// TODO: normalize paths in the same way as macroquad
pub fn get_asset_path() -> String {
    #[cfg(target_os = "ios")]
    let _ = std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap());

    #[cfg(not(target_os = "android"))]
    let path = if let Some(ref pc_assets) = crate::get_context().pc_assets_folder {
        format!("{}/{}", pc_assets, path)
    } else {
        path.to_string()
    };
}
