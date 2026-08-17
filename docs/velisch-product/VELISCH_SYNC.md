# Velisch ↔ VelinScript Sync

## Regel

**Kein Push / Upload auf GitHub ohne ausdrückliche Freigabe.**

## Was synchronisiert wird (bei Freigabe)

| Von VelinScript | Nach Velisch |
|-----------------|--------------|
| Learn-Examples (Hello, Auth, …) | `examples/` |
| User-facing Guides (kuratiert) | `docs/` |
| Version-Manifest (`3.5.0`) | Root / `VERSION` |
| Release-Asset-URLs | Downloads-Abschnitt |

## Was nicht nach Velisch gehört

- `compiler/src/**`
- IR / Passes / interne Architektur (nur verlinken)

## Ablauf (wenn freigegeben)

1. VelinScript Release-Tag `v3.5.0-ga` + Artefakte  
2. Action oder manuelles Copy der kuratierten Pfade  
3. Velisch README/Downloads aktualisieren  
4. Gleiche Produktversion `3.5.0` ausweisen  

## Lokal jetzt

Vorlage unter `docs/velisch-product/` — bereit zum späteren Kopieren ins private Velisch-Repo.
