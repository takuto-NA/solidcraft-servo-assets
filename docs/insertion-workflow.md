# Insert a servo, horn or datum

1. In the **Insert** tab, open **Servo Assets → Insert servo**. Choose the exact
   model and case variant. A preview starts after a choice is made. Inspect the
   mount, shaft and cable entrance before accepting the insertion.
2. For a separate horn, choose **Insert horn**. Each choice names both the
   variant and its representation. Dimension references, partial solids and
   photo estimates have distinct labels. A loose horn uses its own reference
   frame. The **Servo setup** command retains the documented fitted options.
3. For a source datum, open **Inspect → Servo Assets → Insert mounts or axis**.
   Choose a model and one of its documented mounting faces or axes. The output
   reference includes its related mounting sketch or case envelope. A preview
   model can still be useful when its exact datum is missing.

The three commands also appear under **Model → Plugins → Servo Assets**. Search
with a model name such as `SG90`, `C001` or `XM430`. When the host form supports
search, type in the picker and choose the matching row. The selected description
identifies the evidence and origin. Open **Advanced** for X/Y/Z offsets and
optional interface callouts. Move an
accepted reference with SolidCraft's normal tools. Closing the preview leaves the
timeline alone. An accepted insertion is one undo step.

See the [installation and download guide](../README.md#install-and-use) for the
matching ZIP. If the installer reports `manifest size exceeds 16 KiB`, use the
compatibility ZIP. The modern package needs a host that accepts 64 KiB plugin
manifests and supports `commands@0.2` and `storage@0.1`.
Older hosts use a standard list for the same catalog choices, with visible offset
fields. The host extension adds filtering and the summary above the preview. The
plugin's forms work with or without that presentation extension.

## Interpret what is visible

The model selector flags preview positions before insertion. The detailed model
summary shows the nominal case envelope when available, the status of the mount
and output datums, the displayed cable interface and the selected source origin.
Dimensions of the case envelope exclude accessories. For an FS90 the origin is
the near mounting centre at the ear underside. Most serial models use the
minimum corner of their source envelope. Use **Read servo sources** for the full
coordinates, tolerances, evidence and related source links.

On the usual fixed lead, three short conductors show one cable entrance. Outlet
position and cross-section remain planning geometry where the source lacks them.
Serial socket and contact counts are separate from their approximate placement
and shape. For measured routing space, enter the measured or planned values in
**Plan cable space**. Preview geometry carries its own limits and omissions.

**Find assets** accepts model numbers, manufacturers and seller SKUs. For
example, `STS3215 C001` lists old and light-guide case variants separately and
`XL330 M288` finds the full XL330 model number. Uncertain identity stays attached
to SG90-style photo horns. A precise match to a search term never establishes
interchangeability.

## Why this ordering

The previous form put 14 inputs in a single dialog, including cable dimensions,
labels and horn position. A person looking for a mount first had to distinguish
six insertion commands and find related options across separate lists.

| Research | Applied to Servo Assets |
| --- | --- |
| Autodesk documents identifying library parts by component name or full part variant before looking at a preview. [Fusion Electronics Library Manager](https://help.autodesk.com/cloudhelp/ENU/Fusion-ECAD/files/ECAD-SEARCH-CMPNT-TSK.htm) | Use model names and exact variants in the picker. Allow part numbers and seller SKUs in Find assets. |
| The Onshape standard-content workflow chooses compatible categories and part options before placing the result. [Onshape Standard Content](https://cad.onshape.com/help/Content/Assembly/standard_content.htm) | Pair a model with a real source datum and pair each horn with its available representations. Preview before accepting. |
| Making options recognisable, reducing avoidable errors and providing feedback are established usability heuristics. [Nielsen Norman Group usability heuristics](https://www.nngroup.com/articles/ten-usability-heuristics/) | Put evidence in option names and show the selected reference's limits. Keep Cancel and one-step Undo available. |
| Progressive disclosure exposes frequent controls first. Keyboard users should be able to look up and select an option without committing the form. [Progressive disclosure](https://www.nngroup.com/articles/progressive-disclosure/) · [W3C Combobox Pattern](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/) | Start with one required choice. The host's existing searchable picker and disclosure control supply the keyboard and advanced-placement behaviour. |

The design uses SolidCraft's native forms, commands and viewport. Existing entry
IDs and input parameters are available for automation. The new IDs are
`place-servo`, `place-horn` and `place-datum`. See [usage](usage.md) for the
detailed commands and [adding an asset](adding-assets.md) for catalog changes.
