use codegenr_lib::{run_all_codegenr, Options};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::extract::State;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use tracing::{info, error, warn};
use std::io::Write;
use zip::ZipWriter;
use zip::result::ZipError;
use std::io::Cursor;
use serde::Serialize;
use std::fmt;
use crate::models::error::AppError;
use crate::models::codegenr::{GetCodeRequest, DownloadResponse};
use crate::models::language::Framework;
use crate::db::DatabaseManager;
use futures;

const CONFIG_TOML: &str = include_str!("../../assets/codegenr.toml");

/// Charge la configuration depuis le fichier TOML intégré
fn load_config_from_str() -> Result<HashMap<String, Options>, AppError> {
    let map: HashMap<String, Options> = toml::from_str(CONFIG_TOML)
        .map_err(|e| AppError::ConfigError(format!("Échec du parsing TOML: {}", e)))?;
    Ok(map)
}

/// Crée un répertoire s'il n'existe pas déjà
fn ensure_directory_exists(path: &Path) -> Result<(), AppError> {
    if !path.exists() {
        info!("Création du dossier: {}", path.display());
        fs::create_dir(path).map_err(|e| {
            error!("Erreur lors de la création du dossier {}: {}", path.display(), e);
            AppError::IoError(e)
        })?;
    }
    Ok(())
}

/// Crée un répertoire temporaire avec un UUID unique
fn create_temp_directory() -> Result<(PathBuf, Uuid), AppError> {
    let temp_dir = Path::new("./temp");
    ensure_directory_exists(temp_dir)?;
    
    let file_id = Uuid::new_v4();
    let uuid_dir = temp_dir.join(file_id.to_string());
    
    fs::create_dir(&uuid_dir).map_err(|e| {
        error!("Erreur lors de la création du dossier UUID: {}", e);
        AppError::IoError(e)
    })?;
    
    info!("Dossier UUID créé: {}", uuid_dir.display());
    Ok((uuid_dir, file_id))
}

/// Sauvegarde le contenu OpenAPI dans un fichier
fn save_openapi_to_file(uuid_dir: &Path, openapi: &str) -> Result<PathBuf, AppError> {
    let file_path = uuid_dir.join("openapi.yaml");
    
    fs::write(&file_path, openapi).map_err(|e| {
        error!("Erreur lors de l'écriture du fichier OpenAPI: {}", e);
        AppError::IoError(e)
    })?;
    
    info!("Fichier OpenAPI sauvegardé avec succès: {}", file_path.display());
    Ok(file_path)
}

async fn get_sqlx_query(db: &DatabaseManager, framework_id: i32) -> Result<Framework, AppError> {
    sqlx::query_as!(
        Framework,
        r#"
        SELECT 
            id, 
            name, 
            language_id, 
            f_type::text as "f_type?", 
            codegenr_config as "codegenr_config?", 
            created_at as "created_at?", 
            updated_at as "updated_at?"
        FROM frameworks 
        WHERE id = $1
        "#,
        framework_id
    )
    .fetch_optional(db.get_pool())
    .await
    .map_err(|e| AppError::ConfigError(format!("Erreur lors de la récupération du framework: {}", e)))?
    .ok_or_else(|| AppError::NotFound(format!("Framework avec l'ID {} non trouvé", framework_id)))
}

