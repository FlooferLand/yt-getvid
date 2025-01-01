cargo build --release
sudo Copy-Item "target/release/yt-getvid.exe" "$Env:WinDir/System32/yt-getvid.exe"
Write-Output "Installed to '$Env:WinDir/System32/yt-getvid.exe'!"
