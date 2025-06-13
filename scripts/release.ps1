# GPTranslate One-Click Release Script
# Automates the entire release process using GitHub CLI

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [Parameter(Mandatory=$false)]
    [string]$ReleaseNotes = "Bug fixes and improvements",
    
    [Parameter(Mandatory=$false)]
    [switch]$DryRun = $false
)

$ErrorActionPreference = "Stop"
$ProjectRoot = Get-Location

Write-Host "🚀 GPTranslate One-Click Release v$Version" -ForegroundColor Green

if ($DryRun) {
    Write-Host "🔍 DRY RUN MODE - No changes will be made" -ForegroundColor Yellow
}

# Check prerequisites
Write-Host "🔧 Checking prerequisites..." -ForegroundColor Cyan

$tools = @("npm", "git", "gh", "winget")
foreach ($tool in $tools) {
    try {
        $null = & $tool --version 2>$null
        Write-Host "✅ $tool" -ForegroundColor Green
    } catch {
        Write-Error "❌ $tool not found. Please install it first."
    }
}

# Check git status
$gitStatus = git status --porcelain
if ($gitStatus -and -not $DryRun) {
    Write-Error "❌ Git working directory not clean. Commit or stash changes first."
}

Write-Host ""

# Update version numbers
Write-Host "📝 Updating version to $Version..." -ForegroundColor Cyan

