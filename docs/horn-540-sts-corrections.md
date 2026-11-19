# HN13 and STS3215 correction checklist

Audit date: 2026-11-19. All four records now have data revision 2. This is a
catalog correction, not a claim of a physical product revision.

| Previous construction | Source-backed correction | Current geometry or boundary |
| --- | --- | --- |
| HN13-N101 free plate 2.6 inferred from installed projection | Individual ROBOTIS drawing of 22-May-2019 directly specifies 2.6, total 4.9 and boss diameter 10 | Major flange diameter 26 × 2.6, outward boss diameter 10 × (4.9 − 2.6) = 2.3 |
| HN13-N101 blind depth 2.5 for all pattern holes | Individual drawing specifies eight M2.5 × 0.45 TAP THRU on PCD22 and four diameter 2 HOLE THRU on PCD16 | Exact through-pattern references; older assembly maximum-depth statements retained separately |
| HN13-N101 unmodelled center treated as an otherwise complete horn | Retaining-bore diameter and complete rear/cavity topology remain unavailable | Explicit partial major profile; center cavities, rear lip, teeth and edge details omitted |
| HN13-I101 estimated diameter 12 passage | Individual section specifies passage diameter 10, tube outside diameter 12 and flange 2.5 | No 3D part: tube axial span and complete steps are unknown; dimensions and hole pattern remain |
| STS polygonized raster silently interpreted as a finished outline | Explicit diameter 19.95 and hub diameter 9 coexist with polygonized pitch circles and holes | Literal circular major-profile references; no physical facet count, angles or traced vertices |
| STS main smooth diameter 5.9 pocket | Source prints nominal `25T (5.9)`, not a complete smooth-bore or mating profile | All main center cavities omitted; nominal size, clearance 3.2 and pocket depth 3.6 retained as facts |
| STS rear passage | Explicit diameter 6.05 and sectioned plain through topology | Diameter 6.05 passage retained |
| Extra 0.01/0.1 geometry margins | None are physical dimensions | Removed; exact published heights or their arithmetic differences are used |
| Eight STS installed origins derived from case midpoint and preview tips | Case registration and installed outer face are not fully dimensioned | `placement_basis: unknown`, `origin_mm: null`, no replacement parts and no automatic attachment |
| Two C001 lightguide compatibility proposals | Change notice does not establish horn fit | Both bindings removed |

The resulting local references are:

- HN13-N101: one partial major-profile solid; local Z−2.6 to +2.3.
- HN13-I101: `dimensions_only`, no 3D parts.
- STS main disc: one partial major profile; plate Z−2.5 to 0, diameter 9 hub
  extending to Z−4.5. Center cavities are omitted.
- STS rear disc: one partial major profile; plate Z−2.1 to 0, diameter 9 hub
  extending to Z−3.1, with the source-backed diameter 6.05 passage.

Nominal threaded-hole cylinders represent major-diameter pattern references.
They do not specify tap-drill sizes or reconstruct helical thread surfaces.
Small edge features and chamfers remain omitted explicitly.

The known XM540 outer-flange planes remain documented: normal
`[13.75, 16.75, 46.6]` toward +Z, idler `[13.75, 16.75, -2.5]` toward −Z.
The idler still cannot be attached as a 3D shape because its required axial
geometry is missing. No STS installed origin remains numeric.

Primary links, inspected pages, PDF hashes, conflicting dated hole callouts and
remaining schema needs are recorded in
[the full audit](horn-robotis-540-sts-audit.md). Earlier v0.7 gallery artifacts
must not be used as proof of these corrected shapes.
