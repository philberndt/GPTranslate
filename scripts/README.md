# WinGet Publishing Automation

This directory contains scripts to automate the publishing of GPTranslate to the Microsoft WinGet Community Repository.

## Scripts Overview

### 1. `publish-winget.ps1` - Local Manifest Management

Creates WinGet manifest files locally and copies them to the winget-pkgs repository.

**Usage:**

```powershell
.\scripts\publish-winget.ps1 -Version "1.2.1" -InstallerSha256 "151A6AD9F601E41561CF5FC683FC4A244EE955B64669A5F500921DA95BD323BD"
```

**Parameters:**

- `Version`: The version number (e.g., "1.2.1")
- `InstallerSha256`: SHA256 hash of the installer
- `ReleaseNotes`: Optional release notes
- `ManifestVersion`: Manifest schema version (default: "1.10.0")
- `WingetPkgsPath`: Path to winget-pkgs repository

### 2. `winget-auto-publish.ps1` - Automated Submission

Uses Microsoft's `wingetcreate` tool to automatically submit to WinGet repository.

**Prerequisites:**

```powershell
# Install wingetcreate
winget install Microsoft.WingetCreate
```

**Usage:**

```powershell
# Dry run (test without submission)
.\scripts\winget-auto-publish.ps1 -Version "1.2.1" -DryRun

# Live submission
.\scripts\winget-auto-publish.ps1 -Version "1.2.1" -Token "your_github_token"
```

**Parameters:**

- `Version`: The version number to publish
- `Token`: GitHub personal access token (optional but recommended)
- `DryRun`: Test mode without actual submission

### 3. `release-automation.ps1` - Complete Release Pipeline

**NEW!** Complete end-to-end release automation that handles the entire process from version bumping to WinGet publication.

**Features:**

- ✅ **Version Management**: Updates version in package.json, Cargo.toml, tauri.conf.json
- ✅ **Quality Assurance**: Runs type checking, linting, and formatting
- ✅ **Build Process**: Builds the application and generates installers
- ✅ **WinGet Integration**: Creates and validates WinGet manifests
- ✅ **Git Operations**: Commits changes and creates tags
- ✅ **Testing**: Validates manifests with `winget validate` and `winget install --manifest`
- ✅ **Dry Run Mode**: Test the entire pipeline without making changes
- ✅ **Automated Submission**: Optional direct submission to WinGet repository

**Prerequisites:**

```powershell
# Required tools
npm install                                    # Node.js dependencies
winget install Microsoft.WingetCreate         # WinGet creation tool
git                                           # Version control

# Optional: GitHub token for automated submission
$env:GITHUB_TOKEN = "your_token_here"
```

**Usage:**

```powershell
# Complete release with all steps
.\scripts\release-automation.ps1 -NewVersion "1.2.2" -ReleaseNotes "Bug fixes and improvements" -GitHubToken "your_token"

# Dry run to test the process
.\scripts\release-automation.ps1 -NewVersion "1.2.2" -DryRun

# Skip quality checks (faster for testing)
.\scripts\release-automation.ps1 -NewVersion "1.2.2" -SkipTests

# Skip build (if you already built)
.\scripts\release-automation.ps1 -NewVersion "1.2.2" -SkipBuild

# Custom WinGet repository path
.\scripts\release-automation.ps1 -NewVersion "1.2.2" -WingetPkgsPath "C:\Custom\Path\winget-pkgs\manifests\p\PhilBerndt\GPTranslate"
```

**Parameters:**

- `NewVersion` (required): The new version number (e.g., "1.2.2")
- `ReleaseNotes` (optional): Release notes for the WinGet manifest
- `GitHubToken` (optional): GitHub personal access token for automated submission
- `DryRun` (switch): Test mode without making any changes
- `SkipTests` (switch): Skip quality checks (type checking, linting, formatting)
- `SkipBuild` (switch): Skip the build process
- `WingetPkgsPath` (optional): Custom path to winget-pkgs repository

**What the script does:**

1. **Prerequisites Check**: Validates required tools and clean git state
2. **Version Updates**: Updates version numbers in all project files
3. **Quality Checks**: Runs `npm run check`, `npm run lint`, `npm run lint:fix`, `npm run format:check`
4. **Build Application**: Executes `npm run tauri build`
5. **Installer Analysis**: Calculates SHA256 hashes and gathers installer information
6. **WinGet Manifests**: Generates WinGet manifest files
7. **Manifest Validation**: Runs `winget validate --manifest <path>`
8. **Install Testing**: Tests with `winget install --manifest <path> --dry-run`
9. **Git Operations**: Commits version changes and creates git tags
10. **Automated Submission**: Optionally submits to WinGet repository

**Output Example:**

