$version = '1.4.1'
$exec = 'winnowdemo'
$pkg = $exec + '_' + $version + '_win64'

# Build release
cargo build --release
# Create release target
New-Item -ItemType Directory -Force -Path "$pkg"
# Copy executable
Copy-Item "target\release\$exec.exe" -Destination "$pkg"
# Copy usage
Copy-Item README.md -Destination "$pkg"
# Copy test data
Copy-Item input.txt -Destination "$pkg"
Copy-Item AdReadiness.txt -Destination "$pkg"
# Copy AdReadiness launcher
Copy-Item RunAdReadiness.bat -Destination "$pkg"
# Compress
Compress-Archive -Path "$pkg" -DestinationPath "$pkg.zip"