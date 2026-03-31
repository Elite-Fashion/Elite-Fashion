# Interactive Product Showcase Website

A modern, interactive, and scrollable product showcase built with Rust Leptos.

## Features

- 🚀 **Interactive Hero Section** with animations
- 📱 **Responsive Design** works on all devices
- 🎨 **Modern UI** with gradients and glassmorphism
- 🖼️ **Image Gallery** with lightbox functionality
- ⭐ **Testimonials** with interactive carousel
- 📊 **Feature Cards** with hover effects
- 🎯 **Smooth Scrolling** navigation

## Setup

1. Install Rust and Trunk:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install trunk
```

2. Install required targets:
```bash
rustup target add wasm32-unknown-unknown
```

3. Run the development server:
```bash
trunk serve
```

4. Open http://localhost:8080 in your browser.

## Build for Production

```bash
trunk build --release
```

The built files will be in the `dist/` directory.

## Project Structure

```
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk configuration
├── index.html          # Main HTML file
├── frontend/
│   ├── src/
│   │   ├── main.rs     # Application entry point
│   │   ├── app.rs      # Main app component
│   │   ├── styles/     # CSS styles
│   │   └── components/ # Leptos components
│   └── static/         # Static assets
└── README.md
```

## Technologies Used

- **Rust Leptos** - Reactive web framework
- **Trunk** - Web assembly build tool
- **CSS3** - Modern animations and effects
- **WebAssembly** - High-performance web apps
