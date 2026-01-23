# GitHub Wiki Generator für VelinScript (PowerShell)
# Konvertiert docs/ in GitHub Wiki Format

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$DocsDir = Join-Path $ProjectRoot "docs"
$WikiDir = Join-Path $ProjectRoot ".wiki"

Write-Host "GitHub Wiki Generator fuer VelinScript" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# Wiki-Verzeichnis erstellen
if (-not (Test-Path $WikiDir)) {
    New-Item -ItemType Directory -Path $WikiDir | Out-Null
}

# Funktion: Konvertiere Markdown für GitHub Wiki
function Convert-ToWiki {
    param(
        [string]$InputFile,
        [string]$OutputFile,
        [string]$RelativePath
    )
    
    # Erstelle Ausgabe-Verzeichnis
    $OutputDir = Split-Path -Parent $OutputFile
    if (-not (Test-Path $OutputDir)) {
        New-Item -ItemType Directory -Path $OutputDir | Out-Null
    }
    
    # Lese Datei
    $Content = Get-Content $InputFile -Raw -Encoding UTF8
    
    # Konvertiere Links
    $Content = $Content -replace '\[([^\]]+)\]\(([^)]+\.md)\)', '[${1}](${2})'
    $Content = $Content -replace '\[([^\]]+)\]\(guides/([^)]+\.md)\)', '[${1}](Guide-${2})'
    $Content = $Content -replace '\[([^\]]+)\]\(architecture/([^)]+\.md)\)', '[${1}](Architecture-${2})'
    $Content = $Content -replace '\[([^\]]+)\]\(tools/([^)]+\.md)\)', '[${1}](Tools-${2})'
    $Content = $Content -replace '\[([^\]]+)\]\(api/([^)]+\.md)\)', '[${1}](API-${2})'
    $Content = $Content -replace '\[([^\]]+)\]\(language/([^)]+\.md)\)', '[${1}](Language-${2})'
    $Content = $Content -replace '\.md\)', ')'
    $Content = $Content -replace '\(([^)]+)\.md\)', '(${1})'
    
    # Entferne .md aus Link-Namen
    $Content = $Content -replace '\[([^\]]+)\]\(([^)]+)\.md\)', '[${1}](${2})'
    
    # Füge Navigation am Ende hinzu (außer für Home)
    if ($RelativePath -ne "Home.md") {
        $Content += "`n`n---`n`n**← [Zurück zur Übersicht](Home)**"
    }
    
    # Schreibe Datei
    Set-Content -Path $OutputFile -Value $Content -NoNewline -Encoding UTF8
}

# Funktion: Erstelle Wiki-Seite
function Create-WikiPage {
    param(
        [string]$SourceFile,
        [string]$WikiName
    )
    
    $OutputFile = Join-Path $WikiDir "$WikiName.md"
    
    if (Test-Path $SourceFile) {
        Write-Host "  OK $WikiName" -ForegroundColor Green
        Convert-ToWiki -InputFile $SourceFile -OutputFile $OutputFile -RelativePath "$WikiName.md"
    } else {
        Write-Host "  WARN $WikiName (Datei nicht gefunden: $SourceFile)" -ForegroundColor Yellow
    }
}

Write-Host "Erstelle Wiki-Seiten..." -ForegroundColor Cyan
Write-Host ""

# Home-Seite
Write-Host "Home.md" -ForegroundColor Yellow
Create-WikiPage -SourceFile (Join-Path $DocsDir "README.md") -WikiName "Home"

# Getting Started
Write-Host ""
Write-Host "Getting Started..." -ForegroundColor Yellow
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\getting-started.md") -WikiName "Getting-Started"
Create-WikiPage -SourceFile (Join-Path $ProjectRoot "QUICK_START.md") -WikiName "Quick-Start"

# Language
Write-Host ""
Write-Host "Language..." -ForegroundColor Yellow
Create-WikiPage -SourceFile (Join-Path $DocsDir "language\specification.md") -WikiName "Language-Specification"
Create-WikiPage -SourceFile (Join-Path $DocsDir "language\basics.md") -WikiName "Language-Basics"

