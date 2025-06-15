# GPTranslate Winget Manifest Generator and Validator
# This script helps prepare your winget manifest for submission

param(
    [string]$InstallerPath = "",
    [switch]$Validate = $false,
    [switch]$Help = $false
)

if ($Help) {
    Write-Host "GPTranslate Winget Manifest Generator" -ForegroundColor Green
    Write-Host ""
    Write-Host "Usage:"
    Write-Host "  .\prepare-winget.ps1 -InstallerPath 'path\to\installer.msi' [-Validate]"
    Write-Host ""
    Write-Host "Parameters:"
    Write-Host "  -InstallerPath    Path to the MSI installer file"
    Write-Host "  -Validate         Validate the manifest files"
    Write-Host "  -Help             Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\prepare-winget.ps1 -InstallerPath '.\target\release\bundle\msi\GPTranslate_1.1.0_x64_en-US.msi'"
    Write-Host "  .\prepare-winget.ps1 -Validate"
    exit 0
}

$ManifestDir = ".\manifests\p\PhilBerndt\GPTranslate\1.1.0"

function Get-FileHash-SHA256 {
    param([string]$FilePath)
    if (Test-Path $FilePath) {
        return (Get-FileHash -Path $FilePath -Algorithm SHA256).Hash
    }
    return $null
}

function Update-InstallerManifest {
    param([string]$InstallerPath)
    
    if (-not (Test-Path $InstallerPath)) {
        Write-Error "Installer file not found: $InstallerPath"
        return $false
    }
    
    $hash = Get-FileHash-SHA256 -FilePath $InstallerPath
    $fileSize = (Get-Item $InstallerPath).Length
    $extension = [System.IO.Path]::GetExtension($InstallerPath).ToLower()
    
    Write-Host "Installer Information:" -ForegroundColor Green
    Write-Host "  File: $InstallerPath"
    Write-Host "  Size: $fileSize bytes"
    Write-Host "  SHA256: $hash"
    Write-Host "  Type: $(if ($extension -eq '.exe') { 'NSIS/Nullsoft' } elseif ($extension -eq '.msi') { 'MSI' } else { 'Unknown' })"
    
    # Update the installer manifest with the actual hash
    $installerManifest = "$ManifestDir\PhilBerndt.GPTranslate.installer.yaml"
    if (Test-Path $installerManifest) {
        $content = Get-Content $installerManifest -Raw
        $content = $content -replace "INSERT_SHA256_HASH_HERE", $hash
        Set-Content -Path $installerManifest -Value $content -NoNewline
        Write-Host "Updated installer manifest with SHA256 hash" -ForegroundColor Green
    }
    
    return $true
}

function Validate-Manifest {
    Write-Host "Validating winget manifest..." -ForegroundColor Yellow
    
    # Check if winget is available
    try {
        $wingetVersion = winget --version
        Write-Host "Using winget version: $wingetVersion" -ForegroundColor Green
    }
    catch {
        Write-Error "winget is not available. Please install the App Installer from Microsoft Store."
        return $false
    }
    
    # Validate the manifest
    try {
        Push-Location $ManifestDir
        winget validate .
        Write-Host "Manifest validation completed!" -ForegroundColor Green
        return $true
    }
    catch {
        Write-Error "Manifest validation failed: $_"
        return $false
    }
    finally {
        Pop-Location
    }
}

function Show-NextSteps {
    Write-Host ""
    Write-Host "Next Steps for Winget Submission:" -ForegroundColor Cyan
    Write-Host "================================="
    Write-Host ""
    Write-Host "1. Build and Release Your Application:" -ForegroundColor Yellow
    Write-Host "   - Build your Tauri application: npm run tauri build"
    Write-Host "   - Create a GitHub release with your MSI files"
    Write-Host "   - Update the InstallerUrl in the installer manifest with actual download URLs"
    Write-Host ""
    Write-Host "2. Update Installer Manifest:" -ForegroundColor Yellow
    Write-Host "   - Run this script with -InstallerPath to get SHA256 hashes"
    Write-Host "   - Update the ProductCode GUIDs (get from MSI properties)"
    Write-Host "   - Update InstallerUrls to point to your GitHub release assets"
    Write-Host ""
    Write-Host "3. Fork Microsoft's winget-pkgs Repository:" -ForegroundColor Yellow
    Write-Host "   - Fork: https://github.com/microsoft/winget-pkgs"
    Write-Host "   - Copy your manifest folder to: manifests/p/PhilBerndt/GPTranslate/1.1.0/"
    Write-Host ""
    Write-Host "4. Submit Pull Request:" -ForegroundColor Yellow
    Write-Host "   - Create a branch: git checkout -b add-gptranslate-1.1.0"
    Write-Host "   - Commit your changes: git add . && git commit -m 'Add GPTranslate 1.1.0'"
    Write-Host "   - Push and create PR to microsoft/winget-pkgs"
    Write-Host ""
    Write-Host "5. Validation Tools:" -ForegroundColor Yellow
    Write-Host "   - Use: winget validate . (in manifest directory)"
    Write-Host "   - Test install: winget install --manifest ."
    Write-Host ""
}

# Main execution
Write-Host "GPTranslate Winget Manifest Preparation" -ForegroundColor Green
Write-Host "=======================================" -ForegroundColor Green

if ($InstallerPath) {
    if (Update-InstallerManifest -InstallerPath $InstallerPath) {
        Write-Host "Installer manifest updated successfully!" -ForegroundColor Green
    }
}

if ($Validate) {
    Validate-Manifest
}

if (-not $InstallerPath -and -not $Validate) {
    Show-NextSteps
}

Write-Host ""
Write-Host "Manifest files created in: $ManifestDir" -ForegroundColor Green
