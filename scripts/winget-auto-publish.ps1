# Simplified WinGet publishing automation using wingetcreate
# Prerequisites: Install wingetcreate CLI tool from Microsoft

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [Parameter(Mandatory=$false)]
    [string]$Token,
    
    [Parameter(Mandatory=$false)]
    [switch]$DryRun
)

Write-Host "🚀 WinGet Publishing Automation for GPTranslate v$Version" -ForegroundColor Green

# Check if wingetcreate is installed
try {
    $wingetcreateVersion = wingetcreate --version
    Write-Host "✅ Found wingetcreate: $wingetcreateVersion" -ForegroundColor Green
} catch {
    Write-Error "❌ wingetcreate not found. Please install it first:"
    Write-Error "   winget install Microsoft.WingetCreate"
    exit 1
}

# Construct installer URL
$installerUrl = "https://github.com/philberndt/GPTranslate/releases/download/$Version/GPTranslate_${Version}_x64-setup.exe"
Write-Host "📥 Installer URL: $installerUrl" -ForegroundColor Cyan

# Check if this is an update or new package
$packageId = "PhilBerndt.GPTranslate"

if ($DryRun) {
    Write-Host "🔍 DRY RUN MODE - No actual submission will be made" -ForegroundColor Yellow
    
    # Just validate the installer URL
    try {
        $response = Invoke-WebRequest -Uri $installerUrl -Method Head
        Write-Host "✅ Installer URL is accessible (Status: $($response.StatusCode))" -ForegroundColor Green
    } catch {
        Write-Error "❌ Installer URL is not accessible: $installerUrl"
        exit 1
    }
    
    $command = "wingetcreate update $packageId --urls $installerUrl --version $Version"
    if ($Token) {
        $command += " --token $Token"
    }
    
    Write-Host "🔄 Command that would be executed:" -ForegroundColor Yellow
    Write-Host $command -ForegroundColor White
    
} else {
    Write-Host "🔄 Submitting to WinGet repository..." -ForegroundColor Yellow
    
    # Build wingetcreate command
    $cmd = @(
        "wingetcreate", "update", $packageId,
        "--urls", $installerUrl,
        "--version", $Version,
        "--submit"
    )
    
    if ($Token) {
        $cmd += @("--token", $Token)
    }
    
    # Execute wingetcreate
    try {
        Write-Host "Executing: $($cmd -join ' ')" -ForegroundColor Cyan
        & $cmd[0] $cmd[1..($cmd.Length-1)]
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "🎉 Successfully submitted to WinGet repository!" -ForegroundColor Green
        } else {
            Write-Error "❌ wingetcreate failed with exit code: $LASTEXITCODE"
            exit 1
        }
    } catch {
        Write-Error "❌ Error executing wingetcreate: $_"
        exit 1
    }
}

Write-Host ""
Write-Host "📋 Summary:" -ForegroundColor Cyan
Write-Host "  • Package ID: $packageId" -ForegroundColor White
Write-Host "  • Version: $Version" -ForegroundColor White
Write-Host "  • Installer URL: $installerUrl" -ForegroundColor White
Write-Host "  • Mode: $(if ($DryRun) { 'Dry Run' } else { 'Live Submission' })" -ForegroundColor White

if (-not $DryRun) {
    Write-Host ""
    Write-Host "🔗 Monitor the PR status at:" -ForegroundColor Yellow
    Write-Host "   https://github.com/microsoft/winget-pkgs/pulls" -ForegroundColor White
}