# Architecture
Write-Host ""
Write-Host "Architecture..." -ForegroundColor Yellow
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\compiler-architecture.md") -WikiName "Architecture-Compiler"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\pass-verlauf.md") -WikiName "Architecture-Pass-Verlauf"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\type-inference.md") -WikiName "Architecture-Type-Inference"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\code-ordering-pass.md") -WikiName "Architecture-Code-Ordering"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\ir-representation.md") -WikiName "Architecture-IR"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\borrow-checker.md") -WikiName "Architecture-Borrow-Checker"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\code-generation.md") -WikiName "Architecture-Code-Generation"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\multi-target-compilation.md") -WikiName "Architecture-Multi-Target"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\module-resolution.md") -WikiName "Architecture-Module-Resolution"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\framework-integration.md") -WikiName "Architecture-Framework-Integration"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\parallelization.md") -WikiName "Architecture-Parallelization"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\ai-compiler-passes.md") -WikiName "Architecture-AI-Passes"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\prompt-optimizer.md") -WikiName "Architecture-Prompt-Optimizer"
Create-WikiPage -SourceFile (Join-Path $DocsDir "architecture\system-generation.md") -WikiName "Architecture-System-Generation"

# Guides
Write-Host ""
Write-Host "Guides..." -ForegroundColor Yellow
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-1-basics.md") -WikiName "Guide-Basics"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-2-apis.md") -WikiName "Guide-APIs"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-3-security.md") -WikiName "Guide-Security"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-4-database.md") -WikiName "Guide-Database"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-5-validation.md") -WikiName "Guide-Validation"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-6-authentication.md") -WikiName "Guide-Authentication"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-7-ml.md") -WikiName "Guide-ML-LLM"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-8-intelligence.md") -WikiName "Guide-Intelligence"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-type-inference.md") -WikiName "Guide-Type-Inference"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-ml-training.md") -WikiName "Guide-ML-Training"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-pattern-matching.md") -WikiName "Guide-Pattern-Matching"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-closures.md") -WikiName "Guide-Closures"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-collections.md") -WikiName "Guide-Collections"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-http-client.md") -WikiName "Guide-HTTP-Client"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-string-interpolation.md") -WikiName "Guide-String-Interpolation"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\tutorial-debugger.md") -WikiName "Guide-Debugger"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\vektor-datenbanken.md") -WikiName "Guide-Vektor-Datenbanken"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\cli-reference.md") -WikiName "Guide-CLI-Reference"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\api-keys-setup.md") -WikiName "Guide-API-Keys"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\advanced.md") -WikiName "Guide-Advanced"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\backend.md") -WikiName "Guide-Backend"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\security.md") -WikiName "Guide-Security-Best-Practices"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\ai-ml.md") -WikiName "Guide-AI-ML"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\auto-imports.md") -WikiName "Guide-Auto-Imports"
Create-WikiPage -SourceFile (Join-Path $DocsDir "guides\plugin-development.md") -WikiName "Guide-Plugin-Development"

# Tools
Write-Host ""
Write-Host "Tools..." -ForegroundColor Yellow
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\TOOLS_ÜBERSICHT.md") -WikiName "Tools-Overview"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\vscode-extension.md") -WikiName "Tools-VS-Code-Extension"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\auto-repair.md") -WikiName "Tools-Auto-Repair"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\security-scanner.md") -WikiName "Tools-Security-Scanner"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\linter.md") -WikiName "Tools-Linter"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\formatter.md") -WikiName "Tools-Formatter"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\debugger.md") -WikiName "Tools-Debugger"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\hot-reload.md") -WikiName "Tools-Hot-Reload"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\package-manager.md") -WikiName "Tools-Package-Manager"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\api-doc-generator.md") -WikiName "Tools-API-Doc-Generator"
Create-WikiPage -SourceFile (Join-Path $DocsDir "tools\library-generator.md") -WikiName "Tools-Library-Generator"

# API
Write-Host ""
Write-Host "API..." -ForegroundColor Yellow
Create-WikiPage -SourceFile (Join-Path $DocsDir "api\standard-library.md") -WikiName "API-Standard-Library"
Create-WikiPage -SourceFile (Join-Path $DocsDir "api\decorators.md") -WikiName "API-Decorators"
Create-WikiPage -SourceFile (Join-Path $DocsDir "api\frameworks.md") -WikiName "API-Frameworks"
Create-WikiPage -SourceFile (Join-Path $DocsDir "api\openapi.md") -WikiName "API-OpenAPI"

