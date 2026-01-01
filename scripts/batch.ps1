# ==============================================================================
# Bad Piggies Cryptor CLI - Interactive Wrapper Script
# File: scripts/batch.ps1
#
# This script is designed to be placed in the "scripts/" directory.
# It automatically searches for the binary in common locations.
# ==============================================================================

# Allow script execution for this process
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force | Out-Null

$BinName = "badpiggies-cryptor-cli.exe"

# Define potential paths for the executable
# 1. Current directory
# 2. Cargo build directory (Development)
$PathsToCheck = @(
    ".\$BinName",                  
    "..\target\release\$BinName"   
)

$ToolPath = $null

# Search for the binary
foreach ($path in $PathsToCheck) {
    if (Test-Path $path) {
        $ToolPath = $path
        if ($path -like "*target*") {
            Write-Host "ℹ️  Running in development mode (using target/release binary)" -ForegroundColor Gray
        }
        break
    }
}

if ($null -eq $ToolPath) {
    Write-Host "❌ Error: Could not find '$BinName'." -ForegroundColor Red
    Write-Host "Please ensure the project is built or the binary is placed in the current directory."
    Write-Host "Press any key to exit..."
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    exit
}

Clear-Host
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "   Bad Piggies Cryptor - Interactive Tool" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "Please select an operation mode:"
Write-Host "1) Decrypt (Binary -> XML)"
Write-Host "2) Encrypt (XML -> Binary)"
Write-Host "3) Generate Template (Create Progress.dat.xml)"
Write-Host "==========================================" -ForegroundColor Cyan

$modeChoice = Read-Host "Enter number (1-3)"

switch ($modeChoice) {
    "1" { $cmd = "decrypt" }
    "2" { $cmd = "encrypt" }
    "3" {
        Write-Host "Generating sample file..." -ForegroundColor Yellow
        & $ToolPath generate
        Write-Host "✅ Sample file generated successfully." -ForegroundColor Green
        Start-Sleep -Seconds 2
        exit
    }
    Default {
        Write-Host "Invalid choice. Exiting." -ForegroundColor Red
        exit
    }
}

Write-Host ""
$inputFile = Read-Host "Enter Input File Path (drag & drop allowed)"
# Remove quotes added by PowerShell when dragging files
$inputFile = $inputFile -replace '"',''

if (-not (Test-Path $inputFile)) {
    Write-Host "❌ Error: File '$inputFile' does not exist." -ForegroundColor Red
    exit
}

Write-Host ""
$outputFile = Read-Host "Enter Output File Path (leave empty for auto-naming)"
$outputFile = $outputFile -replace '"',''

Write-Host ""
Write-Host "Select File Type:"
Write-Host "1) Game Save (Progress.dat)"
Write-Host "2) Vehicle Blueprint (.contraption)"
$typeChoice = Read-Host "Enter number (1-2)"

switch ($typeChoice) {
    "1" { $ftype = "progress" }
    "2" { $ftype = "contraption" }
    Default {
        Write-Host "Invalid choice. Exiting." -ForegroundColor Red
        exit
    }
}

# Construct arguments list
$argsList = @($cmd, "-i", $inputFile, $ftype)

if (-not [string]::IsNullOrWhiteSpace($outputFile)) {
    $argsList += "-o"
    $argsList += $outputFile
}

Write-Host ""
Write-Host "Executing..." -ForegroundColor Yellow
Write-Host "------------------------------------------"

# Execute the tool and catch errors
try {
    & $ToolPath $argsList
    if ($LASTEXITCODE -eq 0) {
        Write-Host "------------------------------------------"
        Write-Host "✅ Operation completed successfully!" -ForegroundColor Green
    } else {
        throw "Exit code $LASTEXITCODE"
    }
} catch {
    Write-Host "------------------------------------------"
    Write-Host "❌ Operation failed. Please check the file or logs." -ForegroundColor Red
}

Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")