# GPTranslate 2.0.0

![GPTranslate Logo](https://img.shields.io/badge/GPTranslate-2.0.0-blue?style=for-the-badge&logo=translate&logoColor=white)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](https://choosealicense.com/licenses/mit/)
[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Svelte](https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00)](https://svelte.dev/)

A **fast**, **lightweight**, and **AI-powered** desktop translation application built with Rust and Tauri. GPTranslate provides instant translation between multiple languages with global hotkey support, system tray integration, and a beautiful, responsive user interface.

## 🚀 Key Features

### 🔑 **Global Hotkey Support**

- Configure custom keyboard shortcuts (e.g., `Ctrl+Q`) for instant clipboard translation
- Works system-wide, even when the app is minimized or hidden
- Automatically captures clipboard content and translates it

### 🎯 **Multiple AI Translation Providers**

- **OpenAI GPT Models** - High-quality translations with context awareness
- **Azure OpenAI** - Enterprise-grade translation with custom deployments
- **Azure Translator** - Microsoft's dedicated translation service
- **Ollama** - Local AI models for privacy-focused translation

### 🧠 **Smart Translation Logic**

- Automatic language detection with intelligent switching
- Alternative target language selection when source equals target
- Context-aware prompting for better translation quality
- Alternative translation suggestions for selected text

### 📱 **Modern User Interface**

- **Clean, responsive design** built with Svelte 5 and TailwindCSS
- **Theme support** - Auto, Light, and Dark themes with system detection
- **System tray integration** - Minimize to tray for background operation
- **Real-time translation** with configurable auto-translation settings

### 📊 **Translation History & Management**

- Persistent translation history with smart deduplication
- Export and manage translation entries
- Clear or delete individual translations
- Favorite language management for quick access

### ⚙️ **Advanced Configuration**

- **Model management** - Configure multiple models per provider
- **Custom prompts** - Customize translation behavior
- **Auto-start** - Launch with system startup
- **Fallback providers** - Configure alternative translation sources
- **Debounce settings** - Control auto-translation timing

## 🛠️ Installation

### Windows

#### Option 1: Download from GitHub Releases

1. Visit the [Releases page](https://github.com/philberndt/GPTranslate/releases)
2. Download `GPTranslate_2.0.0_x64-setup.exe`
3. Run the installer and follow the setup wizard

#### Option 2: Using Winget (Windows Package Manager)

```powershell
winget install PhilBerndt.GPTranslate
```

## 🔧 Quick Setup

1. **Launch GPTranslate** after installation
2. **Configure your preferred translation provider**:
   - OpenAI: Add your API key
   - Azure OpenAI: Configure endpoint, API key, and deployment
   - Azure Translator: Add API key and region
   - Ollama: Set up local Ollama server URL
3. **Select your target languages** (primary and alternative)
4. **Configure global hotkey** (optional)
5. **Start translating!**

## 💡 How to Use

### Basic Translation

1. **Type or paste text** in the input field
2. **Select source and target languages** (or use auto-detection)
3. **Translation appears instantly** with auto-translation enabled

### Global Hotkey Translation

1. **Copy text to clipboard** from any application
2. **Press your configured hotkey** (e.g., `Ctrl+Q`)
3. **GPTranslate activates** and translates clipboard content automatically

### Alternative Translations

1. **Select translated text** in the output area
2. **Click "Get Alternatives"** to see different translation options
3. **Choose the best translation** for your context

## 🆕 What's New in Version 2.0.0

### 🎉 Major Enhancements

- **Unified Configuration Directory**: All settings and history now stored in `~/.config/gptranslate`
- **Enhanced Translation Services**: Improved AI provider integration with better error handling
- **Smart Language Detection**: More accurate automatic language detection and switching
- **Alternative Translation Support**: Generate multiple translation options for better accuracy
- **Improved UI Consistency**: Refined interface with better spacing and theme consistency

### 🔧 Technical Improvements

- **Updated Dependencies**: Latest Tauri 2.x, Svelte 5, and modern web stack
- **Better Performance**: Optimized translation processing and UI responsiveness
- **Enhanced Model Management**: Better configuration and management of AI models
- **Improved Error Handling**: More robust error messages and fallback mechanisms

### 🐛 Bug Fixes

- Fixed language switching logic when detected language equals target
- Improved clipboard handling and global hotkey reliability
- Better theme detection and application across the interface
- Resolved issues with translation history deduplication

## 🏗️ Technical Architecture

### Frontend (Svelte 5 + SvelteKit)

- **Modern reactive framework** with Svelte 5 runes
- **TailwindCSS + DaisyUI** for responsive, beautiful UI components
- **TypeScript** for type safety and better development experience

### Backend (Rust + Tauri)

- **High-performance Rust backend** for system integration
- **Tauri framework** for secure, lightweight desktop application
- **Cross-platform compatibility** with native performance

### Key Technologies

- **Language Support**: 100+ languages with comprehensive language detection
- **System Integration**: Global hotkeys, system tray, auto-start
- **Security**: Secure API key storage and encrypted configuration
- **Performance**: Fast translation with intelligent caching

## 🔒 Privacy & Security

- **Local-first approach**: All configuration and history stored locally
- **Secure credential storage**: API keys encrypted and stored securely
- **No telemetry**: No usage data collected or transmitted
- **Ollama support**: Option for completely offline translation with local models

## 🤝 Contributing

We welcome contributions! Please see our [Developer Documentation](DEVELOPERS.md) for setup instructions and development guidelines.

### Quick Development Setup

```bash
# Clone the repository
git clone https://github.com/philberndt/GPTranslate.git
cd GPTranslate

# Install dependencies
npm install

# Start development server
npm run tauri dev
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) for the desktop framework
- UI powered by [Svelte](https://svelte.dev/) and [TailwindCSS](https://tailwindcss.com/)
- Icons from [Heroicons](https://heroicons.com/)
- Language data and utilities for comprehensive language support

## 📞 Support & Contact

- **Website**: [gptranslate.berndt.no](https://gptranslate.berndt.no)
- **Issues**: [GitHub Issues](https://github.com/philberndt/GPTranslate/issues)
- **Developer**: Phil Berndt ([phil@berndt.no](mailto:phil@berndt.no))

---

**Made with ❤️ by Phil Berndt** | **Powered by Rust & Svelte**
