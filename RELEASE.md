# GPTranslate Release Process

This document outlines the complete release process for GPTranslate, including version updates, building, testing, and distribution.

## 📋 Pre-Release Checklist

### 1. Code Quality & Testing
- [ ] Run all linting and formatting checks
- [ ] Fix all Rust clippy warnings: `cargo clippy -- -D warnings`
- [ ] Format Rust code: `cargo fmt`
- [ ] Format TypeScript/Svelte code: `npm run format`
- [ ] Run ESLint: `npm run lint`
- [ ] Test all translation services (OpenAI, Azure, Ollama)
- [ ] Verify UI functionality across different themes
- [ ] Test global hotkeys and system tray integration

### 2. Version Updates
Update version numbers in the following files:
- [ ] `package.json` - Update `version` field
- [ ] `src-tauri/Cargo.toml` - Update `version` field
- [ ] `src-tauri/tauri.conf.json` - Update `version` field

### 3. Build and Validation
- [ ] Clean build: `npm run build`
- [ ] Rust compilation check: `cd src-tauri && cargo build --release`
- [ ] Integration tests with actual translation services

## 🚀 Release Process

### Step 1: Prepare Release
```powershell
# Update versions (done manually in files above)
# Run quality checks
npm run lint
npm run format
cd src-tauri && cargo fmt && cargo clippy -- -D warnings
```

### Step 2: Build Application
```powershell
# Build the application with installers
npm run tauri build
```

This creates:
- `src-tauri/target/release/bundle/nsis/GPTranslate_X.X.X_x64-setup.exe` (NSIS installer)

### Step 3: Create Git Tag and Commit
```powershell
# Commit all changes
git add .
git commit -m "Release vX.X.X: [Brief description of changes]"

# Create and push tag
git tag X.X.X
git push origin main
git push origin X.X.X
```

### Step 4: Create GitHub Release
```powershell
# Create GitHub release with installers
gh release create X.X.X \
  --title "GPTranslate vX.X.X - [Release Title]" \
  --notes "[Release notes - see template below]" \
  "src-tauri/target/release/bundle/nsis/GPTranslate_X.X.X_x64-setup.exe"
```

### Step 5: Create Winget Manifests
1. **Get installer SHA256 hash:**
   ```powershell
   Get-FileHash "src-tauri/target/release/bundle/nsis/GPTranslate_X.X.X_x64-setup.exe" -Algorithm SHA256
   ```

2. **Create manifest directory:**
   ```powershell
   mkdir "manifests/p/PhilBerndt/GPTranslate/X.X.X"
   ```

3. **Create manifest files** (see existing manifests as templates):
   - `PhilBerndt.GPTranslate.yaml`
   - `PhilBerndt.GPTranslate.installer.yaml`
   - `PhilBerndt.GPTranslate.locale.en-US.yaml`

4. **Validate manifests:**
   ```powershell
   cd "manifests/p/PhilBerndt/GPTranslate/X.X.X"
   winget validate .
   ```

### Step 6: Submit to Winget
```powershell
# Submit to Microsoft's winget-pkgs repository
# This process typically involves creating a Pull Request to:
# https://github.com/microsoft/winget-pkgs

# Alternative: Use wingetcreate tool
wingetcreate update PhilBerndt.GPTranslate -u https://github.com/philberndt/GPTranslate/releases/download/X.X.X/GPTranslate_X.X.X_x64-setup.exe -v X.X.X
```

## 📝 Release Notes Template

```markdown
## 🚀 New Features & Enhancements
[List major new features and improvements]

## 🔧 Bug Fixes
[List bug fixes and resolved issues]

## 🏗️ Development
[List technical improvements, dependency updates, etc.]

## 📦 Installation
- **Windows Installer (NSIS)**: GPTranslate_X.X.X_x64-setup.exe

[Brief summary of what this release brings to users]
```

## 🔧 Troubleshooting

### Common Build Issues
- **Cargo build failures**: Ensure all dependencies are up to date
- **Frontend build issues**: Clear node_modules and reinstall: `rm -rf node_modules && npm install`
- **Tauri build issues**: Update Tauri CLI: `npm install -g @tauri-apps/cli@latest`

### Release Issues
- **GitHub release upload failures**: Check file paths and ensure files exist
- **Winget validation failures**: Verify YAML syntax and required fields
- **Hash mismatches**: Regenerate SHA256 hash after any file changes

## 📦 Distribution Channels

### Primary
- **GitHub Releases**: Main distribution channel with installers
- **Winget**: Microsoft's package manager for Windows

### Future Considerations
- **Microsoft Store**: Native Windows store distribution

## 🔒 Security Considerations

- All release artifacts are signed (when code signing certificate is available)
- SHA256 hashes are provided for integrity verification
- GitHub releases are the authoritative source for downloads
- Winget packages are validated by Microsoft before inclusion

## 📊 Post-Release Tasks

- [ ] Verify GitHub release is live and downloadable
- [ ] Monitor winget submission status
- [ ] Update documentation if needed
- [ ] Announce release on relevant channels
- [ ] Monitor for user feedback and issues

## 🔄 Automation Opportunities

Future improvements to consider:
- GitHub Actions for automated builds on tag push
- Automated winget manifest generation
- Automated release notes generation from commit messages
- Integration tests in CI/CD pipeline

---

**Last Updated**: 22/06/2025
**Next Review**: After implementing CI/CD automation