async fn get_config_for_framework(db: &DatabaseManager, framework_ids: Vec<i32>) -> Result<HashMap<String, Options>, AppError> {
    let mut all_configs = load_config_from_str()?;
    let mut selected_configs = HashMap::new();
    
    futures::future::join_all(
        framework_ids.iter().map(|&id| get_sqlx_query(db, id))
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>()?
    .into_iter()
    .filter_map(|framework| framework.codegenr_config)
    .flatten()
    .for_each(|config_name| {
        if let Some(config) = all_configs.remove(&config_name) {
            selected_configs.insert(config_name, config);
        }
    });
    
    Ok(selected_configs)
}

/// Génère le code à partir du fichier OpenAPI
async fn generate_code(db: &DatabaseManager, file_path: &Path, uuid_dir: &Path, framework_ids: Vec<i32>) -> Result<(), AppError> {
    let mut config = get_config_for_framework(db, framework_ids).await?;
    
    let models_dir = uuid_dir.join("models");
    fs::create_dir(&models_dir).map_err(|e| {
        error!("Erreur lors de la création du dossier models: {}", e);
        AppError::IoError(e)
    })?;
    
    // Mettre à jour les chemins de sortie pour toutes les configurations
    for options in config.values_mut() {
        options.source = file_path.to_string_lossy().into_owned();
        let output_path = Path::new(&options.output);
        let last_segment = output_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("models");
        options.output = models_dir.join(last_segment).to_string_lossy().into_owned();
    }
    
    // Exécuter toutes les générations en une seule fois
    run_all_codegenr(config).map_err(|e| {
        error!("Erreur lors de la génération de code: {}", e);
        AppError::CodegenError(e.to_string())
    })?;
    
    Ok(())
}

/// Zippe un répertoire et retourne les données binaires
fn zip_directory(dir_path: &Path) -> Result<Vec<u8>, AppError> {
    let mut zip_buffer = Vec::with_capacity(1024 * 1024); // Pré-allouer 1MB
    let mut zip = ZipWriter::new(Cursor::new(&mut zip_buffer));
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(Some(9)); // Niveau de compression plus bas pour plus de vitesse

    fn add_dir_to_zip(
        zip: &mut ZipWriter<Cursor<&mut Vec<u8>>>,
        dir_path: &Path,
        base_path: &Path,
        options: zip::write::FileOptions<()>,
    ) -> Result<(), AppError> {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.strip_prefix(base_path).unwrap();

            if path.is_dir() {
                zip.add_directory(name.to_string_lossy(), options)?;
                add_dir_to_zip(zip, &path, base_path, options)?;
            } else {
                zip.start_file(name.to_string_lossy(), options)?;
                zip.write_all(&fs::read(&path)?)?;
            }
        }
        Ok(())
    }

    add_dir_to_zip(&mut zip, dir_path, dir_path, options)?;
    zip.finish()?;
    Ok(zip_buffer)
}

/// Sauvegarde les données zip dans un fichier public
fn save_zip_to_public(zip_data: Vec<u8>, file_id: &Uuid) -> Result<PathBuf, AppError> {
    let public_dir = Path::new("./public");
    ensure_directory_exists(public_dir)?;
    
    let zip_path = public_dir.join(format!("{}.zip", file_id));
    
    fs::write(&zip_path, zip_data).map_err(|e| {
        error!("Erreur lors de la sauvegarde du zip: {}", e);
        AppError::IoError(e)
    })?;
    
    info!("Fichier zip sauvegardé dans le dossier public: {}", zip_path.display());
    Ok(zip_path)
}

/// Nettoie le répertoire temporaire
fn cleanup_temp_directory(uuid_dir: &Path) {
    if let Err(e) = fs::remove_dir_all(uuid_dir) {
        warn!("Erreur lors du nettoyage du dossier temporaire: {}", e);
    } else {
        info!("Dossier temporaire nettoyé: {}", uuid_dir.display());
    }
}

/// Point d'entrée principal pour la génération de code
#[axum::debug_handler]
pub async fn get_code(
    State(db): State<DatabaseManager>,
    request: Json<GetCodeRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let start = std::time::Instant::now();
    
    // Étape 1: Créer un répertoire temporaire
    let (uuid_dir, file_id) = create_temp_directory().map_err(StatusCode::from)?;
    
    // Étape 2: Sauvegarder le contenu OpenAPI
    let file_path = save_openapi_to_file(&uuid_dir, &request.openapi).map_err(StatusCode::from)?;
    
    // Étape 3: Générer le code
    generate_code(&db, &file_path, &uuid_dir, request.framework_id.clone())
        .await
        .map_err(StatusCode::from)?;
    
    // Étape 4: Zipper le répertoire
    let zip_data = zip_directory(&uuid_dir).map_err(StatusCode::from)?;
    
    // Étape 5: Sauvegarder le zip dans le répertoire public
    let zip_path = save_zip_to_public(zip_data.clone(), &file_id).map_err(StatusCode::from)?;
    
    // Étape 6: Nettoyer le répertoire temporaire
    cleanup_temp_directory(&uuid_dir);
    
    // Étape 7: Retourner l'URL de téléchargement
    let download_url = format!("/api/download/{}.zip", file_id);
    
    let duration = start.elapsed();
    info!("Temps total de traitement: {:?}", duration);
    
    Ok(Json(DownloadResponse { 
        download_url,
        size_bytes: zip_data.len() as u64,
    }))
}