# VelinScript Windows Development Setup
# This script sets up the development environment for Windows users

Write-Host "🚀 VelinScript Windows Development Setup" -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Green

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")
if (-not $isAdmin) {
    Write-Host "⚠️  This script should be run as Administrator for best results" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🔍 Checking development environment..." -ForegroundColor Cyan

# Check if WSL2 is available
try {
    $wslStatus = wsl --status 2>$null
    $wslAvailable = $LASTEXITCODE -eq 0
} catch {
    $wslAvailable = $false
}

if ($wslAvailable) {
    Write-Host "✅ WSL2 is available" -ForegroundColor Green
    
    # Check if Ubuntu is installed
    try {
        $ubuntuCheck = wsl -l -q | Select-String -Pattern "Ubuntu" -SimpleMatch
        if ($ubuntuCheck) {
            Write-Host "✅ Ubuntu is installed in WSL2" -ForegroundColor Green
            
            Write-Host ""
            Write-Host "🎯 RECOMMENDED SOLUTION: Use WSL2 for development" -ForegroundColor Green
            Write-Host ""
            Write-Host "📋 Next steps:" -ForegroundColor Yellow
            Write-Host "   1. Open WSL2 terminal: wsl -d Ubuntu" -ForegroundColor White
            Write-Host "   2. Navigate to project: cd /mnt/d/velinscript" -ForegroundColor White
            Write-Host "   3. Install OpenSSL: sudo apt update && sudo apt install libssl-dev pkg-config" -ForegroundColor White
            Write-Host "   4. Run tests: cargo test ai_semantic_test" -ForegroundColor White
            Write-Host ""
            Write-Host "💡 This is the most reliable solution for Windows developers!" -ForegroundColor Gray
            
            # Create a helper script
            $helperScript = @"
# VelinScript WSL2 Helper
# Run this in WSL2 Ubuntu terminal

echo "🚀 Setting up VelinScript in WSL2..."

# Update package lists
sudo apt update

# Install OpenSSL development libraries
sudo apt install -y libssl-dev pkg-config

# Navigate to project (adjust path as needed)
cd /mnt/d/velinscript

echo "✅ Setup complete! You can now run:"
echo "   cargo test ai_semantic_test"
echo "   cargo test --all-features"
"@
            
            $helperScript | Out-File -FilePath "wsl2-setup.sh" -Encoding UTF8
            Write-Host "📝 Created wsl2-setup.sh - run this in WSL2 Ubuntu" -ForegroundColor Green
            
        } else {
            Write-Host "⚠️  Ubuntu is not installed in WSL2" -ForegroundColor Yellow
            Write-Host "📥 Installing Ubuntu..." -ForegroundColor Cyan
            wsl --install -d Ubuntu
            Write-Host "🔄 Please restart your computer and run this script again" -ForegroundColor Red
        }
    } catch {
        Write-Host "❌ Error checking WSL2 distributions" -ForegroundColor Red
    }
} else {
    Write-Host "⚠️  WSL2 is not available" -ForegroundColor Yellow
    Write-Host "📥 Installing WSL2..." -ForegroundColor Cyan
    
    try {
        wsl --install
        Write-Host "✅ WSL2 installation started" -ForegroundColor Green
        Write-Host "🔄 Please restart your computer and run this script again" -ForegroundColor Red
    } catch {
        Write-Host "❌ Failed to install WSL2" -ForegroundColor Red
        Write-Host "💡 Please install WSL2 manually from Microsoft Store" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "🎯 Alternative Solutions:" -ForegroundColor Cyan
Write-Host "========================" -ForegroundColor Cyan

Write-Host ""
Write-Host "🔧 Option 1: Docker Development" -ForegroundColor Yellow
Write-Host "   Create a Dockerfile with Ubuntu base and OpenSSL" -ForegroundColor White
Write-Host "   See OPENSSL_WINDOWS_COMPLETE_SOLUTION.md for details" -ForegroundColor Gray

Write-Host ""
Write-Host "🔧 Option 2: Manual OpenSSL Installation" -ForegroundColor Yellow
Write-Host "   Install OpenSSL for Windows manually" -ForegroundColor White
Write-Host "   See OPENSSL_WINDOWS_FIX.md for detailed instructions" -ForegroundColor Gray

Write-Host ""
Write-Host "🎉 Setup recommendations created!" -ForegroundColor Green
Write-Host "📖 Check the documentation files for more details:" -ForegroundColor Cyan
Write-Host "   - OPENSSL_WINDOWS_COMPLETE_SOLUTION.md" -ForegroundColor White
Write-Host "   - OPENSSL_WINDOWS_FIX.md" -ForegroundColor White
Write-Host "   - wsl2-setup.sh (for WSL2 setup)" -ForegroundColor White

Write-Host ""
Write-Host "💡 Pro Tip: WSL2 is the most reliable solution for Rust development on Windows!" -ForegroundColor Green