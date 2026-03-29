#Requires -Version 5.1
$ErrorActionPreference = 'Stop'

$Root = Resolve-Path (Join-Path $PSScriptRoot '..')
$VersionFile = Join-Path $Root 'third_party/filament_prebuilt/VERSION'
$Version = (Get-Content -LiteralPath $VersionFile -Raw).Trim()

$Archive = "filament-$Version-windows.tgz"
$Url = "https://github.com/google/filament/releases/download/$Version/$Archive"
$Dest = Join-Path $Root 'third_party/filament_prebuilt/windows-x86_64'
$Tmp = Join-Path ([System.IO.Path]::GetTempPath()) $Archive

Write-Host "Downloading $Url"
Invoke-WebRequest -Uri $Url -OutFile $Tmp

Get-ChildItem -LiteralPath $Dest -Force | Where-Object { $_.Name -ne '.gitkeep' } | Remove-Item -Recurse -Force

New-Item -ItemType Directory -Path $Dest -Force | Out-Null
tar -xf $Tmp -C $Dest

$LibDir = Join-Path $Dest 'lib/x86_64/md'
Write-Host "Extracted to $Dest"
Write-Host "Suggested FILAMENT_LIB_DIR=$LibDir"
Write-Host "(Debug /MDd builds may use lib/x86_64/mdd instead.)"
