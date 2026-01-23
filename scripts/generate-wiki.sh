#!/bin/bash

# GitHub Wiki Generator für VelinScript
# Konvertiert docs/ in GitHub Wiki Format

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
DOCS_DIR="$PROJECT_ROOT/docs"
WIKI_DIR="$PROJECT_ROOT/.wiki"

echo "🚀 GitHub Wiki Generator für VelinScript"
echo "=========================================="
echo ""

# Wiki-Verzeichnis erstellen
mkdir -p "$WIKI_DIR"

# Funktion: Konvertiere Markdown für GitHub Wiki
convert_to_wiki() {
    local input_file="$1"
    local output_file="$2"
    local relative_path="$3"
    
    # Erstelle Ausgabe-Verzeichnis
    mkdir -p "$(dirname "$output_file")"
    
    # Kopiere Datei und konvertiere Links
    sed -E '
        # Konvertiere relative Links zu Wiki-Links
        s|\[([^\]]+)\]\(([^)]+\.md)\)|[\1](\2)|g
        s|\[([^\]]+)\]\(guides/([^)]+\.md)\)|[\1](guides-\2)|g
        s|\[([^\]]+)\]\(architecture/([^)]+\.md)\)|[\1](architecture-\2)|g
        s|\[([^\]]+)\]\(tools/([^)]+\.md)\)|[\1](tools-\2)|g
        s|\[([^\]]+)\]\(api/([^)]+\.md)\)|[\1](api-\2)|g
        s|\[([^\]]+)\]\(language/([^)]+\.md)\)|[\1](language-\2)|g
        s|\.md\)|)|g
        
        # Entferne Datei-Erweiterungen aus Links
        s|\(([^)]+)\.md\)|(\1)|g
    ' "$input_file" > "$output_file"
    
    # Füge Navigation am Ende hinzu (außer für Home)
    if [[ "$relative_path" != "Home.md" ]]; then
        echo "" >> "$output_file"
        echo "---" >> "$output_file"
        echo "" >> "$output_file"
        echo "**← [Zurück zur Übersicht](Home)**" >> "$output_file"
    fi
}

# Funktion: Erstelle Wiki-Seite
create_wiki_page() {
    local source_file="$1"
    local wiki_name="$2"
    local output_file="$WIKI_DIR/$wiki_name.md"
    
    if [[ -f "$source_file" ]]; then
        echo "  ✓ $wiki_name"
        convert_to_wiki "$source_file" "$output_file" "$wiki_name.md"
    else
        echo "  ⚠️  $wiki_name (Datei nicht gefunden: $source_file)"
    fi
}

echo "📝 Erstelle Wiki-Seiten..."
echo ""

# Home-Seite (aus docs/README.md)
echo "📄 Home.md"
create_wiki_page "$DOCS_DIR/README.md" "Home"

# Getting Started
echo ""
echo "📚 Getting Started..."
create_wiki_page "$DOCS_DIR/guides/getting-started.md" "Getting-Started"
create_wiki_page "$PROJECT_ROOT/QUICK_START.md" "Quick-Start"

# Language Specification
echo ""
echo "📋 Language..."
create_wiki_page "$DOCS_DIR/language/specification.md" "Language-Specification"
create_wiki_page "$DOCS_DIR/language/basics.md" "Language-Basics"

# Architecture
echo ""
echo "🏛️  Architecture..."
create_wiki_page "$DOCS_DIR/architecture/compiler-architecture.md" "Architecture-Compiler"
create_wiki_page "$DOCS_DIR/architecture/pass-verlauf.md" "Architecture-Pass-Verlauf"
create_wiki_page "$DOCS_DIR/architecture/type-inference.md" "Architecture-Type-Inference"
create_wiki_page "$DOCS_DIR/architecture/code-ordering-pass.md" "Architecture-Code-Ordering"
create_wiki_page "$DOCS_DIR/architecture/ir-representation.md" "Architecture-IR"
create_wiki_page "$DOCS_DIR/architecture/borrow-checker.md" "Architecture-Borrow-Checker"
create_wiki_page "$DOCS_DIR/architecture/code-generation.md" "Architecture-Code-Generation"
create_wiki_page "$DOCS_DIR/architecture/multi-target-compilation.md" "Architecture-Multi-Target"
create_wiki_page "$DOCS_DIR/architecture/module-resolution.md" "Architecture-Module-Resolution"
create_wiki_page "$DOCS_DIR/architecture/framework-integration.md" "Architecture-Framework-Integration"
create_wiki_page "$DOCS_DIR/architecture/parallelization.md" "Architecture-Parallelization"
create_wiki_page "$DOCS_DIR/architecture/ai-compiler-passes.md" "Architecture-AI-Passes"
create_wiki_page "$DOCS_DIR/architecture/prompt-optimizer.md" "Architecture-Prompt-Optimizer"
create_wiki_page "$DOCS_DIR/architecture/system-generation.md" "Architecture-System-Generation"