# Erstelle _Sidebar.md
Write-Host ""
Write-Host "Erstelle _Sidebar.md..." -ForegroundColor Yellow
$SidebarContent = @"
# Navigation

## Schnellstart
- [Home](Home)
- [Getting Started](Getting-Started)
- [Quick Start](Quick-Start)

## Sprache
- [Language Specification](Language-Specification)
- [Language Basics](Language-Basics)

## Architektur
- [Compiler Architecture](Architecture-Compiler)
- [Pass-Verlauf](Architecture-Pass-Verlauf)
- [Type Inference](Architecture-Type-Inference)
- [Code Ordering](Architecture-Code-Ordering)
- [IR Representation](Architecture-IR)
- [Borrow Checker](Architecture-Borrow-Checker)
- [Code Generation](Architecture-Code-Generation)
- [Multi-Target Compilation](Architecture-Multi-Target)
- [Module Resolution](Architecture-Module-Resolution)
- [Framework Integration](Architecture-Framework-Integration)
- [Parallelization](Architecture-Parallelization)
- [AI Compiler Passes](Architecture-AI-Passes)
- [Prompt Optimizer](Architecture-Prompt-Optimizer)
- [System Generation](Architecture-System-Generation)

## Guides
- [Basics](Guide-Basics)
- [APIs](Guide-APIs)
- [Security](Guide-Security)
- [Database](Guide-Database)
- [Validation](Guide-Validation)
- [Authentication](Guide-Authentication)
- [ML/LLM](Guide-ML-LLM)
- [Intelligence Features](Guide-Intelligence)
- [Type Inference](Guide-Type-Inference)
- [ML Training](Guide-ML-Training)
- [Pattern Matching](Guide-Pattern-Matching)
- [Closures](Guide-Closures)
- [Collections](Guide-Collections)
- [HTTP Client](Guide-HTTP-Client)
- [String Interpolation](Guide-String-Interpolation)
- [Debugger](Guide-Debugger)
- [Vektor-Datenbanken](Guide-Vektor-Datenbanken)
- [CLI Reference](Guide-CLI-Reference)
- [API Keys Setup](Guide-API-Keys)
- [Advanced](Guide-Advanced)
- [Backend](Guide-Backend)
- [Security Best Practices](Guide-Security-Best-Practices)
- [AI/ML](Guide-AI-ML)
- [Auto Imports](Guide-Auto-Imports)
- [Plugin Development](Guide-Plugin-Development)

## Tools
- [Tools Overview](Tools-Overview)
- [VS Code Extension](Tools-VS-Code-Extension)
- [Auto Repair](Tools-Auto-Repair)
- [Security Scanner](Tools-Security-Scanner)
- [Linter](Tools-Linter)
- [Formatter](Tools-Formatter)
- [Debugger](Tools-Debugger)
- [Hot Reload](Tools-Hot-Reload)
- [Package Manager](Tools-Package-Manager)
- [API Doc Generator](Tools-API-Doc-Generator)
- [Library Generator](Tools-Library-Generator)

## API Reference
- [Standard Library](API-Standard-Library)
- [Decorators](API-Decorators)
- [Frameworks](API-Frameworks)
- [OpenAPI](API-OpenAPI)
"@
Set-Content -Path (Join-Path $WikiDir "_Sidebar.md") -Value $SidebarContent -Encoding UTF8

Write-Host ""
Write-Host "Wiki-Generierung abgeschlossen!" -ForegroundColor Green
Write-Host ""
Write-Host "Wiki-Dateien erstellt in: $WikiDir" -ForegroundColor Cyan
Write-Host ""
Write-Host "Naechste Schritte:" -ForegroundColor Yellow
Write-Host "   1. Pruefe die generierten Dateien in .wiki/"
Write-Host "   2. Push zu GitHub Wiki Repository:"
Write-Host "      cd .wiki"
Write-Host "      git init"
Write-Host "      git add ."
Write-Host "      git commit -m `"Update wiki`""
Write-Host "      git remote add origin https://github.com/SkyliteDesign/velinscript.wiki.git"
Write-Host "      git push -u origin master"
Write-Host ""
Write-Host "   Oder verwende GitHub Actions fuer automatische Updates (siehe .github/workflows/wiki.yml)" -ForegroundColor Cyan
