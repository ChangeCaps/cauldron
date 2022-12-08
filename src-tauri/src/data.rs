use cauldron_data::Data;

const APP_DATA_ERROR: &str = "Failed to get app data directory";

#[tauri::command]
pub async fn load_data(app_handle: tauri::AppHandle) -> Result<Data, String> {
    let app_data = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(APP_DATA_ERROR)?;

    let inventory_path = app_data.join("data.ron");

    let source = tokio::fs::read_to_string(inventory_path).await;
    let Ok(source) = source else {
        return Ok(Data::default());
    };

    let inventory = ron::de::from_str(&source).map_err(|e| e.to_string())?;
    Ok(inventory)
}

#[tauri::command]
pub async fn store_data(data: Data, app_handle: tauri::AppHandle) -> Result<(), String> {
    let app_data = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(APP_DATA_ERROR)?;

    let inventory_path = app_data.join("data.ron");

    let source =
        ron::ser::to_string_pretty(&data, Default::default()).map_err(|e| e.to_string())?;
    tokio::fs::write(inventory_path, source)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
