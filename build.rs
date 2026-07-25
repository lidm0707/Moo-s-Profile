fn main() {
    dotenvy::dotenv().ok();

    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-changed=index.html");

    // Provide default values for missing environment variables
    let app_mode = std::env::var("APP_MODE").expect("not found ENV: APP_MODE");
    let supabase_url = std::env::var("SUPABASE_URL").expect("not found ENV: SUPABASE_URL");
    let supabase_anon_key =
        std::env::var("SUPABASE_ANON_KEY").expect("not found ENV: SUPABASE_ANON_KEY");

    // Print all relevant env vars (including defaults)
    println!("cargo:rustc-env=APP_MODE={}", app_mode);
    println!("cargo:rustc-env=SUPABASE_URL={}", supabase_url);
    println!("cargo:rustc-env=SUPABASE_ANON_KEY={}", supabase_anon_key);

    // Also print any other SUPABASE_ prefixed variables
    for (key, value) in std::env::vars() {
        if key.starts_with("SUPABASE_") && key != "SUPABASE_URL" && key != "SUPABASE_ANON_KEY" {
            println!("cargo:rustc-env={}={}", key, value);
        }
    }

    // Google AdSense. PLACEHOLDER_CLIENT_ID is the sentinel that marks an unset
    // publisher ID; the loader skips injection while ADSENSE_CLIENT_ID equals it.
    //
    // Source priority for the publisher ID:
    //   1. ADSENSE_CLIENT_ID env var (explicit override)
    //   2. parse `index.html` for the `adsbygoogle.js?client=ca-pub-XXXX` script
    //      (single source of truth — the script tag must be there for site
    //      verification anyway, so we reuse it instead of duplicating the id)
    //   3. PLACEHOLDER_CLIENT_ID (disables ads at runtime via the equality check)
    let placeholder_client_id = std::env::var("PLACEHOLDER_CLIENT_ID")
        .unwrap_or_else(|_| "ca-pub-0000000000000000".to_string());
    let adsense_client_id = std::env::var("ADSENSE_CLIENT_ID")
        .ok()
        .filter(|v| !v.is_empty() && v != &placeholder_client_id)
        .or_else(adsense_client_id_from_index_html)
        .unwrap_or_else(|| placeholder_client_id.clone());
    println!(
        "cargo:rustc-env=PLACEHOLDER_CLIENT_ID={}",
        placeholder_client_id
    );
    println!("cargo:rustc-env=ADSENSE_CLIENT_ID={}", adsense_client_id);
}

/// Extract the AdSense publisher id (`ca-pub-XXXXXXXXXXXXXXXX`) from the static
/// `index.html` by finding `adsbygoogle.js?client=ca-pub-` and reading the id
/// that follows. Returns None if the file is missing or the marker isn't found.
fn adsense_client_id_from_index_html() -> Option<String> {
    const MARKER: &str = "adsbygoogle.js?client=";
    let html = std::fs::read_to_string("index.html").ok()?;
    let start = html.find(MARKER)?;
    let tail = &html[start + MARKER.len()..];
    // publisher id = `ca-pub-` followed by digits, terminated by `"` or `&`/space
    let end = tail
        .find(|c: char| c == '"' || c == '&' || c.is_whitespace())
        .unwrap_or(tail.len());
    let id = &tail[..end];
    if id.starts_with("ca-pub-") {
        Some(id.to_string())
    } else {
        None
    }
}
