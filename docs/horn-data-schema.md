# Horn accessory data

Horn packs are independent of servo packs. Hardware identity, a source dimension,
a complete contour, named compatibility and installed pose are separate claims.
Version 0.9, merged release `2026-11-19.10`, permits the user-requested
photo-based reconstruction with per-feature evidence. Published facts, printed
annotations and inferred geometry remain separate. A grounded photo estimate
can supply visible shape without claiming a verified manufacturing dimension,
fit or seat. Earlier source snapshots remain historical records.

Each `catalog/horns-*.json` contains schema version 1, release, sources, horns and
mounts. Source IDs are unique across horn packs and use HTTPS references.
Vendor images, PDFs and CAD files are not distributed.

## Horn records

- Stable `id`, manufacturer, model and positive `data_revision` identify a source
  variant. Unknown manufacturer or physical revision is not inferred from shape.
- `status: dimensions_only` requires `parts: []`. Published dimensions and known
  hole patterns remain usable even when no unique solid contour is established.
- `status: partial_dimensions` requires one or more supported outer-profile
  parts. Omitted pockets, teeth and contours are listed explicitly. It does not
  mean the part is complete, printable, guaranteed to mate or safe as a complete
  interference envelope.
- `status: photo_estimate` requires parts and a nonempty `estimates` map.
  Every part has `basis: photo_estimate`. These are calibrated reconstructed
  shapes, explicitly labelled as estimates; they are available only through
  detached `photo-solid` insertion and never automatic servo assembly.
- `dimensions` contains source Dimension records. Numeric choices without a
  source or explicit source derivation must not be promoted to factual dimensions.
  Grounded numerical shape interpretations belong in `estimates` instead.
- Printed photograph annotations use evidence method `photo_annotation`, only
  in `reported_*` dimensions of a `dimensions_only` or `photo_estimate` record.
  They record what the image says; decimal places do not establish measurement
  accuracy. A separate estimate explains any geometric interpretation used to
  constrain a reconstruction. An annotation alone is not a formal hole datum.
- `holes` contains only located source patterns, using local XY. A named thread's
  major diameter is a reference circle, not its pilot drill. Unknown hole phase,
  thread pitch and engagement depth remain unknown.
- `parts` contains original box, cylinder or analytic profile constructions, at
  most eight parts with at most 64 modifiers each. Each part explains its basis
  and omissions. Source solids do not fill missing holes or substitute a nominal
  spline class with a fitted round bore. Photo solids may include an explicitly
  inferred visual cavity with evidence; it remains a shape approximation without
  a spline-fit claim. Removing an unknown cavity from a source solid leaves a
  partial outer volume and must be stated as such.
- `assumptions`, source-linked `observations` and `unknown` describe limitations.
  A warning does not replace per-feature provenance for an estimate.

## Numerical estimates and analytic profiles

`Horn.estimates` is a map from a stable feature key to a `GeometryEstimate`:

- `value`: finite numeric estimate.
- `unit`: `mm`, `deg`, `count` or `ratio`; a count is a positive integer.
- `evidence`: Source ID, locator, interpretation note and method `photo_trace`
  or `photo_inference`. A trace records image-derived geometry; an inference
  explains a relation or modeling interpretation constrained by that image.
- `source_pixel_points`: original-image `[x,y]` anchors when available. An empty
  list is permitted when the evidence note explains the source relation or
  analytic derivation; linked reconstruction notes retain the calibration.
- `uncertainty_mm`: known millimetre uncertainty or null. Null is unknown, not
  zero. Do not infer a physical tolerance from pixel count or printed precision.

Only photo-estimate records have a nonempty estimates map. The current two
reconstructions keep `holes: []`: their twelve/seven linkage cuts and centers are
recorded as estimates, not as verified `MountHole` data. Their printed
`reported_*` labels remain unchanged beside those estimates. Cup width and depth
interpretations do not define a mating spline or installed seat.

`Profile` geometry uses `kind: profile`, `z_mm`, `height_mm` and a bounded,
closed, nondegenerate sequence of segments. A `kind: line` segment has
`start_mm` and `end_mm`. A `kind: arc` segment additionally has `center_mm` and
`ccw`. The adapter creates its own sketch and extrudes the owned sketch ID.
Tapers and circular tips remain analytic geometry; they are not reconstructed
from display meshes or unrelated active sketches. Profile joins/cuts target
only bodies created by the insertion.

`tools/update-photo-horns.py` reproduces the two supplied-photo recipes. The
[reconstruction notes](sg90-photo-reconstruction.md) give pixel anchors, tangent
relations, inferred hole positions and unresolved annotations; the
[source review](sg90-photo-review.md) compares other documented variants.

For solid references, local origin is the shaft center on the outer plate face,
Z0, with +Z outward from the actuator. A rear hub usually extends toward -Z;
a front boss can extend toward +Z. Use the source section, not a general horn
convention. Hole-reference sketches use the same local XY without inferring an
installed seating plane.

## Mount records

Compatibility metadata can exist while placement is unknown:

```json
{
  "asset_id": "canonical.servo.id",
  "horn_id": "canonical.horn.id",
  "output_id": "main",
  "frame": "source_case_frame",
  "compatibility": "documented",
  "compatibility_evidence": {"source":"source-id","locator":"named compatibility table","method":"specification","note":"Named physical accessory pairing."},
  "origin_mm": null,
  "direction": [0, 0, 1],
  "placement_basis": "unknown",
  "placement_evidence": null,
  "replace_model_parts": [],
  "note": "Installed outer plate-face height is unknown."
}
```

`placement_basis` is `documented` or `unknown`. Unknown requires null origin,
null placement evidence and no replacement roles. Do not retain a guessed
coordinate from a preview shaft. A documented pose requires an explicit source
origin, evidence and agreement with the servo's located output centerline.
The source must locate the outer plate face relative to the same case datum;
a loose-part thickness or a pictured shaft does not establish that relation.

Automatic assembly requires documented compatibility, documented placement,
a supported partial solid and a status other than `photo_estimate`. Photo
estimates remain detached even if a binding is later recorded. Other
combinations are refused before host commands.
Unverified compatibility cannot create an assembly. The unsupported AU/TowerPro
and changed-case STS proposals have been removed rather than kept as menu choices.

`replace_model_parts` may replace only named generated reference roles. Unknown
X330 seating no longer removes a factory output face. Source revisions remain
separate; tooth count or similar appearance does not establish compatibility.

## Insertion and saved evidence

`horn-reference` accepts a horn ID, mode, XYZ and visual angle:

| Mode | Accepted representation |
| --- | --- |
| `dimensions` (default) | Known circular references and annotations, without bodies |
| `source-solid` | A supported partial source solid; rejects dimensions-only and photo-estimate records |
| `photo-solid` | A photo-estimate reconstruction with estimated-contour/hole labels; rejects other statuses |

These are detached references. Invalid mode/status combinations are refused
before host commands. A photo solid provides the requested inferred shape,
while dimensions mode remains available for the original labels. No mode infers
compatibility or an installed pose; world placement is a user layout.

`servo-reference` can attach only the combinations satisfying the automatic
assembly requirements. Its visual angle is not an electrical neutral setting.
Axial adjustment is a recorded user offset from the documented baseline.

Saved source snapshots contain canonical identity, revision, source records,
complete fact data, separate per-feature estimates, input parameters, actual
commands and owned body names.
Dimension-only references save the annotation sketch and no bodies. No operation
may cut, rename or move a pre-existing unrelated body.
