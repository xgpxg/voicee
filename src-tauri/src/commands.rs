use aha::models::voxcpm::generate::VoxCPMGenerate;
use aha::utils::audio_utils::save_wav;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

static MODEL: OnceLock<Arc<Mutex<VoxCPMGenerate>>> = OnceLock::new();
const VOICE_MODEL_ID: &'static str = "openbmb/VoxCPM-0.5B";

fn get_app_dir() -> Result<PathBuf, String> {
    let home_dir = std::env::home_dir().ok_or("no home dir")?;
    let app_dir = home_dir.join(".voice-app");
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    }
    Ok(app_dir)
}

fn get_model_dir() -> Result<PathBuf, String> {
    let app_dir = get_app_dir()?;
    let model_dir = app_dir.join("models");
    if !model_dir.exists() {
        fs::create_dir_all(&model_dir).unwrap();
    }
    Ok(model_dir)
}

#[tauri::command]
pub(crate) async fn is_initialized() -> Result<bool, String> {
    let model_path = get_model_dir()?.join(VOICE_MODEL_ID);
    let is_download = fs::exists(model_path.join(".DOWNLOADED")).map_err(|e| e.to_string())?;
    Ok(is_download)
}
#[tauri::command]
pub(crate) async fn download_model() -> Result<(), String> {
    let model_dir = get_model_dir()?;
    modelscope::ModelScope::download(VOICE_MODEL_ID, &model_dir)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn load_model() -> Result<(), String> {
    let model_path = get_model_dir()?.join(VOICE_MODEL_ID);
    MODEL.get_or_init(|| {
        Arc::new(Mutex::new(
            VoxCPMGenerate::init(model_path.display().to_string().as_str(), None, None)
                .map_err(|e| e.to_string())
                .unwrap(),
        ))
    });
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct VoiceCloneInput {
    pub voice_file: String,
    pub voice_text: String,
    pub input: String,
}

#[tauri::command]
pub(crate) async fn voice_clone(input: VoiceCloneInput) -> Result<String, String> {
    let app_dir = get_app_dir()?;

    let voxcpm_generate = MODEL.get().ok_or("model not load")?;

    let generate = voxcpm_generate
        .lock()
        .unwrap()
        .inference(
            input.input,
            input.voice_text.into(),
            format!("file://{}", input.voice_file).into(),
            2,
            1000000,
            10,
            2.0,
            6.0,
        )
        .map_err(|e| e.to_string())?;

    let save_path = app_dir.join("output");
    if !Path::new(&save_path).exists() {
        fs::create_dir_all(&save_path).map_err(|e| e.to_string())?;
    }
    let save_file = save_path
        .join(format!(
            "output_{}.wav",
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ))
        .display()
        .to_string();
    save_wav(&generate, &save_file, 16000).map_err(|e| e.to_string())?;

    Ok(save_file)
}
