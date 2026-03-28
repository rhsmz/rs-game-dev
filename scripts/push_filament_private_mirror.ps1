#Requires -Version 5.1
# Filament サブモジュールの現在のコミットをプライベートミラーへ初回プッシュする。
# .gitmodules を private URL に変える「前」に 1 回実行してもよい（origin は google のまま）。
# .gitmodules 更新後は、他環境で clone する前に必ず本スクリプト相当でミラーに中身があることを確認すること。
$ErrorActionPreference = 'Stop'

$PrivateUrl = 'git@github.com:rhsmz/filament-rs-game-dev.git'
$RepoSlug = 'rhsmz/filament-rs-game-dev'

$FilamentDir = (Resolve-Path (Join-Path $PSScriptRoot '..\third_party\filament')).Path
Set-Location $FilamentDir

Write-Host "Pushing $(git rev-parse --short HEAD) to ${PrivateUrl} (branch private) ..."
git push $PrivateUrl HEAD:refs/heads/private

if (git rev-parse -q --verify refs/tags/v1.70.1 2>$null) {
    Write-Host "Pushing tag v1.70.1 ..."
    git push $PrivateUrl refs/tags/v1.70.1
}

Write-Host "Setting default branch to private ..."
gh repo edit $RepoSlug --default-branch private

Write-Host "Done. You may commit .gitmodules (private URL) and push the parent repo."
