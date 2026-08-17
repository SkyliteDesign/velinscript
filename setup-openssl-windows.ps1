# Windows OpenSSL Fix Script
# This script sets up OpenSSL for Windows development

Write-Host "🚀 Setting up OpenSSL for VelinScript development..." -ForegroundColor Green

# Clear any existing OpenSSL environment variables
Remove-Item Env:OPENSSL_DIR -ErrorAction SilentlyContinue
Remove-Item Env:OPENSSL_LIB_DIR -ErrorAction SilentlyContinue
Remove-Item Env:OPENSSL_INCLUDE_DIR -ErrorAction SilentlyContinue
Remove-Item Env:OPENSSL_STATIC -ErrorAction SilentlyContinue
Remove-Item Env:OPENSSL_VENDORED -ErrorAction SilentlyContinue

# Solution 1: Try using vendored OpenSSL (recommended for development)
Write-Host "📦 Using vendored OpenSSL (automatic compilation)..." -ForegroundColor Yellow
$env:OPENSSL_STATIC = "1"
$env:OPENSSL_VENDORED = "1"

Write-Host "✅ OpenSSL environment configured!" -ForegroundColor Green
Write-Host ""
Write-Host "🔧 You can now run:" -ForegroundColor Cyan
Write-Host "   cargo test ai_semantic_test" -ForegroundColor White
Write-Host ""
Write-Host "💡 The vendored OpenSSL will be compiled automatically on first run." -ForegroundColor Gray
Write-Host "   This may take a few minutes but ensures compatibility." -ForegroundColor Gray

# Keep the environment variables for this session
[Environment]::SetEnvironmentVariable("OPENSSL_STATIC", "1", "Process")
[Environment]::SetEnvironmentVariable("OPENSSL_VENDORED", "1", "Process")