# Moo's Profile

A professional profile page built with Rust and Dioxus framework, showcasing interests, skills, and work history.

## Features

- 🌟 **Professional Profile**: Display of personal information, interests, and skills
- 💼 **Work History Timeline**: Interactive timeline showing career progression
- 🌓 **Dark/Light Mode**: Toggle between themes for comfortable viewing
- 📱 **Responsive Design**: Optimized for both desktop and mobile devices
- 🦀 **Rust & Dioxus**: Built entirely with Rust and the Dioxus UI framework

## Tech Stack

- **Frontend**: [Dioxus](https://dioxuslabs.com/) 0.7
- **Styling**: Custom CSS with responsive design
- **Deployment**: Vercel for production hosting
- **CI/CD**: GitHub Actions for automated builds

## Getting Started

### Prerequisites

- Rust (latest stable version)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/installation/)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/lidm0707/my_profile.git
   cd my_profile
   ```

2. Install Dioxus CLI:
   ```bash
   curl -sSL http://dioxus.dev/install.sh | sh
   ```

3. Run the development server:
   ```bash
   dx serve
   ```

4. Open your browser and navigate to `http://localhost:8080`

### Building for Production

To build the project for production:

```bash
dx build --release
```

The built files will be in the `dist` directory, ready for deployment.

## Environment Variables

This project reads configuration from a `.env` file at build time (via `build.rs` + `dotenvy`).
Copy the documented template to get started:

```bash
cp .env.example .env
```

Required: `APP_MODE`, `SUPABASE_URL`, `SUPABASE_ANON_KEY`. Optional: AdSense vars
(see below). `.env` is gitignored — never commit real secrets. Full reference in
[`.env.example`](.env.example).

## How to Use Google AdSense

AdSense is integrated behind a Cargo feature flag so it is **off by default**
(no real ad requests during `dx serve`). For the complete reference — setup,
troubleshooting, and deployment notes — see [`docs/adsense.md`](docs/adsense.md).

**Quick start:**

1. Get your publisher id (`ca-pub-XXXXXXXXXXXXXXXX`) and an ad slot id from the
   [AdSense console](https://www.google.com/adsense/), and approve your domain.
2. Put the publisher id in the **static `index.html`** `<script>` tag (this is
   what Google's verification crawler reads — the app is client-rendered, so it
   must be in the static HTML). `build.rs` parses it from there automatically,
   so **no `.env` entry is needed** for ads (set `ADSENSE_CLIENT_ID` in `.env`
   only to override).
3. Run/build **with the `ads` feature** to render ad slots:
   ```bash
   dx serve  --features ads          # preview
   dx bundle --release --features ads --out-dir ./dist   # production → dist/
   ```
4. Drop an ad slot anywhere with the reusable `Adsense` component:
   ```rust
   use crate::components::Adsense;

   rsx! {
       Adsense {
           ad_slot: "<YOUR_AD_SLOT_ID>".to_string(),   // from AdSense console → Ads → By ad unit
           // optional (all have sensible defaults):
           // ad_format: Some("auto".to_string()),
           // responsive: Some(true),
           // lazy: Some(true),
           // dark_mode: Some(dark_mode()),
           // style: Some("display:block".to_string()),
           // class: Some("my-ad-wrap".to_string()),
       }
   }
   ```

Notes:

- Vercel deploys the committed `dist/` (no build command in `vercel.json`),
  so rebuild with `dx bundle --release [--features ads] --out-dir ./dist` and
  commit `dist/` to deploy changes.
- The AdSense library loads from the static `index.html` script tag in every
  build (required for site verification). Ad **slots** require the `ads` feature
  and a non-placeholder `ADSENSE_CLIENT_ID`; without them nothing renders.
- Google only serves real ads on approved domains; on `localhost` you may see
  blank spaces or PSAs — that's expected.
- A **consent banner** (Google CMP / Funding Choices) with 2 choices
  (Consent / Manage options) is loaded from the static `index.html`, before
  the AdSense library. Create the GDPR message once in the AdSense console
  (Privacy & messaging) — see `docs/adsense.md` §4c.
- An **`ads.txt`** (AdSense seller verification) lives at the repo root and is
  copied to `dist/public/ads.txt` by the build flow (see `dist/build.md`).
  `vercel.json` has a route exception so `/ads.txt` is served instead of the
  SPA shell. See `docs/adsense.md` §4d.

See [`docs/adsense.md`](docs/adsense.md) for the full reference (verification,
consent banner/CMP, troubleshooting, lazy-load, theme support).

## Project Structure

```
my_profile/
├─ assets/           # Static assets like CSS and images
├─ docs/             # Feature docs (e.g. adsense.md)
├─ src/
│  ├─ components/    # UI components (incl. Adsense ad slot)
│  ├─ features/      # Feature modules (content, etc.)
│  ├─ hooks/         # Custom hooks
│  ├─ routes/        # Route definitions and pages
│  └─ main.rs        # App entry, global head (styles, scripts)
├─ .env.example      # Documented template for required env vars
├─ .github/
│  └─ workflows/
│     └─ ci.yml      # GitHub Actions workflow for CI/CD
├─ Cargo.toml        # Rust project configuration (incl. `ads` feature)
├─ Dioxus.toml       # Dioxus-specific configuration
└─ vercel.json       # Vercel deployment configuration
```

## Deployment

### Vercel

The project is configured for deployment on Vercel. Simply connect your repository to Vercel and it will automatically build and deploy:

1. Push your code to the main branch
2. Connect your repository to Vercel
3. Vercel will detect the framework and build settings automatically

### Manual Deployment

For manual deployment:

1. Build the project:
   ```bash
   dx build --release
   ```

2. Deploy the `dist` folder to your hosting provider of choice

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Connect With Me

[![GitHub](https://img.shields.io/badge/GitHub-lidm0707-black?style=flat-square&logo=github)](https://github.com/lidm0707)  
[![LinkedIn](https://img.shields.io/badge/LinkedIn-kachon--wanglavan-blue?style=flat-square&logo=linkedin)](https://www.linkedin.com/in/kachon-wanglavan-4124a5216/)# Moo-s-Profile
