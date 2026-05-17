#Requires -Version 5.1
[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

$Repo    = "cnaltn/csprotui"
$InstallDir = "$env:LOCALAPPDATA\csprotui"
$BinDir  = "$InstallDir\bin"

function Info  ($msg) { Write-Host "==> $msg" -ForegroundColor Cyan }
function Ok    ($msg) { Write-Host "  " -NoNewline; Write-Host "ok" -ForegroundColor Green -NoNewline; Write-Host " $msg" }
function Warn  ($msg) { Write-Host "  " -NoNewline; Write-Host "!" -ForegroundColor Yellow -NoNewline; Write-Host " $msg" }
function Fail  ($msg) { Write-Host "  " -NoNewline; Write-Host "!!" -ForegroundColor Red -NoNewline; Write-Host " $msg"; exit 1 }

function Banner {
    $accent = "`e[38;2;220;95;60m"
    $rst = "`e[0m"
    $lines = @(
        '██████╗ ███████╗██████╗ ██████╗  ██████╗ ████████╗██╗   ██╗██╗',
        '██╔════╝██╔════╝██╔══██╗██╔══██╗██╔═══██╗╚══██╔══╝██║   ██║██║',
        '██║     ███████╗██████╔╝██████╔╝██║   ██║   ██║   ██║   ██║██║',
        '██║     ╚════██║██╔═══╝ ██╔══██╗██║   ██║   ██║   ██║   ██║██║',
        '╚██████╗███████║██║     ██║  ██║╚██████╔╝   ██║   ╚██████╔╝██║',
        ' ╚═════╝╚══════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝    ╚═╝    ╚═════╝ ╚═╝'
    )
    Write-Host ""
    foreach ($line in $lines) {
        $out = ""
        foreach ($ch in $line.ToCharArray()) {
            if ($ch -eq ' ') {
                $out += " "
            } else {
                $out += "$accent$ch$rst"
            }
        }
        Write-Host $out
    }
    Write-Host ""
    Write-Host "${accent}     CSPROTUI  -  `e[2minstall${rst}"
    Write-Host ""
}

# Detect platform
$Target = "windows-x86_64"
$Archive = "zip"

Banner

Info "Detected platform: $Target"
Write-Host "  " -NoNewline; Write-Host "*" -ForegroundColor Yellow -NoNewline; Write-Host " Platform  Windows (x86-64)  " -NoNewline -ForegroundColor White
Write-Host "->" -ForegroundColor DarkGray -NoNewline; Write-Host "  $Target" -ForegroundColor DarkGray
Write-Host ""

# Fetch latest release
Info "Fetching latest release..."
try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -UseBasicParsing
    $Tag = $release.tag_name
} catch {
    Fail "Could not determine latest release tag"
}
if (-not $Tag) { Fail "Empty tag received from GitHub API" }
Ok "Latest release: $Tag"

# Download
$ArchiveName = "csprotui-$Target.zip"
$DownloadUrl = "https://github.com/$Repo/releases/download/$Tag/$ArchiveName"
$TmpDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString()
New-Item -ItemType Directory -Path $TmpDir | Out-Null

try {
    Info "Downloading $ArchiveName..."
    $zipPath = Join-Path $TmpDir $ArchiveName
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $zipPath -UseBasicParsing
    Ok "Downloaded"

    # Extract
    Info "Extracting..."
    Expand-Archive -Path $zipPath -DestinationPath $TmpDir -Force
    Ok "Extracted"

    # Install scraper deps
    $scraperDir = Join-Path $TmpDir "scraper"
    if ((Test-Path $scraperDir) -and (Get-Command npm -ErrorAction SilentlyContinue)) {
        Info "Installing scraper dependencies..."
        Push-Location $scraperDir
        & npm install 2>&1 | Out-Null
        Pop-Location
        Ok "Scraper ready"
    }

    # Install
    Info "Installing to $InstallDir..."
    if (Test-Path $InstallDir) {
        Remove-Item -Recurse -Force $InstallDir
    }
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    New-Item -ItemType Directory -Path $BinDir -Force | Out-Null

    $srcBinary = Join-Path $TmpDir "csprotui.exe"
    if (-not (Test-Path $srcBinary)) {
        $srcBinary = Join-Path $TmpDir "csprotui"
    }
    if (-not (Test-Path $srcBinary)) {
        Fail "csprotui binary not found in archive"
    }

    Move-Item -Path $srcBinary -Destination $InstallDir -Force
    if (Test-Path $scraperDir) {
        Move-Item -Path $scraperDir -Destination $InstallDir -Force
    }
    Ok "Installed to $InstallDir"

    # Wrapper batch
    $wrapper = Join-Path $BinDir "csprotui.cmd"
    @"
@echo off
set "CSPROTUI_SCRAPER_DIR=$InstallDir\scraper"
"$InstallDir\csprotui.exe" %*
"@ | Set-Content -Path $wrapper -Encoding ASCII
    Ok "Wrapper created at $wrapper"

    # Add to PATH
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$BinDir*") {
        Info "Adding $BinDir to PATH..."
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$BinDir", "User")
        Ok "PATH updated (restart terminal to apply)"
    } else {
        Ok "PATH already contains $BinDir"
    }

    # CSPROTUI_SCRAPER_DIR env var
    [Environment]::SetEnvironmentVariable("CSPROTUI_SCRAPER_DIR", "$InstallDir\scraper", "User")
    Ok "Environment variable set"

    Write-Host ""
    Write-Host "CSPROTUI $Tag installed!" -ForegroundColor Green
    Write-Host "    Run: csprotui"
    Write-Host "    (restart your terminal if PATH was just updated)"
} finally {
    Remove-Item -Recurse -Force $TmpDir -ErrorAction SilentlyContinue
}
