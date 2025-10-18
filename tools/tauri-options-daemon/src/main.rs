use std::{
    collections::HashMap,
    env, fs,
    net::SocketAddr,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use jsonrpsee::core::RpcResult;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use jsonrpsee::RpcModule;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
enum NoiseLevel {
    Polite,
    LoudAndProud,
    FranklyQuitePedantic,
}

impl Default for NoiseLevel {
    fn default() -> Self {
        Self::Polite
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TargetDevice {
    id: String,
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CliOptions {
    dev: bool,
    features: Option<Vec<String>>,
    args: Vec<String>,
    noise_level: NoiseLevel,
    vars: HashMap<String, String>,
    config: Vec<Value>,
    target_device: Option<TargetDevice>,
}

impl Default for CliOptions {
    fn default() -> Self {
        Self {
            dev: false,
            features: None,
            args: vec!["--lib".into()],
            noise_level: NoiseLevel::default(),
            vars: collect_env_vars(),
            config: Vec::new(),
            target_device: None,
        }
    }
}

fn main() -> Result<()> {
    let repo_root = locate_repo_root()?;
    let identifier = resolve_identifier(&repo_root)?;

    let options = CliOptions::default();
    let (lease, socket_addr) = spawn_server(options)?;
    write_addr_file(&identifier, socket_addr)?;

    println!("tauri-options-daemon listening on ws://{socket_addr}");

    let _lease = lease;
    loop {
        thread::sleep(Duration::from_secs(30));
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn locate_repo_root() -> Result<PathBuf> {
    if let Ok(root) = env::var("UNICEL_REPO_ROOT") {
        return Ok(PathBuf::from(root));
    }

    let current = env::current_dir().context("failed to resolve current directory")?;
    for ancestor in current.ancestors() {
        if ancestor.join("src-tauri/tauri.conf.json").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }

    Err(anyhow::anyhow!(
        "Unable to locate project root containing src-tauri/tauri.conf.json"
    ))
}

fn resolve_identifier(repo_root: &Path) -> Result<String> {
    if let Ok(id) = env::var("TAURI_APP_IDENTIFIER") {
        if !id.is_empty() {
            return Ok(id);
        }
    }

    let config_path = repo_root.join("src-tauri/tauri.conf.json");
    let raw = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    let parsed: Value = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse {}", config_path.display()))?;
    if let Some(id) = parsed.get("identifier").and_then(Value::as_str) {
        if !id.is_empty() {
            return Ok(id.to_string());
        }
    }

    Ok(String::from("com.unicel.app"))
}

fn collect_env_vars() -> HashMap<String, String> {
    let mut vars = HashMap::new();
    vars.insert("RUST_LOG_STYLE".into(), "always".into());

    for (key, value) in env::vars() {
        if key.starts_with("TAURI")
            || key.starts_with("WRY")
            || key.starts_with("CARGO_")
            || key.starts_with("RUST_")
            || key == "TMPDIR"
            || key == "PATH"
        {
            vars.insert(key, value);
        }
    }

    vars
}

struct ServerLease {
    #[allow(dead_code)]
    runtime: Runtime,
    #[allow(dead_code)]
    handle: ServerHandle,
}

fn spawn_server(options: CliOptions) -> Result<(ServerLease, SocketAddr)> {
    let runtime = Runtime::new().context("failed to initialize tokio runtime")?;
    let (handle, addr) = runtime
        .block_on(async move {
            let server = ServerBuilder::default()
                .build("127.0.0.1:0")
                .await
                .context("failed to build JSON-RPC server")?;

            let snapshot = options.clone();
            let mut module = RpcModule::new(());
            module
                .register_method("options", move |_, _, _| -> RpcResult<CliOptions> {
                    Ok(snapshot.clone())
                })
                .context("failed to register options method")?;

            let addr = server.local_addr().context("failed to get local address")?;
            let handle = server.start(module);

            Ok::<(ServerHandle, SocketAddr), anyhow::Error>((handle, addr))
        })
        .context("failed to launch JSON-RPC server")?;

    Ok((ServerLease { runtime, handle }, addr))
}

fn write_addr_file(identifier: &str, addr: SocketAddr) -> Result<()> {
    let path = env::temp_dir().join(format!("{identifier}-server-addr"));
    fs::write(&path, addr.to_string())
        .with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}
