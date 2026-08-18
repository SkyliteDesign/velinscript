# Velisch ↔ VelinScript Sync

## Regel

**Kein Push / Upload auf GitHub ohne ausdrückliche Freigabe (go).**

Zwei öffentliche Repos:

| Repo | Rolle | URL |
|------|--------|-----|
| **VelinScript** | Compiler / Core (`velin`) | https://github.com/SkyliteDesign/velinscript |
| **Velisch** | Sprache / Produktfläche | https://github.com/SkyliteDesign/velisch |

Stand 3.5.1: beide Remotes zeigen noch denselben Compiler-Baum (Spiegel). Zielbild: Velisch ohne `compiler/src/**` (nur Guides, Examples, Downloads).

## Was synchronisiert wird (bei Freigabe)

| Von VelinScript | Nach Velisch |
|-----------------|--------------|
| Learn-Examples (Hello, Auth, …) | `examples/` |
| User-facing Guides (kuratiert) | `docs/` |
| Version-Manifest (`3.5.1`) | Root / `VERSION` |
| Release-Asset-URLs | Downloads-Abschnitt (`v3.5.1`) |

## Was nicht nach Velisch gehört

- `compiler/src/**`
- IR / Passes / interne Architektur (nur verlinken)
- `velin_dev/` (lokal)

## Ablauf (wenn **go**)

1. VelinScript: `git push origin master` und Tag `v3.5.1` (optional `v3.5.1-ga`)
2. Release-Artefakte am VelinScript-Tag
3. Velisch: gleicher Tag **oder** kuratierte Produktfläche (Vorlage `docs/velisch-product/`)
4. Gleiche Produktversion **3.5.1** ausweisen

## Lokal

Vorlage unter `docs/velisch-product/`. Tags werden lokal angelegt; Push erst nach go.
