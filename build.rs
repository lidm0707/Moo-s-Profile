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
    let placeholder_ad_slot =
        std::env::var("PLACEHOLDER_AD_SLOT").unwrap_or_else(|_| "0000000000".to_string());
    let adsense_client_id = std::env::var("ADSENSE_CLIENT_ID")
        .ok()
        .filter(|v| !v.is_empty() && v != &placeholder_client_id)
        .or_else(adsense_client_id_from_index_html)
        .unwrap_or_else(|| placeholder_client_id.clone());
    println!(
        "cargo:rustc-env=PLACEHOLDER_CLIENT_ID={}",
        placeholder_client_id
    );
    println!(
        "cargo:rustc-env=PLACEHOLDER_AD_SLOT={}",
        placeholder_ad_slot
    );
    println!("cargo:rustc-env=ADSENSE_CLIENT_ID={}", adsense_client_id);
}

/// Extract the AdSense publisher id (`ca-pub-XXXXXXXXXXXXXXXX`) from the static
/// `index.html` by finding `adsbygoogle.js?client=ca-pub-` and reading the id
/// that follows. Returns None if the file is missing or the marker isn't found.
fn adsense_client_id_from_index_html() -> Option<String> {
    let html = std::fs::read_to_string("index.html").ok()?;
    parse_adsense_client_id(&html).map(str::to_string)
}

/// Pure parser: extract the AdSense publisher id from an HTML string by finding
/// `adsbygoogle.js?client=ca-pub-` and reading the id that follows. Id is
/// terminated by `"`, `&`, whitespace, or end-of-string. Returns `None` when the
/// marker is absent or the captured id doesn't start with `ca-pub-`.
fn parse_adsense_client_id(html: &str) -> Option<&str> {
    const MARKER: &str = "adsbygoogle.js?client=";
    let start = html.find(MARKER)?;
    let tail = &html[start + MARKER.len()..];
    let end = tail
        .find(|c: char| c == '"' || c == '&' || c.is_whitespace())
        .unwrap_or(tail.len());
    let id = &tail[..end];
    id.starts_with("ca-pub-").then_some(id)
}

#[cfg(test)]
mod tests {
    use super::parse_adsense_client_id;

    #[test]
    fn parses_valid_client_id() {
        let html = "<script src=\"https://pagead2.googlesyndication.com/\
            pagead/js/adsbygoogle.js?client=ca-pub-3526470154848781\" \
            crossorigin=\"anonymous\"></script>";
        assert_eq!(
            parse_adsense_client_id(html),
            Some("ca-pub-3526470154848781")
        );
    }

    #[test]
    fn returns_none_when_marker_absent() {
        let html = "<html><head><title>no adsense here</title></head></html>";
        assert_eq!(parse_adsense_client_id(html), None);
    }

    #[test]
    fn returns_none_when_id_lacks_ca_pub_prefix() {
        let html = "<script src=\"https://pagead2.googlesyndication.com/\
            pagead/js/adsbygoogle.js?client=pub-12345\"></script>";
        assert_eq!(parse_adsense_client_id(html), None);
    }

    #[test]
    fn terminates_at_quote_amper_or_whitespace() {
        // `&` case (query-string continuation)
        let a = "adsbygoogle.js?client=ca-pub-1111111111111111&crossorigin=anonymous";
        assert_eq!(parse_adsense_client_id(a), Some("ca-pub-1111111111111111"));
        // whitespace case
        let b = "adsbygoogle.js?client=ca-pub-2222222222222222 crossorigin";
        assert_eq!(parse_adsense_client_id(b), Some("ca-pub-2222222222222222"));
        // end-of-string case (no terminator)
        let c = "adsbygoogle.js?client=ca-pub-3333333333333333";
        assert_eq!(parse_adsense_client_id(c), Some("ca-pub-3333333333333333"));
    }
}