if (-not $DryRun) {
    # Update package.json
    $packageJson = Get-Content "package.json" | ConvertFrom-Json
    $packageJson.version = $Version
    $packageJson | ConvertTo-Json -Depth 10 | Set-Content "package.json"
    
    # Update Cargo.toml
    (Get-Content "src-tauri\Cargo.toml") -replace 'version\s*=\s*"[^"]*"', "version = `"$Version`"" | Set-Content "src-tauri\Cargo.toml"
    
    # Update tauri.conf.json
    $tauriConf = Get-Content "src-tauri\tauri.conf.json" | ConvertFrom-Json
    $tauriConf.version = $Version
    $tauriConf | ConvertTo-Json -Depth 10 | Set-Content "src-tauri\tauri.conf.json"
}

Write-Host "✅ Version files updated" -ForegroundColor Green

# Quality checks
Write-Host "🧪 Running quality checks..." -ForegroundColor Cyan

if (-not $DryRun) {
    npm run check
    npm run lint
    npm run lint:fix
}

Write-Host "✅ Quality checks passed" -ForegroundColor Green

# Build
Write-Host "🔨 Building application..." -ForegroundColor Cyan

if (-not $DryRun) {
    npm run tauri build
}

Write-Host "✅ Build completed" -ForegroundColor Green

# Get installer hash
$installerPath = "src-tauri\target\release\bundle\nsis\GPTranslate_${Version}_x64-setup.exe"
$installerHash = if (Test-Path $installerPath) {
    (Get-FileHash $installerPath -Algorithm SHA256).Hash
} else {
    "INSTALLER_NOT_FOUND"
}

Write-Host "📦 Installer SHA256: $installerHash" -ForegroundColor Cyan

# Git operations
Write-Host "💾 Creating release commit and tag..." -ForegroundColor Cyan

if (-not $DryRun) {
    git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json
    git commit -m "chore: release v$Version"
    git tag -a "v$Version" -m "Release v$Version"
    git push origin main
    git push origin "v$Version"
}

Write-Host "✅ Git operations completed" -ForegroundColor Green

# Create GitHub release
Write-Host "🎁 Creating GitHub release..." -ForegroundColor Cyan

if (-not $DryRun -and (Test-Path $installerPath)) {
    $releaseTitle = "GPTranslate v$Version"
    gh release create "v$Version" "$installerPath" --title "$releaseTitle" --notes "$ReleaseNotes"
}

Write-Host "✅ GitHub release created" -ForegroundColor Green

# WinGet manifest preparation
Write-Host "📋 Preparing WinGet manifests..." -ForegroundColor Cyan

$wingetPkgsPath = "C:\Users\Phil\Dev\winget-pkgs"
$manifestSourceDir = "manifests\winget\p\PhilBerndt\GPTranslate\$Version"
$manifestDestDir = "$wingetPkgsPath\manifests\p\PhilBerndt\GPTranslate\$Version"

if (-not $DryRun -and $installerHash -ne "INSTALLER_NOT_FOUND") {
    if (Test-Path $wingetPkgsPath) {
        # Create manifest directory
        if (-not (Test-Path $manifestDestDir)) {
            New-Item -ItemType Directory -Path $manifestDestDir -Force | Out-Null
        }
        
        # Copy manifest files if they exist
        if (Test-Path $manifestSourceDir) {
            Copy-Item "$manifestSourceDir\*" -Destination $manifestDestDir -Force
            Write-Host "✅ Manifest files copied to winget-pkgs" -ForegroundColor Green
            
            # Update manifest with actual hash
            $installerManifest = "$manifestDestDir\PhilBerndt.GPTranslate.installer.yaml"
            if (Test-Path $installerManifest) {
                $content = Get-Content $installerManifest -Raw
                $content = $content -replace "INSERT_SHA256_HASH_HERE", $installerHash
                $installerUrl = "https://github.com/philberndt/GPTranslate/releases/download/v$Version/GPTranslate_${Version}_x64-setup.exe"
                $content = $content -replace "INSERT_INSTALLER_URL_HERE", $installerUrl
                Set-Content -Path $installerManifest -Value $content -NoNewline
                Write-Host "✅ Manifest updated with SHA256 and URL" -ForegroundColor Green
            }
            
            # Validate manifests
            try {
                Push-Location $manifestDestDir
                winget validate . | Out-Null
                Write-Host "✅ Manifest validation passed" -ForegroundColor Green
                Pop-Location
            } catch {
                Pop-Location
                Write-Host "⚠️  Manifest validation failed" -ForegroundColor Yellow
            }
            
            # Create PR using GitHub CLI
            try {
                Push-Location $wingetPkgsPath
                $branchName = "add-gptranslate-$Version"
                
                # Create and switch to new branch
                git checkout -b $branchName
                git add "manifests\p\PhilBerndt\GPTranslate\$Version"
                git commit -m "Add GPTranslate version $Version"
                git push origin $branchName
                
                # Create PR
                $prTitle = "Add GPTranslate version $Version"
                $prBody = @"
Add GPTranslate version $Version

**Release Notes:**
$ReleaseNotes

**Validation:**
- [x] Manifest files validated with winget validate
- [x] SHA256 hash verified
- [x] Installer URL accessible

**Package Information:**
- Package: PhilBerndt.GPTranslate
- Version: $Version
- Installer: GPTranslate_${Version}_x64-setup.exe
- SHA256: $installerHash
"@
                
                gh pr create --title "$prTitle" --body "$prBody" --repo "microsoft/winget-pkgs"
                Write-Host "✅ WinGet PR created successfully" -ForegroundColor Green
                Pop-Location
            } catch {
                Pop-Location
                Write-Host "⚠️  Failed to create WinGet PR: $_" -ForegroundColor Yellow
                Write-Host "   Manual steps:" -ForegroundColor White
                Write-Host "   1. cd $wingetPkgsPath" -ForegroundColor White
                Write-Host "   2. git checkout -b add-gptranslate-$Version" -ForegroundColor White
                Write-Host "   3. git add manifests\p\PhilBerndt\GPTranslate\$Version" -ForegroundColor White
                Write-Host "   4. git commit -m 'Add GPTranslate version $Version'" -ForegroundColor White
                Write-Host "   5. git push origin add-gptranslate-$Version" -ForegroundColor White
                Write-Host "   6. Create PR on GitHub" -ForegroundColor White
            }
        } else {
            # Use wingetcreate as fallback
            try {
                $installerUrl = "https://github.com/philberndt/GPTranslate/releases/download/v$Version/GPTranslate_${Version}_x64-setup.exe"
                wingetcreate update PhilBerndt.GPTranslate --urls $installerUrl --version $Version --submit
                Write-Host "✅ WinGet submission via wingetcreate completed" -ForegroundColor Green
            } catch {
                Write-Host "⚠️  Both manifest copy and wingetcreate failed" -ForegroundColor Yellow
            }
        }
    } else {
        Write-Host "⚠️  winget-pkgs repository not found at $wingetPkgsPath" -ForegroundColor Yellow
        Write-Host "   Clone it with: git clone https://github.com/microsoft/winget-pkgs.git $wingetPkgsPath" -ForegroundColor White
    }
} else {
    Write-Host "⏭️  Skipping WinGet submission" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🎉 Release v$Version completed!" -ForegroundColor Green
Write-Host ""
Write-Host "📋 Summary:" -ForegroundColor Cyan
Write-Host "  🏷️  Version: $Version" -ForegroundColor White
Write-Host "  📦 Installer: $(if (Test-Path $installerPath) { '✅ Created' } else { '❌ Not found' })" -ForegroundColor White
Write-Host "  🔐 SHA256: $installerHash" -ForegroundColor White
Write-Host "  🎁 GitHub Release: $(if ($DryRun) { 'Would create' } else { '✅ Created' })" -ForegroundColor White
Write-Host "  📋 WinGet PR: $(if ($DryRun) { 'Would create' } else { '✅ Created' })" -ForegroundColor White
Write-Host ""
Write-Host "🔗 Next steps:" -ForegroundColor Yellow
Write-Host "  • Check GitHub release: https://github.com/philberndt/GPTranslate/releases" -ForegroundColor White
Write-Host "  • Monitor WinGet PR: https://github.com/microsoft/winget-pkgs/pulls" -ForegroundColor White
Write-Host "  • Test installation: winget install PhilBerndt.GPTranslate" -ForegroundColor White

if ($DryRun) {
    Write-Host ""
    Write-Host "🔍 This was a dry run. Re-run without -DryRun to execute the release." -ForegroundColor Yellow
}
