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

## Project Structure

```
my_profile/
├─ assets/           # Static assets like CSS and images
├─ src/
│  └─ main.rs        # Main application code with routing and components
├─ .github/
│  └─ workflows/
│     └─ ci.yml      # GitHub Actions workflow for CI/CD
├─ Cargo.toml        # Rust project configuration
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
