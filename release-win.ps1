$version = '1.2.0'
$exec = 'winnowdemo'
$pkg = $exec + '_' + $version + '_win64'

# Build release
cargo build --release
# Create release target
New-Item -ItemType Directory -Force -Path "$pkg"
# Copy executable
Copy-Item "target\release\$exec.exe" -Destination "$pkg"
# Copy test data
Copy-Item input.txt -Destination "$pkg"
# Compress
Compress-Archive -Path "$pkg" -DestinationPath "$pkg.zip"