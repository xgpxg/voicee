use crate::utils::{decrypt_data, encrypt_data};
use aha::models::voxcpm::generate::VoxCPMGenerate;
use aha::utils::audio_utils::save_wav;
use hex;
use serde::Deserialize;
use serde_json;
use short_uuid::short;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use tauri::{AppHandle, Manager};

static MODEL: OnceLock<Arc<Mutex<VoxCPMGenerate>>> = OnceLock::new();
const VOICE_MODEL_ID: &'static str = "openbmb/VoxCPM-0.5B";

const VERIFY_VERSION: &'static str = "v1";

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
pub(crate) async fn voice_clone(
    app_handle: AppHandle,
    input: VoiceCloneInput,
) -> Result<String, String> {
    verify(app_handle)?;

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

#[tauri::command]
pub(crate) fn verify(app_handle: AppHandle) -> Result<(), String> {
    let verify_file = app_handle
        .path()
        .config_dir()
        .map_err(|e| e.to_string())?
        .join("eeciov")
        .join(".eeciov");

    let verify_file_back = app_handle
        .path()
        .temp_dir()
        .map_err(|e| e.to_string())?
        .join(".eeciov");

    let _ = fs::create_dir_all(verify_file.parent().unwrap());

    if !verify_file.exists() && !verify_file_back.exists() {
        // 文件不存在，创建新的验证数据
        let now = chrono::Local::now().timestamp();
        let exp_ts = now + 30; //86400 * 30; // 30天后过期

        let id = get_unique_id(app_handle)?;
        // 格式：<VERIFY_VERSION>;<ID>;<EXP_TS>
        let verify_data = format!("{};{};{}", VERIFY_VERSION, id, exp_ts);

        let encrypted_data = encrypt_data(&verify_data)?;
        // 将加密数据转换为十六进制字符串
        let hex_encrypted_data = encrypted_data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        fs::write(&verify_file, &hex_encrypted_data).map_err(|e| e.to_string())?;
        fs::write(&verify_file_back, &hex_encrypted_data).map_err(|e| e.to_string())?;
        return Ok(());
    }

    if verify_file_back.exists() {
        // 从备份文件恢复
        let _ = fs::copy(verify_file_back, &verify_file);
    }

    // 文件存在，读取并解密验证数据
    let hex_encrypted_data = fs::read_to_string(&verify_file)
        .map_err(|e| e.to_string())?
        .to_lowercase();
    // 将十六进制字符串转换回字节数组
    let encrypted_data = hex::decode(&hex_encrypted_data).map_err(|e| e.to_string())?;
    let verify_data = decrypt_data(&encrypted_data)?;

    // 格式：<VERIFY_VERSION>;<ID>;<EXP_TS>
    let parts: Vec<&str> = verify_data.split(";").collect();
    if parts.len() != 3 {
        return Err("无效的激活码[001]".to_string());
    }
    if parts[0] != VERIFY_VERSION {
        return Err("无效的激活码[002]".to_string());
    }

    let exp_ts = parts[2].parse::<i64>().map_err(|_| "无效的激活码[003]")?;

    // 检查是否过期
    let current_time = chrono::Local::now().timestamp();

    if exp_ts < current_time {
        return Err("未激活".to_string());
    }
    Ok(())
}

/// code格式：code;<VERIFY_VERSION>;<ID>;<EXP_TS>
#[tauri::command]
pub(crate) fn update_verify_code(app_handle: AppHandle, code: String) -> Result<(), String> {
    println!("update_verify_code: {}", code);
    let encrypted_data =
        hex::decode(&code.to_lowercase()).map_err(|e| "无效的激活码[004]".to_string())?;
    let verify_data = decrypt_data(&encrypted_data).map_err(|e| "无效的激活码[005]".to_string())?;
    let parts: Vec<&str> = verify_data.split(";").collect();
    if parts.len() != 4 {
        return Err("无效的激活码[006]".to_string());
    }
    // 激活码前缀为"code;"
    if parts[0] != "code" {
        return Err("无效的激活码[007]".to_string());
    }
    // 版本不匹配
    if parts[1] != VERIFY_VERSION {
        return Err("无效的激活码[008]".to_string());
    }
    // 检查ID
    if parts[2] != get_unique_id(app_handle.clone())? {
        return Err("无效的激活码[009]".to_string());
    }
    let data = format!("{};{};{}", parts[1], parts[2], parts[3]);
    let encrypted_data = encrypt_data(&data)?;

    let verify_file = app_handle
        .path()
        .config_dir()
        .map_err(|_| "验证失败[001]".to_string())?
        .join("eeciov")
        .join(".eeciov");
    let verify_file_back = app_handle
        .path()
        .temp_dir()
        .map_err(|_| "验证失败[002]".to_string())?
        .join(".eeciov");

    let hex_encrypted_data = encrypted_data
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    fs::write(&verify_file, &hex_encrypted_data).map_err(|e| e.to_string())?;
    fs::write(&verify_file_back, &hex_encrypted_data).map_err(|e| e.to_string())?;

    Ok(())
}
/// 生成8位随机ID，包含数字和字母，全大写
#[tauri::command]
pub(crate) fn gen_unique_id(app_handle: AppHandle) -> Result<(), String> {
    let file = app_handle
        .path()
        .app_config_dir()
        .map_err(|_| "环境检查失败[001]".to_string())?
        .join(".ID");

    if file.exists() {
        return Ok(());
    }

    let _ = fs::create_dir_all(file.parent().unwrap());

    println!("dir: {}", file.display());
    let id = &short!().to_string()[..8].to_uppercase();

    fs::write(file, id).map_err(|_| "环境检查失败[002]".to_string())?;

    Ok(())
}

#[tauri::command]
pub(crate) fn get_unique_id(app_handle: AppHandle) -> Result<String, String> {
    let file = app_handle
        .path()
        .app_config_dir()
        .map_err(|_| "环境检查失败[003]".to_string())?
        .join(".ID");
    if !file.exists() {
        return Err("环境检查失败[004]".to_string());
    }
    Ok(fs::read_to_string(file).map_err(|_| "环境检查失败[005]".to_string())?)
}