```text
🚀 GPTranslate Release Automation v1.2.2
================================================

🔧 Validating prerequisites...
✅ npm: 10.1.0
✅ git: 2.41.0
✅ winget: v1.6.2721
✅ wingetcreate: 1.6.0

📝 Updating version numbers to 1.2.2...
✅ Updated version in package.json
✅ Updated version in src-tauri\Cargo.toml
✅ Updated version in src-tauri\tauri.conf.json

🧪 Running quality checks...
  📋 Type checking...
  ✅ Type check passed
  🔍 Linting...
  ✅ Lint check passed

🔨 Building application...
✅ Build completed successfully

📦 Gathering installer information...
✅ NSIS Installer found:
  📁 Path: GPTranslate_1.2.2_x64-setup.exe
  📊 Size: 15432108 bytes
  🔐 SHA256: 151A6AD9F601E41561CF5FC683FC4A244EE955B64669A5F500921DA95BD323BD

📋 Generating WinGet manifests...
✅ WinGet manifests generated successfully

🔍 Validating WinGet manifests...
✅ WinGet manifest validation passed!

🧪 Testing WinGet manifest installation...
✅ WinGet manifest install test passed!

🎉 Release automation completed!
```

### 4. `release.ps1` - One-Click Release Script

**RECOMMENDED!** Simple, streamlined release script that automates everything using GitHub CLI.

**Features:**

- ✅ **Complete Automation**: One command does everything
- ✅ **GitHub CLI Integration**: Uses `gh` for releases and PRs
- ✅ **Smart Manifest Handling**: Copies your existing manifests to winget-pkgs
- ✅ **Automatic PR Creation**: Creates WinGet PR automatically
- ✅ **Built-in Validation**: Validates manifests before submission
- ✅ **Dry Run Support**: Test without making changes

**Prerequisites:**

```powershell
# Required tools
npm install                                    # Node.js dependencies
winget install GitHub.CLI                     # GitHub CLI
git clone https://github.com/microsoft/winget-pkgs.git C:\Users\Phil\Dev\winget-pkgs

# Authenticate GitHub CLI (one time setup)
gh auth login
```

**Usage:**

```powershell
# Complete release (recommended)
.\scripts\release.ps1 -Version "1.2.3"

# With custom release notes
.\scripts\release.ps1 -Version "1.2.3" -ReleaseNotes "Added Ollama support and UI improvements"

# Test run first
.\scripts\release.ps1 -Version "1.2.3" -DryRun
```

**Parameters:**

- `Version` (required): The version number (e.g., "1.2.3")
- `ReleaseNotes` (optional): Custom release notes (default: "Bug fixes and improvements")
- `DryRun` (switch): Test mode without making any changes

**What it does:**

1. **Prerequisites Check**: Validates npm, git, gh CLI, winget are available
2. **Version Updates**: Updates package.json, Cargo.toml, tauri.conf.json
3. **Quality Checks**: Runs `npm run check`, `npm run lint`, `npm run lint:fix`
4. **Build Application**: Executes `npm run tauri build`
5. **Git Operations**: Commits, tags, and pushes to GitHub
6. **GitHub Release**: Creates release with installer using `gh release create`
7. **WinGet Manifests**: Copies manifests to winget-pkgs, updates with SHA256/URL
8. **Manifest Validation**: Runs `winget validate` on manifests
9. **WinGet PR**: Creates PR to microsoft/winget-pkgs automatically

**Output Example:**

```text
🚀 GPTranslate One-Click Release v1.2.3
🔧 Checking prerequisites...
✅ npm
✅ git
✅ gh
✅ winget

📝 Updating version to 1.2.3...
✅ Version files updated

🧪 Running quality checks...
✅ Quality checks passed

🔨 Building application...
✅ Build completed

📦 Installer SHA256: 151A6AD9F601E41561CF5FC683FC4A244EE955B64669A5F500921DA95BD323BD

💾 Creating release commit and tag...
✅ Git operations completed

🎁 Creating GitHub release...
✅ GitHub release created

📋 Preparing WinGet manifests...
✅ Manifest files copied to winget-pkgs
✅ Manifest updated with SHA256 and URL
✅ Manifest validation passed
✅ WinGet PR created successfully

🎉 Release v1.2.3 completed!

🔗 Next steps:
  • Check GitHub release: https://github.com/philberndt/GPTranslate/releases
  • Monitor WinGet PR: https://github.com/microsoft/winget-pkgs/pulls
  • Test installation: winget install PhilBerndt.GPTranslate
```

### 5. GitHub Actions Workflow

Automatic publishing triggered by GitHub releases.

**File:** `.github/workflows/winget-publish.yml`

**Features:**

- Triggers on new releases
- Downloads installer and calculates SHA256
- Creates WinGet manifests
- Submits to WinGet repository via PR

**Setup:**

1. Add these secrets to your GitHub repository:
   - `WINGET_TOKEN`: GitHub personal access token with repo access
   - `WINGET_FORK_USER`: Your GitHub username (optional)

