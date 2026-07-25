use dioxus::prelude::*;

#[cfg(feature = "ads")]
const ADSENSE_CLIENT_ID: &str = env!("ADSENSE_CLIENT_ID");

#[cfg(feature = "ads")]
const PLACEHOLDER_CLIENT_ID: &str = env!("PLACEHOLDER_CLIENT_ID");

#[cfg(feature = "ads")]
const DEFAULT_AD_FORMAT: &str = "auto";

#[cfg(feature = "ads")]
const DEFAULT_RESPONSIVE: bool = true;

#[cfg(feature = "ads")]
const DEFAULT_LAZY: bool = true;

#[cfg(feature = "ads")]
const DEFAULT_INS_STYLE: &str = "display:block";

#[cfg(feature = "ads")]
const ADSBYGOOGLE_CLASS: &str = "adsbygoogle";

#[cfg(feature = "ads")]
const ADSENSE_WRAP_CLASS: &str = "adsense-wrap";

#[cfg(feature = "ads")]
const ADSENSE_WRAP_DARK_SUFFIX: &str = " adsense-wrap-dark";

#[cfg(feature = "ads")]
const ADSENSE_WRAP_LIGHT_SUFFIX: &str = " adsense-wrap-light";

/// Sentinel that marks an unset AdSense slot id. Public so callers can compare
/// against it without hardcoding the value. Always available (no feature gate)
/// so call sites don't need to gate their imports. Override the default via the
/// PLACEHOLDER_AD_SLOT env var (see build.rs).
pub const PLACEHOLDER_AD_SLOT: &str = env!("PLACEHOLDER_AD_SLOT");

/// How far outside the viewport an ad may be before it loads (IntersectionObserver
/// rootMargin). Prefetching slightly improves perceived load without early requests.
#[cfg(feature = "ads")]
const LAZY_ROOT_MARGIN: &str = "200px";

#[cfg(feature = "ads")]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
    export function push_adsense_now() {
        try {
            (window.adsbygoogle = window.adsbygoogle || []).push({});
        } catch (e) {
            console.warn('[adsense] push failed:', e);
        }
    }
    export function lazy_push_adsense(el, rootMargin) {
        try {
            if (!('IntersectionObserver' in window)) { push_adsense_now(); return; }
            const obs = new IntersectionObserver((entries, observer) => {
                for (const entry of entries) {
                    if (entry.isIntersecting) {
                        observer.disconnect();
                        push_adsense_now();
                        break;
                    }
                }
            }, { rootMargin: rootMargin || '200px' });
            obs.observe(el);
        } catch (err) {
            console.warn('[adsense] lazy init failed:', err);
            push_adsense_now();
        }
    }
")]
extern "C" {
    fn push_adsense_now();
    fn lazy_push_adsense(el: &web_sys::Element, root_margin: &str);
}

/// Props shared between the active and disabled render paths.
#[derive(Clone, PartialEq)]
struct AdsenseArgs {
    ad_slot: String,
    ad_format: Option<String>,
    responsive: Option<bool>,
    lazy: Option<bool>,
    dark_mode: Option<bool>,
    style: Option<String>,
    class: Option<String>,
}

#[cfg(feature = "ads")]
fn render_adsense(args: AdsenseArgs) -> Element {
    if ADSENSE_CLIENT_ID == PLACEHOLDER_CLIENT_ID {
        web_sys::console::warn_1(
            &"[adsense] ADSENSE_CLIENT_ID is unset; ad slot not rendered.".into(),
        );
        return rsx! {};
    }
    if args.ad_slot == PLACEHOLDER_AD_SLOT || args.ad_slot.is_empty() {
        web_sys::console::warn_1(
            &"[adsense] ad_slot is unset; pass a real AdSense slot id.".into(),
        );
        return rsx! {};
    }

    let AdsenseArgs {
        ad_slot,
        ad_format,
        responsive,
        lazy,
        dark_mode,
        style,
        class,
    } = args;

    let responsive = responsive.unwrap_or(DEFAULT_RESPONSIVE);
    let lazy = lazy.unwrap_or(DEFAULT_LAZY);
    let ad_format = ad_format.unwrap_or_else(|| DEFAULT_AD_FORMAT.to_string());
    let wrap_class = match dark_mode {
        Some(true) => ADSENSE_WRAP_DARK_SUFFIX,
        Some(false) => ADSENSE_WRAP_LIGHT_SUFFIX,
        None => "",
    };
    let ins_class = format!("{ADSBYGOOGLE_CLASS} {}", class.unwrap_or_default());
    let ins_style = style.unwrap_or_else(|| DEFAULT_INS_STYLE.to_string());

    rsx! {
        div { class: "{ADSENSE_WRAP_CLASS}{wrap_class}",
            ins {
                class: ins_class,
                style: ins_style,
                "data-ad-client": ADSENSE_CLIENT_ID,
                "data-ad-slot": ad_slot,
                "data-ad-format": ad_format,
                "data-full-width-responsive": if responsive { "true" } else { "false" },
                onmounted: move |evt| {
                    // MountedData::downcast yields the web_sys::Element on the web target.
                    if let Some(el) = evt.downcast::<web_sys::Element>() {
                        if lazy {
                            lazy_push_adsense(el, LAZY_ROOT_MARGIN);
                        } else {
                            push_adsense_now();
                        }
                    } else {
                        push_adsense_now();
                    }
                },
            }
        }
    }
}

#[cfg(not(feature = "ads"))]
fn render_adsense(_args: AdsenseArgs) -> Element {
    rsx! {}
}

/// Reusable Google AdSense ad slot.
///
/// - Renders `<ins class="adsbygoogle">` and initializes it when it scrolls into
///   view (IntersectionObserver; falls back to immediate init if unavailable or
///   the element can't be resolved). AdSense queues pushes, so late script loads
///   are handled automatically.
/// - `ad_format` sets `data-ad-format` (defaults to `"auto"`; also `"horizontal"`,
///   `"rectangle"`, `"vertical"`).
/// - `lazy` toggles viewport-deferred init (default `true`).
/// - `responsive` toggles `data-full-width-responsive` (default `true`).
/// - `dark_mode` opts into a theme-aware wrapper class so the slot blends with the
///   site theme.
///
/// Renders nothing when the `ads` cargo feature is off or the publisher ID is
/// still the placeholder.
#[component]
pub fn Adsense(
    ad_slot: String,
    ad_format: Option<String>,
    responsive: Option<bool>,
    lazy: Option<bool>,
    dark_mode: Option<bool>,
    style: Option<String>,
    class: Option<String>,
) -> Element {
    render_adsense(AdsenseArgs {
        ad_slot,
        ad_format,
        responsive,
        lazy,
        dark_mode,
        style,
        class,
    })
}