# Guides
echo ""
echo "📖 Guides..."
create_wiki_page "$DOCS_DIR/guides/tutorial-1-basics.md" "Guide-Basics"
create_wiki_page "$DOCS_DIR/guides/tutorial-2-apis.md" "Guide-APIs"
create_wiki_page "$DOCS_DIR/guides/tutorial-3-security.md" "Guide-Security"
create_wiki_page "$DOCS_DIR/guides/tutorial-4-database.md" "Guide-Database"
create_wiki_page "$DOCS_DIR/guides/tutorial-5-validation.md" "Guide-Validation"
create_wiki_page "$DOCS_DIR/guides/tutorial-6-authentication.md" "Guide-Authentication"
create_wiki_page "$DOCS_DIR/guides/tutorial-7-ml.md" "Guide-ML-LLM"
create_wiki_page "$DOCS_DIR/guides/tutorial-8-intelligence.md" "Guide-Intelligence"
create_wiki_page "$DOCS_DIR/guides/tutorial-type-inference.md" "Guide-Type-Inference"
create_wiki_page "$DOCS_DIR/guides/tutorial-ml-training.md" "Guide-ML-Training"
create_wiki_page "$DOCS_DIR/guides/tutorial-pattern-matching.md" "Guide-Pattern-Matching"
create_wiki_page "$DOCS_DIR/guides/tutorial-closures.md" "Guide-Closures"
create_wiki_page "$DOCS_DIR/guides/tutorial-collections.md" "Guide-Collections"
create_wiki_page "$DOCS_DIR/guides/tutorial-http-client.md" "Guide-HTTP-Client"
create_wiki_page "$DOCS_DIR/guides/tutorial-string-interpolation.md" "Guide-String-Interpolation"
create_wiki_page "$DOCS_DIR/guides/tutorial-debugger.md" "Guide-Debugger"
create_wiki_page "$DOCS_DIR/guides/vektor-datenbanken.md" "Guide-Vektor-Datenbanken"
create_wiki_page "$DOCS_DIR/guides/cli-reference.md" "Guide-CLI-Reference"
create_wiki_page "$DOCS_DIR/guides/api-keys-setup.md" "Guide-API-Keys"
create_wiki_page "$DOCS_DIR/guides/advanced.md" "Guide-Advanced"
create_wiki_page "$DOCS_DIR/guides/backend.md" "Guide-Backend"
create_wiki_page "$DOCS_DIR/guides/security.md" "Guide-Security-Best-Practices"
create_wiki_page "$DOCS_DIR/guides/ai-ml.md" "Guide-AI-ML"
create_wiki_page "$DOCS_DIR/guides/auto-imports.md" "Guide-Auto-Imports"
create_wiki_page "$DOCS_DIR/guides/plugin-development.md" "Guide-Plugin-Development"

# Tools
echo ""
echo "🛠️  Tools..."
create_wiki_page "$DOCS_DIR/tools/TOOLS_ÜBERSICHT.md" "Tools-Overview"
create_wiki_page "$DOCS_DIR/tools/vscode-extension.md" "Tools-VS-Code-Extension"
create_wiki_page "$DOCS_DIR/tools/auto-repair.md" "Tools-Auto-Repair"
create_wiki_page "$DOCS_DIR/tools/security-scanner.md" "Tools-Security-Scanner"
create_wiki_page "$DOCS_DIR/tools/linter.md" "Tools-Linter"
create_wiki_page "$DOCS_DIR/tools/formatter.md" "Tools-Formatter"
create_wiki_page "$DOCS_DIR/tools/debugger.md" "Tools-Debugger"
create_wiki_page "$DOCS_DIR/tools/hot-reload.md" "Tools-Hot-Reload"
create_wiki_page "$DOCS_DIR/tools/package-manager.md" "Tools-Package-Manager"
create_wiki_page "$DOCS_DIR/tools/api-doc-generator.md" "Tools-API-Doc-Generator"
create_wiki_page "$DOCS_DIR/tools/library-generator.md" "Tools-Library-Generator"

# API
echo ""
echo "📚 API..."
create_wiki_page "$DOCS_DIR/api/standard-library.md" "API-Standard-Library"
create_wiki_page "$DOCS_DIR/api/decorators.md" "API-Decorators"
create_wiki_page "$DOCS_DIR/api/frameworks.md" "API-Frameworks"
create_wiki_page "$DOCS_DIR/api/openapi.md" "API-OpenAPI"

# Erstelle _Sidebar.md für GitHub Wiki Navigation
echo ""
echo "📑 Erstelle _Sidebar.md..."
cat > "$WIKI_DIR/_Sidebar.md" << 'EOF'
# Navigation

## 🚀 Schnellstart
- [Home](Home)
- [Getting Started](Getting-Started)
- [Quick Start](Quick-Start)

## 📋 Sprache
- [Language Specification](Language-Specification)
- [Language Basics](Language-Basics)

## 🏛️ Architektur
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

## 📖 Guides
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

## 🛠️ Tools
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

## 📚 API Reference
- [Standard Library](API-Standard-Library)
- [Decorators](API-Decorators)
- [Frameworks](API-Frameworks)
- [OpenAPI](API-OpenAPI)
EOF

echo ""
echo "✅ Wiki-Generierung abgeschlossen!"
echo ""
echo "📁 Wiki-Dateien erstellt in: $WIKI_DIR"
echo ""
echo "📝 Nächste Schritte:"
echo "   1. Prüfe die generierten Dateien in .wiki/"
echo "   2. Push zu GitHub Wiki Repository:"
echo "      cd .wiki"
echo "      git init"
echo "      git add ."
echo "      git commit -m 'Update wiki'"
echo "      git remote add origin https://github.com/SkyliteDesign/velinscript.wiki.git"
echo "      git push -u origin master"
echo ""
echo "   Oder verwende GitHub Actions für automatische Updates (siehe .github/workflows/wiki.yml)"
