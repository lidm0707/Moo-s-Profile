fn main() {
    dotenvy::dotenv().ok();

    println!("cargo:rerun-if-changed=.env");

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
    // Both are optional and fall back to the same placeholder so the build never
    // breaks when the env is unset (see components/adsense.rs / main.rs).
    let placeholder_client_id = std::env::var("PLACEHOLDER_CLIENT_ID")
        .unwrap_or_else(|_| "ca-pub-0000000000000000".to_string());
    let adsense_client_id =
        std::env::var("ADSENSE_CLIENT_ID").unwrap_or_else(|_| placeholder_client_id.clone());
    println!(
        "cargo:rustc-env=PLACEHOLDER_CLIENT_ID={}",
        placeholder_client_id
    );
    println!("cargo:rustc-env=ADSENSE_CLIENT_ID={}", adsense_client_id);
}
