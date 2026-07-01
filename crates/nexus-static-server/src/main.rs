use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::thread;

#[derive(Debug)]
struct Config {
    root: PathBuf,
    host: String,
    port: u16,
    index: String,
}

fn main() -> Result<()> {
    let config = parse_args()?;
    let listener = TcpListener::bind((config.host.as_str(), config.port))
        .with_context(|| format!("failed to bind {}:{}", config.host, config.port))?;
    let addr = listener.local_addr()?;
    println!(
        "NEXUS_STATIC_SERVER_URL=http://{}:{}/",
        addr.ip(),
        addr.port()
    );
    std::io::stdout().flush().ok();

    let root = fs::canonicalize(&config.root)
        .with_context(|| format!("failed to canonicalize root {}", config.root.display()))?;
    for stream in listener.incoming() {
        let root = root.clone();
        let index = config.index.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(error) = handle_client(stream, &root, &index) {
                        eprintln!("nexus-static-server request failed: {error:#}");
                    }
                });
            }
            Err(error) => eprintln!("nexus-static-server accept failed: {error}"),
        }
    }
    Ok(())
}

fn parse_args() -> Result<Config> {
    let mut root: Option<PathBuf> = None;
    let mut host = String::from("127.0.0.1");
    let mut port = 0u16;
    let mut index = String::from("index.html");

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().map(PathBuf::from),
            "--host" => host = args.next().unwrap_or(host),
            "--port" => {
                let value = args.next().context("--port requires a value")?;
                port = value
                    .parse()
                    .with_context(|| format!("invalid port {value}"))?;
            }
            "--index" => index = args.next().unwrap_or(index),
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            other => bail!("unknown argument: {other}"),
        }
    }

    let root = root.context("--root is required")?;
    Ok(Config {
        root,
        host,
        port,
        index,
    })
}

fn print_help() {
    println!("nexus-static-server --root <dir> [--host 127.0.0.1] [--port 0] [--index index.html]");
}

fn handle_client(mut stream: TcpStream, root: &Path, index: &str) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let raw_path = parts.next().unwrap_or("/");

    while {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        !line.trim_end().is_empty()
    } {}

    if method != "GET" && method != "HEAD" {
        write_response(
            &mut stream,
            405,
            "text/plain; charset=utf-8",
            b"Method not allowed",
        )?;
        return Ok(());
    }

    let requested = sanitize_url_path(raw_path);
    let mut file_path = root.join(&requested);
    if file_path.is_dir() {
        file_path = file_path.join(index);
    }

    let canonical = fs::canonicalize(&file_path).ok();
    let valid = canonical
        .as_ref()
        .map(|path| path.starts_with(root) && path.is_file())
        .unwrap_or(false);

    let resolved = if valid {
        canonical.unwrap()
    } else {
        root.join(index)
    };

    if !resolved.exists() {
        write_response(&mut stream, 404, "text/plain; charset=utf-8", b"Not found")?;
        return Ok(());
    }

    let content_type = content_type(&resolved);
    let mut bytes = Vec::new();
    fs::File::open(&resolved)?.read_to_end(&mut bytes)?;
    if method == "HEAD" {
        write_head(&mut stream, 200, content_type, bytes.len())?;
    } else {
        write_response(&mut stream, 200, content_type, &bytes)?;
    }
    Ok(())
}

fn sanitize_url_path(raw_path: &str) -> PathBuf {
    let without_query = raw_path.split(['?', '#']).next().unwrap_or("/");
    let decoded = percent_decode(without_query.trim_start_matches('/'));
    let mut clean = PathBuf::new();
    for component in Path::new(&decoded).components() {
        if let Component::Normal(part) = component {
            clean.push(part);
        }
    }
    clean
}

fn percent_decode(value: &str) -> String {
    let mut out = Vec::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = u8::from_str_radix(&value[i + 1..i + 3], 16) {
                out.push(hex);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn content_type(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
    {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "wasm" => "application/wasm",
        "ico" => "image/x-icon",
        "glb" => "model/gltf-binary",
        _ => "application/octet-stream",
    }
}

fn write_head(stream: &mut TcpStream, status: u16, content_type: &str, len: usize) -> Result<()> {
    let reason = status_reason(status);
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Length: {len}\r\nContent-Type: {content_type}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n"
    )?;
    Ok(())
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> Result<()> {
    write_head(stream, status, content_type, body.len())?;
    stream.write_all(body)?;
    Ok(())
}

fn status_reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "OK",
    }
}
