# Insert a 3D servo or horn

1. Open the **Model** tab and find **Plugins**. Choose **Servo model** for a
   servo or **Horn model** for a separate horn.
2. Enter a model number or name in the filter and select the matching model.
   The servo filter also accepts manufacturers and seller SKUs.
3. Review the preview and click **Run**. **Cancel** closes the preview.
   **Undo** removes the complete insertion in one step.

A servo appears as one named component in the Browser. Use the component's
**Move** action to move the housing, shaft and cable markers together. The horn
picker offers partial solids based on drawings and clearly labelled photo
estimates. A horn inserts as a separate part at its source origin. See the
[model limits and evidence](usage.md) for shape, fit and placement details.

## Why the toolbar has two actions

Each action names and produces a 3D model. Model lookup stays in the insertion
dialog, alongside the catalog choices and the preview. The form also keeps
electrically distinct variants such as `XM430-W350-T` and `XM430-W350-R` separate.
[Windows ribbon guidance](https://learn.microsoft.com/en-us/windows/win32/uxguide/cmd-ribbons#commands) · [Onshape insertion guide](https://cad.onshape.com/help/Content/Assembly/insert_parts_and_assemblies.htm)

SolidCraft 0.47 puts every declared command in the Model plugin group.
Developers can build the detailed command set with
[`./build.sh --tools`](releasing.md#local-builds). That package includes
mounting sketches, output references and the source-data commands.
[SolidCraft 0.47 plugin entries](https://github.com/tangency-labs/solidcraft/blob/v0.47.0/docs/plugins.md#menu-and-ribbon-entries)
