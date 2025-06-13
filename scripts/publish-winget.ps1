# PowerShell script to automate winget manifest publishing
# This script automates the process of creating and publishing winget manifests

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [Parameter(Mandatory=$true)]
    [string]$InstallerSha256,
    
    [Parameter(Mandatory=$false)]
    [string]$ReleaseNotes,
    
    [Parameter(Mandatory=$false)]
    [string]$ManifestVersion = "1.10.0",
    
    [Parameter(Mandatory=$false)]
    [string]$WingetPkgsPath = "c:\Users\Phil\Dev\winget-pkgs\manifests\p\PhilBerndt\GPTranslate"
)

Write-Host "🚀 Automating winget manifest creation for GPTranslate v$Version" -ForegroundColor Green

# Validate inputs
if (-not (Test-Path $WingetPkgsPath)) {
    Write-Error "❌ Winget-pkgs repository not found at: $WingetPkgsPath"
    exit 1
}

# Create version directory
$versionDir = Join-Path $WingetPkgsPath $Version
$sourceDir = "c:\Users\Phil\Dev\GPTranslate\manifests\winget\p\PhilBerndt\GPTranslate\$Version"

if (-not (Test-Path $sourceDir)) {
    Write-Error "❌ Source manifest directory not found at: $sourceDir"
    exit 1
}

Write-Host "📁 Creating version directory: $versionDir" -ForegroundColor Yellow
New-Item -ItemType Directory -Path $versionDir -Force | Out-Null

# Copy manifest files
Write-Host "📋 Copying manifest files..." -ForegroundColor Yellow
Copy-Item "$sourceDir\*" -Destination $versionDir -Force

# Validate manifests exist
$requiredFiles = @(
    "PhilBerndt.GPTranslate.yaml",
    "PhilBerndt.GPTranslate.installer.yaml", 
    "PhilBerndt.GPTranslate.locale.en-US.yaml"
)

foreach ($file in $requiredFiles) {
    $filePath = Join-Path $versionDir $file
    if (-not (Test-Path $filePath)) {
        Write-Error "❌ Required manifest file missing: $file"
        exit 1
    }
    Write-Host "✅ Found: $file" -ForegroundColor Green
}

Write-Host ""
Write-Host "🎉 Manifest creation completed successfully!" -ForegroundColor Green
Write-Host "📍 Location: $versionDir" -ForegroundColor Cyan
Write-Host ""
Write-Host "🔄 Next steps for publishing to winget:" -ForegroundColor Yellow
Write-Host "1. Navigate to winget-pkgs repository: cd $WingetPkgsPath\..\..\.." -ForegroundColor White
Write-Host "2. Create a new branch: git checkout -b gptranslate-$Version" -ForegroundColor White
Write-Host "3. Add the manifest: git add manifests/p/PhilBerndt/GPTranslate/$Version/" -ForegroundColor White
Write-Host "4. Commit changes: git commit -m 'Add GPTranslate version $Version'" -ForegroundColor White
Write-Host "5. Push branch: git push origin gptranslate-$Version" -ForegroundColor White
Write-Host "6. Create PR on GitHub" -ForegroundColor White

# Optionally run winget validate (requires winget CLI)
Write-Host ""
Write-Host "🔍 Validating manifest..." -ForegroundColor Yellow
try {
    $validateResult = winget validate $versionDir 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Manifest validation passed!" -ForegroundColor Green
    } else {
        Write-Host "⚠️  Manifest validation warnings/errors:" -ForegroundColor Red
        Write-Host $validateResult -ForegroundColor Red
    }
} catch {
    Write-Host "⚠️  Could not run winget validate (winget CLI may not be available)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "📊 Manifest summary:" -ForegroundColor Cyan
Write-Host "  • Package: PhilBerndt.GPTranslate" -ForegroundColor White
Write-Host "  • Version: $Version" -ForegroundColor White
Write-Host "  • Schema: $ManifestVersion" -ForegroundColor White
Write-Host "  • SHA256: $InstallerSha256" -ForegroundColor White
