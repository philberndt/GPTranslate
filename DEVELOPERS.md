# GPTranslate Developer Documentation

This document provides comprehensive guidance for developers working on GPTranslate, a fast and lightweight desktop translation application built with Rust (Tauri) and Svelte.

## 📋 Table of Contents

1. [Project Overview](#project-overview)
2. [Development Environment Setup](#development-environment-setup)
3. [Architecture Overview](#architecture-overview)
4. [Frontend Development](#frontend-development)
5. [Backend Development](#backend-development)
6. [Translation Providers](#translation-providers)
7. [Configuration System](#configuration-system)
8. [Testing Strategy](#testing-strategy)
9. [Build & Release Process](#build--release-process)
10. [Common Development Tasks](#common-development-tasks)
11. [Troubleshooting](#troubleshooting)

## 🎯 Project Overview

GPTranslate is a desktop translation application that provides:

- **Real-time translation** with multiple AI providers
- **Global hotkey support** for system-wide translation
- **System tray integration** for background operation
- **Modern UI** built with Svelte 5 and TailwindCSS
- **Cross-platform compatibility** via Tauri framework

### Technology Stack

- **Frontend**: Svelte 5, SvelteKit, TypeScript, TailwindCSS, DaisyUI
- **Backend**: Rust, Tauri 2.x
- **Build System**: Vite, Cargo
- **Styling**: TailwindCSS with DaisyUI components
- **Icons**: Heroicons

## 🛠️ Development Environment Setup

### Prerequisites

1. **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
2. **Node.js** (18+) - [Install Node.js](https://nodejs.org/)
3. **Git** - For version control

### Setup Instructions

```bash
# Clone the repository
git clone https://github.com/philberndt/GPTranslate.git
cd GPTranslate

# Install frontend dependencies
npm install

# Install Tauri CLI (if not already installed)
npm install -g @tauri-apps/cli@latest

# Start development server
npm run tauri dev
```

### Development Commands

#### Frontend Development

- `npm run dev` - Start Vite development server with hot reload
- `npm run build` - Build frontend for production
- `npm run preview` - Preview production build
- `npm run check` - Run Svelte type checking
- `npm run check:watch` - Run type checking in watch mode

#### Code Quality

- `npm run lint` - Run ESLint on all files
- `npm run lint:fix` - Fix auto-fixable ESLint issues
- `npm run format` - Format code with Prettier
- `npm run format:check` - Check if code is properly formatted

#### Rust Backend

- `cd src-tauri && cargo build` - Build Rust backend
- `cd src-tauri && cargo build --release` - Release build
- `cd src-tauri && cargo fmt` - Format Rust code
- `cd src-tauri && cargo clippy -- -D warnings` - Run Clippy with warnings as errors

#### Tauri Application

- `npm run tauri dev` - Start development with Tauri app
- `npm run tauri build` - Build production application with installer

## 🏗️ Architecture Overview

### Application Structure

```
GPTranslate/
├── src/                          # Frontend (Svelte)
│   ├── lib/                      # Reusable components
│   │   ├── components/           # UI components
│   │   ├── utils/               # Utility functions
│   │   └── languages.ts         # Language definitions
│   ├── routes/                   # SvelteKit routes
│   └── app.html                  # Main HTML template
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs               # Main library entry
│   │   ├── config.rs            # Configuration management
│   │   ├── history.rs           # Translation history
│   │   ├── translation.rs       # Core translation logic
│   │   ├── provider_factory.rs  # Translation provider factory
│   │   ├── trans_*.rs           # Translation provider implementations
│   │   ├── theme.rs             # Theme detection
│   │   └── tray.rs              # System tray functionality
│   ├── Cargo.toml               # Rust dependencies
│   └── tauri.conf.json          # Tauri configuration
├── package.json                  # Frontend dependencies
└── CLAUDE.md                     # Claude Code instructions
```

### Core Components

#### Frontend Components (Svelte)

- **+page.svelte** - Main translation interface
- **Settings.svelte** - Application configuration
- **History.svelte** - Translation history management
- **ModelSelector.svelte** - AI model selection
- **CompactLanguageDropdown.svelte** - Language selection
- **AlternativeTranslations.svelte** - Alternative translation display

#### Backend Modules (Rust)

- **lib.rs** - Application setup, event handlers, Tauri commands
- **translation.rs** - Translation service abstraction and smart prompting
- **config.rs** - User settings and model management
- **history.rs** - Translation history management
- **provider_factory.rs** - Creates translation service instances
- **trans\_\*.rs** - Individual translation provider implementations

## 🎨 Frontend Development

### Svelte 5 Patterns

GPTranslate uses Svelte 5 with the new runes syntax:

```typescript
// State management with runes
let translatedText = $state("");
let isTranslating = $state(false);

// Derived state
let hasConfiguredProvider = $derived(isAnyProviderConfigured(config));

// Effects
$effect(() => {
  if (originalText.trim()) {
    performTranslation();
  }
});
```

### Component Development Guidelines

1. **State Management**: Use Svelte 5 runes (`$state`, `$derived`, `$effect`)
2. **Styling**: TailwindCSS classes with DaisyUI components
3. **Icons**: Heroicons for consistent iconography
4. **Reactivity**: Leverage Svelte's reactive patterns for UI updates

### Adding New Components

```svelte
<script lang="ts">
  import { invoke } from "../lib/tauri"
  import { CheckIcon } from "heroicons-svelte/24/outline"

  let componentState = $state("initial")

  async function handleAction() {
    try {
      await invoke("backend_command", { param: "value" })
      componentState = "success"
    } catch (error) {
      console.error("Action failed:", error)
    }
  }
</script>

<div class="component-container">
  <button class="btn btn-primary" onclick={handleAction}>
    <CheckIcon class="w-4 h-4" />
    Action Button
  </button>
</div>
```

## 🦀 Backend Development

### Tauri Commands

Backend functions are exposed via `#[tauri::command]`:

```rust
#[tauri::command]
async fn translate_text(
    text: String,
    target_language: String,
    state: State<'_, AppState>,
) -> Result<TranslationResult, String> {
    let service = state.translation_service.lock().await;
    service.translate(&text, &target_language)
        .await
        .map_err(|e| e.to_string())
}
```

### State Management

Application state uses `Arc<Mutex<>>` for thread safety:

```rust
pub struct AppState {
    pub config: Arc<Mutex<Config>>,
    pub translation_service: Arc<Mutex<TranslationService>>,
}
```

### Error Handling

- Use `anyhow::Result` for general error handling
- Custom `Error` enum for translation-specific errors
- Frontend receives string error messages

## 🔌 Translation Providers

### Provider Interface

All translation providers implement the `TranslationProvider` trait:

```rust
#[async_trait]
pub trait TranslationProvider: Send + Sync {
    async fn translate(
        &self,
        text: &str,
        target_language: &str,
        detected_language: Option<&str>,
        custom_prompt: Option<&str>,
    ) -> Result<TranslationResult>;

    async fn get_alternative_translations(
        &self,
        text: &str,
        target_language: &str,
        detected_language: Option<&str>,
    ) -> Result<AlternativeTranslationsResult>;
}
```

### Adding New Providers

1. **Create provider module**: `src-tauri/src/trans_newprovider.rs`
2. **Implement trait**: Implement `TranslationProvider` for your provider
3. **Update factory**: Add provider creation logic in `provider_factory.rs`
4. **Update config**: Add provider-specific settings to `Config` struct
5. **Update UI**: Add configuration options in frontend settings

### Example Provider Implementation

```rust
use async_trait::async_trait;
use crate::translation::{TranslationProvider, TranslationResult};

pub struct NewProvider {
    api_key: String,
}

#[async_trait]
impl TranslationProvider for NewProvider {
    async fn translate(
        &self,
        text: &str,
        target_language: &str,
        detected_language: Option<&str>,
        custom_prompt: Option<&str>,
    ) -> Result<TranslationResult> {
        // Implementation here
    }
}
```

## ⚙️ Configuration System

### Configuration Structure

The `Config` struct in `config.rs` manages all application settings:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub api_provider: String,
    pub target_language: String,
    pub alternative_target_language: String,
    pub favorite_languages: Vec<String>,
    pub auto_start: bool,
    pub hotkey: String,
    pub theme: String,
    // ... provider-specific settings
}
```

### Configuration Storage

- **Location**: `~/.config/gptranslate/gptranslate.json`
- **Format**: JSON with pretty printing
- **Migration**: Automatic migration for backward compatibility

### Adding Configuration Options

1. **Add field to Config struct**
2. **Update Default implementation**
3. **Add migration logic in load() method**
4. **Update frontend settings UI**
5. **Use configuration in relevant code**

## 🧪 Testing Strategy

### Manual Testing

- Test translation services through UI
- Use `test_translation_from_clipboard` function
- Integration tests with actual API providers

### Automated Testing

```bash
# Run Rust tests
cd src-tauri && cargo test

# Run frontend checks
npm run check
npm run lint
```

### Testing Translation Providers

Create test configurations for each provider and verify:

- Basic translation functionality
- Error handling
- Alternative translations
- Language detection

## 🚀 Build & Release Process

### Development Build

```bash
npm run tauri dev
```

### Production Build

```bash
npm run tauri build
```

This creates:

- `src-tauri/target/release/bundle/nsis/GPTranslate_X.X.X_x64-setup.exe` (Windows installer)

### Release Checklist

1. **Code Quality**
   - Run all linting: `npm run lint`
   - Format code: `npm run format`
   - Rust checks: `cargo fmt && cargo clippy -- -D warnings`

2. **Version Updates**
   - Update `package.json` version
   - Update `src-tauri/Cargo.toml` version
   - Update `src-tauri/tauri.conf.json` version

3. **Build & Test**
   - Clean build: `npm run build`
   - Rust compilation: `cargo build --release`
   - Integration tests with translation services

4. **Release**
   - Create git tag
   - Build production installer
   - Create GitHub release
   - Submit to Winget

## 🔧 Common Development Tasks

### Adding a New Language

1. Update `src/lib/languages.ts`
2. Add language code to language arrays
3. Test with translation providers

### Modifying Translation Logic

Core logic in `src-tauri/src/translation.rs`:

- Smart prompting in `create_smart_prompt()`
- Provider abstraction in `TranslationService`

### UI Component Development

1. Create new component in `src/lib/`
2. Import and use in parent components
3. Follow established patterns for state management
4. Use TailwindCSS and DaisyUI for styling

### Global Hotkey Configuration

Hotkey handling in `src-tauri/src/lib.rs`:

- Parse hotkey combinations
- Register/unregister global shortcuts
- Handle hotkey events

## 🐛 Troubleshooting

### Common Build Issues

**Cargo build failures**

- Ensure all dependencies are up to date
- Check Rust toolchain version

**Frontend build issues**

- Clear node_modules: `rm -rf node_modules && npm install`
- Check Node.js version compatibility

**Tauri build issues**

- Update Tauri CLI: `npm install -g @tauri-apps/cli@latest`
- Check system dependencies

### Development Issues

**Translation provider errors**

- Verify API keys and endpoints
- Check network connectivity
- Review provider-specific documentation

**State synchronization issues**

- Check Tauri command implementations
- Verify state management patterns
- Use browser dev tools for frontend debugging

### Runtime Issues

**Global hotkey not working**

- Check hotkey registration in system
- Verify hotkey format (e.g., "Ctrl+Q")
- Test with different hotkey combinations

**Theme issues**

- Check system theme detection
- Verify CSS custom properties
- Test theme switching logic

## 📚 Additional Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Svelte 5 Documentation](https://svelte.dev/docs/svelte/overview)
- [TailwindCSS Documentation](https://tailwindcss.com/docs)
- [DaisyUI Components](https://daisyui.com/components/)
- [Rust Documentation](https://doc.rust-lang.org/)

## 🤝 Contributing Guidelines

1. **Fork the repository** and create a feature branch
2. **Follow code style** guidelines (use formatters)
3. **Test thoroughly** before submitting PR
4. **Update documentation** for new features
5. **Write clear commit messages**

### Code Style

- **Rust**: Use `cargo fmt` and `cargo clippy`
- **TypeScript/Svelte**: Use `npm run format` and `npm run lint`
- **Commits**: Use conventional commit format

### Pull Request Process

1. Ensure all tests pass
2. Update documentation as needed
3. Request review from maintainers
4. Address feedback promptly

---

**Happy coding!** 🚀

For questions or support, please open an issue on GitHub or contact the maintainers.