## Quick Start (Recommended)

**For most users, use the simple one-click release script:**

```powershell
# One-time setup
git clone https://github.com/microsoft/winget-pkgs.git C:\Users\Phil\Dev\winget-pkgs
gh auth login

# Release in one command
.\scripts\release.ps1 -Version "1.2.3"
```

That's it! The script handles everything automatically.

## Publishing Process

### Recommended: One-Click Release

```powershell
# Complete release with automatic WinGet submission
.\scripts\release.ps1 -Version "1.2.3" -ReleaseNotes "Your release notes here"
```

This single command:

- Updates all version files
- Runs quality checks and builds
- Creates Git commit/tag and pushes
- Creates GitHub release with installer
- Copies manifests to winget-pkgs repository
- Updates manifests with SHA256 and URLs
- Validates manifests
- Creates PR to microsoft/winget-pkgs automatically

### Alternative: Manual Process (Advanced Users)

1. **Build the application:**

   ```powershell
   npm run tauri build
   ```

2. **Get installer SHA256:**

   ```powershell
   Get-FileHash -Algorithm SHA256 "src-tauri\target\release\bundle\nsis\GPTranslate_1.2.1_x64-setup.exe"
   ```

3. **Create manifests:**

   ```powershell
   .\scripts\publish-winget.ps1 -Version "1.2.1" -InstallerSha256 "YOUR_SHA256_HERE"
   ```

4. **Test with wingetcreate (dry run):**

   ```powershell
   .\scripts\winget-auto-publish.ps1 -Version "1.2.1" -DryRun
   ```

5. **Submit to WinGet:**

   ```powershell
   .\scripts\winget-auto-publish.ps1 -Version "1.2.1" -Token "YOUR_GITHUB_TOKEN"
   ```

### Automated Process (GitHub Actions)

1. **Create a GitHub release** with tag format `v1.2.1`
2. **Upload the installer** as a release asset
3. **GitHub Actions automatically:**
   - Downloads the installer
   - Calculates SHA256
   - Creates WinGet manifests
   - Submits PR to winget-pkgs repository

## Manifest Structure (Schema v1.10.0)

The automation creates three manifest files:

### `PhilBerndt.GPTranslate.yaml` (Version Manifest)

```yaml
PackageIdentifier: PhilBerndt.GPTranslate
PackageVersion: 1.2.1
DefaultLocale: en-US
ManifestType: version
ManifestVersion: 1.10.0
```

### `PhilBerndt.GPTranslate.installer.yaml` (Installer Manifest)

```yaml
PackageIdentifier: PhilBerndt.GPTranslate
PackageVersion: 1.2.1
Platform:
- Windows.Desktop
MinimumOSVersion: 10.0.0.0
InstallerType: nullsoft
Scope: user
InstallModes:
- interactive
- silent
UpgradeBehavior: install
ReleaseDate: 2025-06-13
Installers:
- Architecture: x64
  InstallerUrl: https://github.com/philberndt/GPTranslate/releases/download/1.2.1/GPTranslate_1.2.1_x64-setup.exe
  InstallerSha256: 151A6AD9F601E41561CF5FC683FC4A244EE955B64669A5F500921DA95BD323BD
ManifestType: installer
ManifestVersion: 1.10.0
```

### `PhilBerndt.GPTranslate.locale.en-US.yaml` (Locale Manifest)

Contains package metadata, description, tags, and release notes.

## Best Practices

1. **Always test with dry run first**
2. **Use consistent version numbering** (semantic versioning)
3. **Verify installer URLs** are accessible before submission
4. **Include meaningful release notes** for users
5. **Monitor PR status** at <https://github.com/microsoft/winget-pkgs/pulls>

## Troubleshooting

### Common Issues

1. **wingetcreate not found:**

   ```powershell
   winget install Microsoft.WingetCreate
   ```

2. **Invalid SHA256:**
   - Ensure the installer file exists at the specified URL
   - Recalculate hash after rebuild

3. **Manifest validation errors:**

   ```powershell
   winget validate "path\to\manifest\folder"
   ```

4. **GitHub token issues:**
   - Ensure token has `repo` scope
   - Check token hasn't expired

### Getting Help

- WinGet documentation: <https://docs.microsoft.com/en-us/windows/package-manager/>
- wingetcreate documentation: <https://github.com/microsoft/winget-create>
- Community repository: <https://github.com/microsoft/winget-pkgs>

## Release Checklist

- [ ] Version updated in all files (package.json, Cargo.toml, tauri.conf.json)
- [ ] Application built successfully
- [ ] Installer tested locally
- [ ] SHA256 calculated
- [ ] Manifests created and validated
- [ ] Dry run successful
- [ ] GitHub release created with proper tag
- [ ] PR submitted to winget-pkgs
- [ ] PR reviewed and merged
