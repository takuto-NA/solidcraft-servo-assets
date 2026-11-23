# Attribution

| Files | Origin and license |
|---|---|
| `src/`, `catalog/`, `tests/`, `tools/`, `xtask/`, `docs/`, `.cargo/`, `.github/`, build scripts, manifests, README, CONTRIBUTING and AGENTS | Original Servo Assets implementation, catalog transcription of numeric facts, contribution templates and documentation. MIT OR Apache-2.0. Vendor source links and per-dimension evidence are retained in the catalog and `docs/sources.md`. No vendor PDF, product artwork or CAD file is redistributed. |
| Release CAD captures and demo designs | Original geometry constructed by this plugin and captured in SolidCraft. MIT OR Apache-2.0. Photo-estimated shapes retain separate source and inference records; the vendor photograph itself is not redistributed. |
| Release assets `servo-assets-overview-v1.svg` and `servo-assets-overview-v1.png` | Original SVG composition and its PNG export, using unmodified native SolidCraft captures of this plugin's v0.9.1 geometry. MIT OR Apache-2.0. The editable SVG embeds the captures, source IDs and camera metadata. It references system text fonts without redistributing font files. No vendor artwork or CAD media is included. |
| Release assets `servo-assets-readme-hero-v2.svg` and `servo-assets-readme-hero-v2.png` | Original SVG composition and PNG export using native SolidCraft renders of this plugin's v0.10.0 assets. MIT OR Apache-2.0. The SVG embeds the renders, catalog IDs and camera metadata. It uses the system Lato font for captions. |
| `icons/insert-servo.svg`, `icons/place-horn.svg`, `icons/place-datum.svg` | Original Servo Assets command icons. MIT OR Apache-2.0. |
| `icons/options.svg`, `icons/asset-id.svg`, `icons/search.svg`, `icons/sources.svg`, `icons/envelope.svg`, `icons/shaft-axis.svg`, `icons/cable-space.svg` | Original Servo Assets command icons. MIT OR Apache-2.0. |
| `docs/third-party-notices.md` | License notices reproduced from the permissively licensed registry dependencies in the Cargo lockfiles. Individual copyright holders and MIT, Unicode-3.0 or Zlib terms are retained in that file; these notices keep their original terms. |
| `wit/plugin.wit`, `wit-0.1/plugin.wit` | SolidCraft `crates/craft-plugin/wit/plugin.wit` and `wit-0.1/plugin.wit`, commit `0574266e77ce2b3f4daabbcaddaa696183415188`, copyright 2026 Tangency Labs and contributors, MIT OR Apache-2.0. https://github.com/tangency-labs/solidcraft |
| `wit/plugin.wit` storage and dialog interfaces | SolidCraft 0.47 API definitions at commit `56d2ede6`, copyright 2026 Tangency Labs and contributors, MIT OR Apache-2.0. [Host WIT](https://github.com/tangency-labs/solidcraft/blob/56d2ede6/crates/craft-plugin/wit/plugin.wit) |

The MIT-licensed [SolidCraft Gears](https://github.com/bherbruck/solidcraft-gears)
plugin was studied for the command-plugin packaging pattern. Its implementation,
artwork and geometry were not copied. Servo geometry uses source dimensions and
SolidCraft's own documented commands.

Servo product and manufacturer names identify the referenced hardware. They do not
imply endorsement or verified interchangeability. Datasheets remain external
reference material; the package contains original geometry instructions and facts.
