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

### 3. GitHub Actions Workflow

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

## Publishing Process

### Manual Process (Recommended for first-time setup)

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
