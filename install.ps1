$ErrorActionPreference = "Stop"

$repo = "Haiko333/burnr"
$api = "https://api.github.com/repos/$repo/releases/latest"

Write-Host "Installing Burnr..."

$release = Invoke-RestMethod -Uri $api
$version = $release.tag_name
Write-Host "Latest version: $version"

$asset = $release.assets | Where-Object { $_.name -like "*.msi" } | Select-Object -First 1

if (-not $asset) {
    $asset = $release.assets | Where-Object { $_.name -like "*setup*.exe" -or $_.name -like "*_x64-setup.exe" } | Select-Object -First 1
}

if (-not $asset) {
    Write-Host "Error: No Windows installer found in release."
    exit 1
}

$url = $asset.browser_download_url
$installer = "$env:TEMP\burnr-installer$([System.IO.Path]::GetExtension($asset.name))"

Write-Host "Downloading $($asset.name)..."
Invoke-WebRequest -Uri $url -OutFile $installer

Write-Host "Running installer..."
if ($installer -like "*.msi") {
    Start-Process msiexec.exe -ArgumentList "/i `"$installer`" /quiet" -Wait
} else {
    Start-Process $installer -ArgumentList "/S" -Wait
}

Remove-Item $installer -Force
Write-Host "Done! Burnr is installed."
