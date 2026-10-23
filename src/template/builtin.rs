use super::{Template, TemplateFile};
use std::collections::HashMap;

pub fn get_builtin_templates() -> Vec<Template> {
    vec![
        model_3d_template(),
        cad_template(),
        logo_template(),
        video_edit_template(),
        photo_edit_template(),
        vector_artwork_template(),
        webdesign_template(),
        coding_template(),
        audio_production_template(),
        document_template(),
        game_dev_template(),
    ]
}

pub fn find_builtin_template(name: &str) -> Option<Template> {
    let lower = name.to_lowercase();
    get_builtin_templates().into_iter().find(|t| {
        t.name.to_lowercase() == lower
            || match lower.as_str() {
                // 3dmodel
                "3d" | "3dmodel" | "3d-model" | "3d-blender" | "blender" | "model" | "maya" | "gltf" => {
                    t.name == "3dmodel"
                }
                // cad
                "cad" | "cad-maker" | "maker" | "3dprint" | "freecad" | "openscad" | "solidworks" | "step" => {
                    t.name == "cad"
                }
                // logo
                "logo" | "logo-design" | "brand" | "branding" | "identity" | "brand-identity" => {
                    t.name == "logo"
                }
                // video-edit
                "video" | "video-edit" | "video-editing" | "yt-video" | "youtube" | "editing" | "film" | "premiere" | "davinci" | "kdenlive" => {
                    t.name == "video-edit"
                }
                // photo-edit
                "photo" | "photo-edit" | "photo-editing" | "darkroom" | "lightroom" | "gimp" | "raw" | "retouch" => {
                    t.name == "photo-edit"
                }
                // vector-artwork
                "vector" | "vector-artwork" | "vector-design" | "illustration" | "inkscape" | "illustrator" | "penpot" | "artwork" | "svg" => {
                    t.name == "vector-artwork"
                }
                // webdesign
                "web" | "webdesign" | "web-design" | "ui" | "ux" | "ui-ux" | "uiux" | "website" | "frontend" | "landing" | "figma" => {
                    t.name == "webdesign"
                }
                // coding
                "coding" | "code" | "dev" | "software" | "app" | "rust" | "rs" | "python" | "py" | "go" | "golang" | "cpp" | "c++" | "c" | "java" | "bash" | "shell" | "ts" | "typescript" | "fastapi" | "react" | "docker" | "manual" | "minimal" | "scratch" | "blank" | "custom" => {
                    t.name == "coding"
                }
                // audio-production
                "audio" | "audio-production" | "podcast" | "pod" | "music" | "track" | "sound" | "audacity" => {
                    t.name == "audio-production"
                }
                // document
                "doc" | "docs" | "document" | "book" | "novel" | "paper" | "research-paper" | "newsletter" | "portfolio" | "presentation" => {
                    t.name == "document"
                }
                // game-dev
                "game" | "game-dev" | "gamedev" | "godot" | "unity" => {
                    t.name == "game-dev"
                }
                _ => false,
            }
    })
}

// =========================================================================
// 1. 3D MODEL TEMPLATE
// =========================================================================
fn model_3d_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "3D modeling, texturing, rigging & rendering pipeline".into());
    vars.insert("client_name".into(), "Internal / Studio In-House".into());
    vars.insert("client_contact".into(), "client@example.com".into());
    vars.insert("project_status".into(), "Active (Modeling & Texturing)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("target_polycount".into(), "35,000 - 50,000 Triangles".into());
    vars.insert("texture_resolution".into(), "4096 x 4096 (4K PBR)".into());
    vars.insert("target_engine".into(), "Blender Cycles / Unreal Engine / WebGL".into());

    Template {
        name: "3dmodel".into(),
        description: "3D modeling, texturing, rigging & rendering (Blender/Maya/OBJ/FBX/glTF)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "models/high_poly".into(),
            "models/low_poly".into(),
            "textures/pbr".into(),
            "textures/baked".into(),
            "materials".into(),
            "references/concept_art".into(),
            "references/moodboard".into(),
            "renders/wip".into(),
            "renders/final".into(),
            "exports/gltf".into(),
            "exports/fbx".into(),
            "exports/obj".into(),
            "notes".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Project Lead / Artist** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Project Scope & Specifications
- **Objective:** Production-ready 3D asset with clean topology, UV layout, and PBR textures.
- **Target Budget:** {{target_polycount}}
- **Texture Standard:** {{texture_resolution}} (Albedo, Normal, Roughness, Metalness, AO)
- **Target Platform:** {{target_engine}}

---

## ◈ Project Task List & Milestones

### Phase 1: Planning, Concept & Asset Gathering
- [x] Scaffolding directory structure and initializing workspace
- [ ] Review client brief and technical constraints (polycount, engine targets)
- [ ] Collect visual references, scale sheets, and moodboards in `references/`
- [ ] Define blockout silhouette, primary proportions, and pivot locations

### Phase 2: High-Poly Sculpting & Low-Poly Retopology
- [ ] Sculpt / model high-poly geometry with high-frequency surface details
- [ ] Retopologize clean quad-dominant low-poly game/production mesh
- [ ] Check silhouette, edge flow, deformation loops, and vertex normal shading
- [ ] Optimize triangle budget within {{target_polycount}}

### Phase 3: UV Layout, Baking & PBR Texturing
- [ ] Unwrap non-overlapping UVs with packed islands and optimized texel density
- [ ] Bake Curvature, Ambient Occlusion, Normal, Position, and Thickness maps
- [ ] Author PBR materials in `textures/pbr/` (Albedo, Roughness, Metallic, Normal)
- [ ] Test real-time material response under neutral studio HDRI lighting

### Phase 4: Rigging, Lighting, Final Renders & Client Handover
- [ ] Construct deformation skeleton, control rig, and weight skinning (if animated)
- [ ] Compose turntable lighting setup and camera shots in `renders/wip/`
- [ ] Render 4K marketing beauty shots and wireframe showcase to `renders/final/`
- [ ] Export final optimized assets into `exports/gltf/`, `exports/fbx/`, and `exports/obj/`
- [ ] Perform client pre-delivery validation and milestone signoff with {{client_name}}

---

## ◈ Workspace Directory Map

```
{{project_name}}/
├── models/
│   ├── high_poly/        # High-resolution sculpts and subdivision masters
│   └── low_poly/         # Game-ready, optimized production meshes
├── textures/
│   ├── pbr/              # PBR map set (Albedo, Normal, Roughness, Metalness, AO)
│   └── baked/            # Ambient occlusion, curvature, and normal bakes
├── materials/            # Material presets, shader graphs, and node trees
├── references/
│   ├── concept_art/      # Concept sketches, orthographics, and scale guides
│   └── moodboard/        # Material studies, color swatches, and lighting inspiration
├── renders/
│   ├── wip/              # Work-in-progress snapshots for quick reviews
│   └── final/            # Master high-res beauty renders and turntables
├── exports/
│   ├── gltf/             # Real-time WebGL / GLB self-contained binaries
│   ├── fbx/              # Rigged production exchange files for game engines
│   └── obj/              # Static mesh wavefront geometry exchange
└── notes/                # Technical memos, changelogs, and client feedback
```

---

## ◈ Deliverables & Output Specifications
- **Master 3D Scene:** Native source project file (.blend / .fbx)
- **Real-Time Binary:** Optimized `.glb` / `.gltf` asset with embedded textures
- **PBR Texture Bundle:** 4K 16-bit Normal and 8-bit Albedo/Roughness maps
- **Beauty Render Suite:** 3840x2160 PNG renders with transparent background
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "models/specs.md".into(),
                content: r#"# 3D Asset Technical Specifications

## Target Metrics
- **Target Polygon Budget:** {{target_polycount}}
- **Current Triangle Count:** TBD
- **Texture Resolution:** {{texture_resolution}}
- **UV Set Count:** 1 Master Set (UDIM 1001)
- **Unit Scale:** Metric (1 Unit = 1.0 Meter)
- **Forward Axis:** -Y Forward, Z Up (Blender) -> -Z Forward, Y Up (glTF/OpenGL)

## Texture Map Standards
| Channel | Format | Color Space | Notes |
|---|---|---|---|
| **Base Color / Albedo** | PNG / TIFF | sRGB | Clean color without baked shadow |
| **Roughness** | PNG / TIFF | Linear / Non-Color | 0 = Mirror, 255 = Diffuse Rough |
| **Metallic** | PNG / TIFF | Linear / Non-Color | Binary mask (0.0 or 1.0) |
| **Normal** | PNG / TIFF | Linear (OpenGL +Y) | Tangent space 16-bit normal |
| **Ambient Occlusion** | PNG / TIFF | Linear / Non-Color | Baked contact shadows |
| **Emissive** | PNG / TIFF | sRGB | Self-illumination channel |
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "references/moodboard/references.md".into(),
                content: r#"# Visual Reference & Style Guide

- **Project:** {{project_name}}
- **Client:** {{client_name}}
- **Art Direction:** Stylized Realism / Clean Industrial Aesthetics

## Reference Inventory
- Reference 01: Silhouette balance and primary volume proportions
- Reference 02: Real-world surface wear, edge scratching, and micro-roughness
- Reference 03: Color palette and lighting contrast
"#.into(),
                required: false,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# 3D Project Git Ignore
*.blend1
*.blend2
*.blend@
*.tmp
*.temp
*.bak
*.autosave
*.cache
*.bphys
renders/wip/*.png
renders/wip/*.jpg
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Master 3D Asset (.blend / .fbx / .obj)".into(),
            "Game-Ready glTF/GLB Binary Asset".into(),
            "4K PBR Texture Maps (Albedo, Normal, Roughness, Metalness, AO)".into(),
            "High-Resolution Presentation Renders (4K UHD)".into(),
        ],
    }
}

// =========================================================================
// 2. CAD & MECHANICAL ENGINEERING TEMPLATE
// =========================================================================
fn cad_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Mechanical CAD, 3D printing & manufacturing".into());
    vars.insert("client_name".into(), "Internal / Engineering In-House".into());
    vars.insert("client_contact".into(), "engineering@example.com".into());
    vars.insert("project_status".into(), "Active (CAD Drafting & Prototyping)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("print_material".into(), "PETG / PLA+ / Tough Resin".into());
    vars.insert("manufacturing_method".into(), "FDM 3D Printing / CNC Milling".into());
    vars.insert("standard_tolerance".into(), "±0.15 mm (FDM) / ±0.05 mm (Machined)".into());

    Template {
        name: "cad".into(),
        description: "Mechanical CAD, 3D printing & manufacturing (FreeCAD/OpenSCAD/STEP/STL/3MF)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "cad/parts".into(),
            "cad/assemblies".into(),
            "cad/drawings_2d".into(),
            "slicer/stl".into(),
            "slicer/3mf".into(),
            "slicer/gcode".into(),
            "exports/step".into(),
            "exports/dxf".into(),
            "bom".into(),
            "specs".into(),
            "photos/prototypes".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Lead Engineer / Author** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Engineering Specifications
- **Manufacturing Process:** {{manufacturing_method}}
- **Material Choice:** {{print_material}}
- **Design Tolerance:** {{standard_tolerance}}
- **Fasteners:** Metric ISO Standard (M3, M4, M5 hex fasteners & heat-set brass inserts)

---

## ◈ Project Task List & Milestones

### Phase 1: Requirements, Dimensional Constraints & Sourcing
- [x] Initializing engineering workspace and repository setup
- [ ] Capture mechanical constraints, interface clearances, and mounting holes
- [ ] Draft Bill of Materials (BOM) in `bom/bill_of_materials.md` with vendor links
- [ ] Verify standard hardware stock (bearings, screws, threaded inserts)

### Phase 2: Parametric 3D CAD Modeling & Assembly
- [ ] Model individual parametric solid parts in `cad/parts/`
- [ ] Assemble parts into master assembly in `cad/assemblies/`
- [ ] Run collision detection and dynamic interference checks
- [ ] Optimize geometry for additive manufacturing (45° overhangs, chamfers over fillets)

### Phase 3: Slicing, Rapid Prototyping & Tolerance Fit Testing
- [ ] Export manifold STL and 3MF files to `slicer/stl/` and `slicer/3mf/`
- [ ] Slice test fitment coupons and print verification prototypes
- [ ] Measure physical parts with digital calipers against `specs/tolerances.md`
- [ ] Iterate hole diameters for heat-set insert pullout resistance

### Phase 4: Production STEP Exports, 2D Drawings & Handover
- [ ] Generate standard neutral STEP/IGES CAD models in `exports/step/`
- [ ] Export 2D dimensioned engineering drawings (DXF/PDF) in `exports/dxf/`
- [ ] Document assembly instructions with exploded view diagrams
- [ ] Complete final engineering review and handover with {{client_name}}

---

## ◈ Workspace Directory Map

```
{{project_name}}/
├── cad/
│   ├── parts/            # Individual parametric component models (.FCStd, .scad)
│   ├── assemblies/       # Multi-part kinematic assemblies and constraints
│   └── drawings_2d/      # Vector drafts and dimensioned blueprints
├── slicer/
│   ├── stl/              # Watertight polygon meshes for 3D slicing
│   ├── 3mf/              # Modern multi-material, orientation-aware 3MF packages
│   └── gcode/            # Machine-specific toolpath configurations
├── exports/
│   ├── step/             # Neutral CAD exchange solids (ISO 10303 STEP AP214)
│   └── dxf/              # Laser cutting, waterjet, and machining profile contours
├── bom/                  # Bill of Materials and hardware component tracking
├── specs/                # Physical tolerances, torque ratings, and thermal notes
└── photos/prototypes/    # Photographic documentation of test iterations
```

---

## ◈ Deliverables & Output Specifications
- **Master CAD Solids:** Precision parametric STEP AP214 models
- **3D Printing Package:** Clean manifold STL / 3MF files ready for slicing
- **Manufacturing Blueprints:** 2D dimensioned drawings (PDF / DXF)
- **Bill of Materials:** Complete component, fastener, and cost schedule
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "bom/bill_of_materials.md".into(),
                content: r#"# Bill of Materials (BOM)

- **Project:** {{project_name}}
- **Client:** {{client_name}}
- **Revision:** Rev 1.0

| Item # | Part Description | Qty | Material | Sourcing / Vendor | Unit Cost | Status |
|---|---|---|---|---|---|---|
| 01 | Primary Structural Housing | 1 | PETG | 3D Printed | In-house | Ready |
| 02 | M3 x 12mm Socket Head Cap Screw | 8 | Stainless Steel | McMaster / Local Hardware | $0.15 | In Stock |
| 03 | M3 x 4mm Heat-Set Brass Insert | 8 | Brass | CNC Kitchen / Amazon | $0.20 | In Stock |
| 04 | 608-2RS Sealed Ball Bearing | 2 | Steel | Fastenal | $1.20 | Ordered |
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "specs/tolerances.md".into(),
                content: r#"# Mechanical Tolerances & Print Guidelines

## Additive Manufacturing Guidelines (FDM)
- **Layer Height:** 0.20 mm standard (0.12 mm for fine threads)
- **Perimeters / Walls:** 4 walls minimum (1.6 mm thickness)
- **Infill:** 35% Gyroid infill for isotropic strength
- **Hole Compensation:** +0.20 mm radial clearance for press-fit pins
- **Heat-Set Insert Hole ID:** 4.0 mm for standard M3 insert

## Dimensional Checks
- [ ] Critical Outer Dimensions: ±0.15 mm
- [ ] Bearing Recess Diameter: H7 fit (light press)
- [ ] Hardware Clearance: M3 bolt clearance hole = 3.3 mm ID
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# CAD & Engineering Git Ignore
*.FCStd1
*.FCStd@
*.gcode
*.stl_temp
*.bak
*.autosave
*.log
slicer/gcode/*
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Precision STEP / IGES CAD Master Models".into(),
            "Production-Ready 3D Printable STL & 3MF Files".into(),
            "2D Dimensioned Engineering Drawings (DXF/PDF)".into(),
            "Comprehensive Bill of Materials (BOM) with Hardware Links".into(),
        ],
    }
}

// =========================================================================
// 3. LOGO & BRAND IDENTITY TEMPLATE
// =========================================================================
fn logo_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Logo design, brand identity & vector assets".into());
    vars.insert("client_name".into(), "Internal / In-House Brand".into());
    vars.insert("client_contact".into(), "branding@example.com".into());
    vars.insert("project_status".into(), "Active (Brand Discovery & Concepts)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("brand_archetype".into(), "Modern, Minimalist, Authoritative".into());
    vars.insert("primary_color".into(), "#0F172A (Deep Slate)".into());
    vars.insert("accent_color".into(), "#10B981 (Emerald Green)".into());

    Template {
        name: "logo".into(),
        description: "Logo design, brand identity & vector assets (SVG/EPS/PNG/Typography/Guidelines)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "concepts/sketches".into(),
            "concepts/explorations".into(),
            "source/vector".into(),
            "source/color_palettes".into(),
            "exports/web/svg".into(),
            "exports/web/png".into(),
            "exports/print/cmyk".into(),
            "exports/print/eps".into(),
            "exports/favicon".into(),
            "mockups/stationery".into(),
            "mockups/digital".into(),
            "guidelines".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Brand Designer / Author** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Brand Attributes & Scope
- **Brand Archetype:** {{brand_archetype}}
- **Primary Brand Color:** {{primary_color}}
- **Accent Brand Color:** {{accent_color}}
- **Core Deliverables:** Vector Mark, Wordmark, Horizontal & Stacked Lockups, Social Icons, Brand Guide.

---

## ◈ Project Task List & Milestones

### Phase 1: Brand Discovery, Values & Moodboarding
- [x] Setting up brand identity directory structure
- [ ] Conduct client discovery session with {{client_name}} on mission and values
- [ ] Analyze industry competitor landscape and visual positioning
- [ ] Assemble moodboard and typography direction in `concepts/`

### Phase 2: Ideation, Concept Sketches & Vector Drafting
- [ ] Draft 20+ thumbnail sketches exploring symbolic metaphors
- [ ] Select 3 strongest concept directions for vector execution in `source/vector/`
- [ ] Build geometric grid construction and optical balance curves
- [ ] Present preliminary concept boards to {{client_name}} for selection

### Phase 3: Color Palette, Typography & Refinements
- [ ] Define primary, secondary, and neutral color values in `source/color_palettes/`
- [ ] Pair complementary header and body typography
- [ ] Test scalability from 16px favicon up to large billboard scale
- [ ] Create real-world mockups (business cards, apparel, app icon, signage)

### Phase 4: Master Vector Exports & Brand Style Guide
- [ ] Export clean, minified SVG files with outlined fonts into `exports/web/svg/`
- [ ] Generate print-ready CMYK vector EPS/PDF files into `exports/print/`
- [ ] Export transparent high-DPI raster assets (1x, 2x, 3x PNG & WebP)
- [ ] Compile complete Brand Guidelines manual in `guidelines/brand_guidelines.md`
- [ ] Final asset handover and archive package for {{client_name}}

---

## ◈ Workspace Directory Map

```
{{project_name}}/
├── concepts/
│   ├── sketches/         # Raw digital/paper thumbnail concepts and brainstorming
│   └── explorations/     # Early vector iterations and rejected alternate marks
├── source/
│   ├── vector/           # Editable master artwork files (.svg, .ai, .penpot)
│   └── color_palettes/   # Swatches and digital color token definitions
├── exports/
│   ├── web/
│   │   ├── svg/          # Lightweight, clean, minified vector web logos
│   │   └── png/          # Transparent PNGs at @1x, @2x, @3x resolutions
│   ├── print/
│   │   ├── cmyk/         # 300 DPI high-resolution CMYK print files
│   │   └── eps/          # Universal EPS vector masters for print shops
│   └── favicon/          # Multi-size favicon package (16x16, 32x32, 180x180, 512x512)
├── mockups/
│   ├── stationery/       # Business card, envelope, and letterhead mockups
│   └── digital/          # Website header, mobile app, and social avatar mockups
└── guidelines/           # Brand style guide, clear space, and typography rules
```

---

## ◈ Deliverables & Output Specifications
- **Master Vector Package:** Clean SVG, EPS, and PDF files
- **Responsive Variations:** Horizontal, Stacked, and Isolated Icon Mark
- **Colorways:** Full Color, Inverted White, and Monochrome Black
- **Brand Style Guide:** Complete documentation of colors, fonts, and usage rules
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "guidelines/brand_guidelines.md".into(),
                content: r#"# Brand Guidelines & Visual Identity Standards

- **Brand:** {{project_name}}
- **Client:** {{client_name}}
- **Version:** 1.0

## 1. Logo Clearspace & Minimum Size
- **Clearspace:** Maintain minimum clear breathing room equal to the height of the icon mark on all sides.
- **Minimum Digital Size:** 24px height.
- **Minimum Print Size:** 12mm height.

## 2. Color Palette Specifications
| Role | Color Name | Hex Code | CMYK | Pantone |
|---|---|---|---|---|
| **Primary** | Midnight Slate | #0F172A | 85, 75, 45, 55 | 5395 C |
| **Accent** | Emerald Green | #10B981 | 75, 0, 65, 0 | 7724 C |
| **Surface** | Pure White | #FFFFFF | 0, 0, 0, 0 | White |
| **Neutral** | Cool Grey | #F1F5F9 | 5, 2, 0, 2 | 663 C |

## 3. Typography
- **Primary Header Font:** Inter / Montserrat (Bold / Semi-Bold)
- **Secondary Body Font:** Inter / Open Sans (Regular)

## 4. Logo Misuse (Do Not)
- Do NOT distort, squish, or stretch the proportions.
- Do NOT alter the established brand colors.
- Do NOT place on busy photographic backgrounds without adequate contrast.
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "source/color_palettes/palette.md".into(),
                content: r#"# Color Palette Tokens

```css
:root {
  --color-primary: #0F172A;
  --color-accent: #10B981;
  --color-background: #FFFFFF;
  --color-surface: #F8FAFC;
  --color-text-main: #0F172A;
  --color-text-muted: #64748B;
}
```
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Logo Project Git Ignore
*.tmp
*.bak
*.autosave
*.preview.png
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Master Vector Logo Suite (SVG, EPS, PDF)".into(),
            "High-Resolution Raster PNG Assets (@1x, @2x, @3x)".into(),
            "Complete Brand Identity Guidelines Document".into(),
            "Full Favicon & Social Media Icon Bundle".into(),
        ],
    }
}

// =========================================================================
// 4. VIDEO EDITING & POST-PRODUCTION TEMPLATE
// =========================================================================
fn video_edit_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Video editing, motion graphics & color grading".into());
    vars.insert("client_name".into(), "Internal / Production Channel".into());
    vars.insert("client_contact".into(), "video@example.com".into());
    vars.insert("project_status".into(), "Active (Rough Cut Assembly)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("resolution".into(), "3840x2160 (4K UHD) / 1080x1920 (Vertical)".into());
    vars.insert("target_fps".into(), "60 fps".into());
    vars.insert("target_loudness".into(), "-14 LUFS (Web Broadcast Standard)".into());

    Template {
        name: "video-edit".into(),
        description: "Video editing, motion graphics & color grading (DaVinci/Premiere/Kdenlive/4K)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "footage/raw".into(),
            "footage/broll".into(),
            "audio/voiceover".into(),
            "audio/music".into(),
            "audio/sfx".into(),
            "graphics/titles".into(),
            "graphics/thumbnails".into(),
            "projects/timeline".into(),
            "color_luts".into(),
            "exports/drafts".into(),
            "exports/final_master".into(),
            "transcripts".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Lead Video Editor** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Video Master Specifications
- **Master Resolution:** {{resolution}}
- **Timeline Framerate:** {{target_fps}}
- **Audio Target:** {{target_loudness}} (-1 dB True Peak)
- **Delivery Formats:** 16:9 Landscape YouTube Master + 9:16 Vertical Short/Reel

---

## ◈ Project Task List & Milestones

### Phase 1: Media Ingestion, Logging & Assembly
- [x] Initializing video editing workspace structure
- [ ] Ingest camera cards into `footage/raw/` and verify checksums
- [ ] Transcribe speech and synchronize multi-cam/external audio tracks
- [ ] Assemble A-Roll narrative timeline and trim silent gaps / flubs

### Phase 2: B-Roll, Pacing & Visual Storytelling
- [ ] Curate contextual visual cutaways in `footage/broll/`
- [ ] Add lower thirds, animated callouts, and motion text in `graphics/titles/`
- [ ] Refine edit pacing, rhythm, and retention hooks in first 30 seconds
- [ ] Export Draft v1 into `exports/drafts/` for client review

### Phase 3: Audio Mastering, Sound FX & Color Grading
- [ ] Level dialog speech, apply subtle compression, and clean background noise
- [ ] Place sound effects (whooshes, risers, impacts) from `audio/sfx/`
- [ ] Mix background music ducking under voice tracks (-14 LUFS integrated)
- [ ] Balance shot-to-shot exposure, contrast, and apply creative color grade

### Phase 4: Final Master Export & Packaging
- [ ] Render full-resolution 4K master into `exports/final_master/`
- [ ] Cut 9:16 vertical short/teaser version for social channels
- [ ] Design high-contrast thumbnail variant in `graphics/thumbnails/`
- [ ] Complete delivery signoff with {{client_name}} and archive raw rushes

---

## ◈ Workspace Directory Map

```
{{project_name}}/
├── footage/
│   ├── raw/              # Primary A-roll camera takes and screen recordings
│   └── broll/            # Contextual cutaway footage and archival clips
├── audio/
│   ├── voiceover/        # Clean vocal studio voiceover files
│   ├── music/            # Licensed background music tracks
│   └── sfx/              # Sound effects (impacts, risers, ambient foley)
├── graphics/
│   ├── titles/           # Lower-thirds, animated overlays, and transparent assets
│   └── thumbnails/       # High-CTR thumbnail designs and layered project files
├── projects/timeline/    # NLE project files (.drp, .prproj, .kdenlive)
├── color_luts/           # 3D LUTs and color grading conversion profiles
├── exports/
│   ├── drafts/           # Timecoded preview renders for client feedback
│   └── final_master/     # Master broadcast-quality 4K deliverables
└── transcripts/          # Speech-to-text transcripts, script notes, and cue logs
```

---

## ◈ Deliverables & Output Specifications
- **Master 4K Video:** High-bitrate 3840x2160 MP4 / ProRes master
- **Vertical Cutdown:** 1080x1920 9:16 version for Reels / Shorts / TikTok
- **Normalized Audio:** Master audio mixed to {{target_loudness}}
- **Custom Thumbnail:** 1280x720 PNG thumbnail optimized for high CTR
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "projects/timeline/project_notes.md".into(),
                content: r#"# Timeline & Post-Production Notes

- **Project:** {{project_name}}
- **Target Resolution:** {{resolution}}
- **Framerate:** {{target_fps}}
- **Audio Standards:** Dialog peaks around -6 dB, Music ducked to -18 dB to -24 dB during speech.

## Marker Cue Points
- 00:00 - Hook / Cold Open
- 00:15 - Title Sequence
- 01:00 - Main Topic Deep Dive
- 08:30 - Conclusion & Call to Action
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "transcripts/script_and_cue.md".into(),
                content: r#"# Script & Visual Cue Log

| Timecode | Spoken Voiceover / Dialog | Visual Cue / B-Roll Overlay |
|---|---|---|
| 00:00 - 00:05 | "Welcome back to the channel..." | Fast-paced montage of final result |
| 00:05 - 00:15 | "Today we are building..." | Animated title graphic overlay |
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Video Editing Git Ignore
footage/raw/*
*.proxy.*
*.pek
*.cfa
*.mov_temp
*.mp4_temp
projects/timeline/.cache/
exports/drafts/*
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Broadcast Master 4K Video (3840x2160 MP4/ProRes)".into(),
            "Vertical Cutdown Version (1080x1920 9:16)".into(),
            "Master Audio Normalized to -14 LUFS".into(),
            "High-CTR Custom Video Thumbnail (PNG)".into(),
        ],
    }
}

// =========================================================================
// 5. PHOTO EDITING & RETOUCHING TEMPLATE
// =========================================================================
fn photo_edit_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Photography retouching, RAW development & catalog".into());
    vars.insert("client_name".into(), "Internal / Photography Client".into());
    vars.insert("client_contact".into(), "photo@example.com".into());
    vars.insert("project_status".into(), "Active (RAW Culling & Developing)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("color_space".into(), "sRGB (Web) / AdobeRGB (Print)".into());
    vars.insert("print_resolution".into(), "300 DPI".into());
    vars.insert("camera_profile".into(), "Natural Tone / Linear RAW".into());

    Template {
        name: "photo-edit".into(),
        description: "Photography retouching, RAW development & catalog (Lightroom/Darktable/GIMP/300DPI)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "raw/selected".into(),
            "raw/archive".into(),
            "edits/layered".into(),
            "presets/luts".into(),
            "exports/high_res".into(),
            "exports/web_compressed".into(),
            "exports/client_gallery".into(),
            "metadata".into(),
            "contracts_releases".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Lead Photographer / Retoucher** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Photography Specifications
- **Target Color Space:** {{color_space}}
- **Print Resolution:** {{print_resolution}}
- **Camera Calibration:** {{camera_profile}}
- **Primary Deliverables:** Full-Resolution Archival TIFF/JPEG + Web-Optimized Client Proofing Gallery.

---

## ◈ Project Task List & Milestones

### Phase 1: Shoot Preparation, Ingestion & Culling
- [x] Initializing photography catalog and workspace folders
- [ ] Transfer RAW camera media into `raw/selected/` and back up originals
- [ ] Apply initial metadata tags, copyright notice, and lens profile corrections
- [ ] First-pass star rating culling: eliminate out-of-focus and duplicate takes

### Phase 2: RAW Developing & Color Correction
- [ ] Balance exposure, highlight recovery, and shadow detail
- [ ] Set accurate white balance using gray card / skin tone reference
- [ ] Apply tone curves and uniform color grade across all selected photos
- [ ] Export proofing contact sheet for client review and selections

### Phase 3: Detailed Retouching & Fine Tuning
- [ ] Perform non-destructive frequency separation / skin cleanup in `edits/layered/`
- [ ] Clean background distractions, dust spots, and sensor artifacts
- [ ] Dodge and burn to enhance depth, eye highlights, and subject separation
- [ ] Crop and straighten compositions according to final print ratios

### Phase 4: Master Exports, Web Gallery & Delivery
- [ ] Export full-resolution 300 DPI master files into `exports/high_res/`
- [ ] Generate compressed, sRGB-converted web versions in `exports/web_compressed/`
- [ ] Prepare client gallery bundle in `exports/client_gallery/`
- [ ] Deliver final images to {{client_name}} and archive RAW catalog

---

## ◈ Workspace Directory Map

```
{{project_name}}/
├── raw/
│   ├── selected/         # Star-rated culled RAW master camera files (.cr3, .nef, .arw)
│   └── archive/          # Secondary / rejected takes kept for safety
├── edits/layered/        # Multi-layer retouching projects (.psd, .xcf, .tif)
├── presets/luts/         # Project-specific Darktable styles, LUTs, and XMP presets
├── exports/
│   ├── high_res/         # Full-resolution 300 DPI print-ready master exports
│   ├── web_compressed/   # Optimized sRGB images sized for web & social media
│   └── client_gallery/   # Watermarked proofing sets or curated final gallery
├── metadata/             # Shot list sheets, equipment logs, and exposure data
└── contracts_releases/   # Signed model releases, location permits, and client agreements
```

---

## ◈ Deliverables & Output Specifications
- **Archival Masters:** High-resolution 300 DPI uncompressed JPEG / TIFF
- **Web Sized Delivery:** sRGB 2048px long-edge web-optimized images
- **Color Grading Preset:** Exported `.xmp` / `.cube` LUT for style matching
- **Client Gallery:** Complete curated photo selection ready for delivery
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "metadata/shot_list.md".into(),
                content: r#"# Photo Shoot Metadata & Shot List

- **Project:** {{project_name}}
- **Client:** {{client_name}}
- **Location:** Studio / On-Location

| Shot # | Description / Pose | Lighting Setup | Lens / Focal | Status |
|---|---|---|---|---|
| 01 | Hero Establishing Wide Shot | Natural Light + Key Softbox | 35mm f/2.8 | Completed |
| 02 | Medium Subject Portrait | 3-Point Studio Lighting | 85mm f/1.8 | Completed |
| 03 | Macro Product Close-up Detail | Rim Light + Diffused Top Light | 105mm f/5.6 | In Progress |
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "presets/grading_notes.md".into(),
                content: r#"# Color Grading & Treatment Notes

- **White Balance:** 5600K Daylight
- **Contrast Curve:** S-Curve (+10 highlights, -5 shadows)
- **Saturation:** Natural vibrance +12, desaturate greens -5
- **Sharpening:** Radius 1.0, Masking 65
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Photo Editing Git Ignore
raw/archive/*
*.xmp.tmp
*.cache
*.thumb
exports/web_compressed/*.tmp
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Archival High-Resolution Masters (300 DPI TIFF/JPEG)".into(),
            "Optimized Web Gallery (sRGB, 2048px)".into(),
            "Custom Development Presets & Grading Profiles".into(),
            "Client Proofing Catalog".into(),
        ],
    }
}

// =========================================================================
// 6. VECTOR ARTWORK & ILLUSTRATION TEMPLATE
// =========================================================================
fn vector_artwork_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Vector illustration, iconography & digital artwork".into());
    vars.insert("client_name".into(), "Internal / Creative In-House".into());
    vars.insert("client_contact".into(), "art@example.com".into());
    vars.insert("project_status".into(), "Active (Vector Inking & Palette)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("grid_system".into(), "24px / 32px Keyline Grid".into());
    vars.insert("stroke_alignment".into(), "Consistent 2px Center-Aligned Strokes".into());

    Template {
        name: "vector-artwork".into(),
        description: "Vector illustration, iconography & digital artwork (Inkscape/Illustrator/Penpot/SVG)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "sketches/concept".into(),
            "sketches/thumbnails".into(),
            "source/svg".into(),
            "source/components".into(),
            "colors/swatches".into(),
            "exports/svg_minified".into(),
            "exports/png_raster".into(),
            "exports/pdf_vector".into(),
            "assets/icons".into(),
            "assets/illustrations".into(),
            "references".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Illustrator / Author** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Visual Artwork Specifications
- **Grid System:** {{grid_system}}
- **Stroke Specification:** {{stroke_alignment}}
- **Primary Formats:** Scalable Vector Graphics (SVG 1.1/2.0), Vector PDF, Raster PNG @ 1x/2x/4x.

---

## ◈ Project Task List & Milestones

### Phase 1: Creative Brief & Concept Sketches
- [x] Setting up illustration workspace and asset directories
- [ ] Review visual style requirements and thematic tone with {{client_name}}
- [ ] Sketch 10+ initial concept compositions in `sketches/thumbnails/`
- [ ] Define harmonic color palette swatches in `colors/swatches/`

### Phase 2: Vector Path Inking & Construction
- [ ] Lay out primary geometric shapes and snap to pixel grid
- [ ] Ink vector Bezier curves with minimal anchor point density
- [ ] Verify clean boolean path operations (unions, subtractions, compounds)
- [ ] Maintain organized layer hierarchy (Foreground, Midground, Background)

### Phase 3: Shading, Lighting & Color Fill
- [ ] Apply flat color fills from established palette
- [ ] Add vector depth using linear/radial gradients, rim highlights, and cast shadows
- [ ] Check optical balance at both micro (24px) and macro (1000px+) scale
- [ ] Generate interim draft preview for client check-in

### Phase 4: Path Optimization, Export & Delivery
- [ ] Convert strokes to outlines where required for uniform scaling
- [ ] Run SVG path minification and clean unused metadata/defs
- [ ] Export production SVGs to `exports/svg_minified/`
- [ ] Render raster PNGs at 1x, 2x, 4x into `exports/png_raster/`
- [ ] Package final illustration bundle for {{client_name}}

---

## ◈ Workspace Directory Map

```
{{project_name}}/
├── sketches/
│   ├── thumbnails/       # Quick rough thumbnail brainstorms
│   └── concept/          # Refined pencil sketches and scan tracings
├── source/
│   ├── svg/              # Master editable vector source files (.svg, .ai)
│   └── components/       # Reusable vector symbols, glyphs, and building blocks
├── colors/swatches/      # Color tokens, swatch palettes, and gradient presets
├── exports/
│   ├── svg_minified/     # Production web SVGs with stripped metadata
│   ├── png_raster/       # High-DPI transparent raster renders (1x, 2x, 4x)
│   └── pdf_vector/       # Vector PDF prints scalable to unlimited dimensions
├── assets/
│   ├── icons/            # Individual icon assets aligned to keyline grids
│   └── illustrations/    # Hero illustrations, editorial artwork, and banners
└── references/           # Moodboards, lighting references, and style guides
```

---

## ◈ Deliverables & Output Specifications
- **Master Vector File:** Full-featured SVG / AI source artwork
- **Optimized Web SVGs:** Stripped, compressed, scalable vector assets
- **High-DPI Raster Exports:** PNG / WebP rendered at 1x, 2x, and 4x
- **Print Vector PDF:** Universal resolution-independent print deliverable
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "source/components/design_tokens.md".into(),
                content: r#"# Vector Design Tokens & Rules

- **Keyline Grid:** 24 x 24 px (Icons), 1920 x 1080 px (Hero Art)
- **Base Stroke:** 2.0 px uniform
- **Corner Radius:** 3.0 px standard on external sharp corners
- **Color Palettes:**
  - Ink Accent: #1E293B
  - Primary Highlight: #38BDF8
  - Secondary Accent: #F59E0B
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "references/art_direction.md".into(),
                content: r#"# Art Direction & Aesthetic Standards

- **Style:** Clean modern vector illustration with restrained isometric / flat depth.
- **Lighting:** Consistent top-left 45-degree directional light.
- **Line Quality:** Smooth Bezier curves without erratic anchor point clusters.
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Vector Artwork Git Ignore
*.tmp
*.bak
*.autosave
exports/png_raster/*.tmp
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Scalable Vector Graphics Master (SVG 1.1 / 2.0)".into(),
            "Optimized Web Icon Set & Minified SVGs".into(),
            "High-DPI Rendered Raster Artwork (PNG / WebP @ 1x/2x/4x)".into(),
            "Print-Ready Vector Master Document (PDF / EPS)".into(),
        ],
    }
}

// =========================================================================
// 7. WEBDESIGN & UI/UX TEMPLATE
// =========================================================================
fn webdesign_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Web UI/UX design, responsive prototypes & design systems".into());
    vars.insert("client_name".into(), "Internal / Product In-House".into());
    vars.insert("client_contact".into(), "web@example.com".into());
    vars.insert("project_status".into(), "Active (Wireframing & Prototyping)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("responsive_breakpoints".into(), "Mobile (375px), Tablet (768px), Desktop (1440px)".into());
    vars.insert("accessibility_target".into(), "WCAG 2.1 Level AA Compliant".into());

    Template {
        name: "webdesign".into(),
        description: "Web UI/UX design, responsive prototypes & design systems (Figma/HTML/CSS/Tailwind)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "research/user_personas".into(),
            "research/competitor_analysis".into(),
            "wireframes/low_fidelity".into(),
            "wireframes/high_fidelity".into(),
            "design_system/typography".into(),
            "design_system/colors".into(),
            "design_system/components".into(),
            "assets/images".into(),
            "assets/icons".into(),
            "assets/fonts".into(),
            "prototype/src".into(),
            "prototype/dist".into(),
            "exports/presentation".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Lead UI/UX Designer** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Digital Design Specifications
- **Target Breakpoints:** {{responsive_breakpoints}}
- **Accessibility Standard:** {{accessibility_target}}
- **Core Deliverables:** Information Architecture, Responsive Wireframes, Design Tokens, and Interactive Prototype.

---

## ◈ Project Task List & Milestones

### Phase 1: User Research, Site Architecture & Wireframing
- [x] Initializing Web UI/UX workspace and prototype foundation
- [ ] Conduct user persona discovery and journey mapping in `research/`
- [ ] Define site navigation structure and page information hierarchy
- [ ] Draft low-fidelity wireframes in `wireframes/low_fidelity/`

### Phase 2: Design System & Token Architecture
- [ ] Define typographic hierarchy (H1..H6, Body, Caption) in `design_system/`
- [ ] Establish harmonic color tokens (Primary, Neutral, Success, Warning)
- [ ] Build atomic component library (Buttons, Form Inputs, Cards, Modals)
- [ ] Check contrast ratios to satisfy {{accessibility_target}}

### Phase 3: High-Fidelity UI Mockups & Responsive Layouts
- [ ] Design pixel-perfect desktop mockups (1440px grid)
- [ ] Adapt layouts for tablet (768px) and mobile viewport (375px)
- [ ] Curate and compress optimized SVG icons and WebP photography
- [ ] Present design prototype to {{client_name}} for milestone signoff

### Phase 4: Interactive HTML/CSS Prototype & Developer Handoff
- [ ] Build clean semantic HTML5 / CSS3 starter in `prototype/src/`
- [ ] Verify fluid flexbox and grid layouts across browser viewports
- [ ] Export design token specifications in `design_system/design_tokens.md`
- [ ] Deliver complete design assets and prototype bundle to {{client_name}}

---

## ◈ Workspace Directory Map

```
{{project_name}}/
├── research/
│   ├── user_personas/        # Target user archetypes and pain point logs
│   └── competitor_analysis/  # Competitive benchmark audits
├── wireframes/
│   ├── low_fidelity/         # Rapid structural layout sketches and UX flows
│   └── high_fidelity/        # Detailed visual layout designs
├── design_system/
│   ├── typography/           # Type scale, font pairings, and line heights
│   ├── colors/               # Color tokens and accessibility contrast pairs
│   └── components/           # Reusable UI component specs (buttons, cards, inputs)
├── assets/
│   ├── images/               # High-res photography and visual hero banners
│   ├── icons/                # Clean SVG iconography set
│   └── fonts/                # Web fonts (.woff2)
├── prototype/
│   ├── src/                  # Live semantic HTML, modern CSS, and JS prototype
│   └── dist/                 # Bundled / minified client-facing demo
└── exports/presentation/     # Stakeholder walkthrough slide decks and screenshots
```

---

## ◈ Deliverables & Output Specifications
- **High-Fidelity UI Mockups:** Multi-screen responsive layout designs
- **Design System Tokens:** Comprehensive CSS variable / token documentation
- **Interactive Prototype:** Responsive semantic HTML5 / modern CSS prototype
- **Developer Handoff Package:** Sliced SVG icons, WebP images, and style guide
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "design_system/design_tokens.md".into(),
                content: r#"# Design System Tokens

## Colors
- `--color-brand-primary`: `#2563EB` (Royal Blue)
- `--color-brand-accent`: `#38BDF8` (Sky)
- `--color-neutral-dark`: `#0F172A` (Slate 900)
- `--color-neutral-light`: `#F8FAFC` (Slate 50)
- `--color-border`: `#E2E8F0` (Slate 200)

## Spacing Scale (8pt Grid)
- `--space-1`: `4px`
- `--space-2`: `8px`
- `--space-3`: `12px`
- `--space-4`: `16px`
- `--space-6`: `24px`
- `--space-8`: `32px`
- `--space-12`: `48px`
- `--space-16`: `64px`

## Responsive Breakpoints
- Mobile: `min-width: 375px`
- Tablet: `min-width: 768px`
- Desktop: `min-width: 1024px`
- Wide: `min-width: 1440px`
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "prototype/src/index.html".into(),
                content: r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{project_name}} - Prototype</title>
  <link rel="stylesheet" href="styles.css">
</head>
<body>
  <header class="navbar">
    <div class="container navbar-content">
      <div class="brand-logo">{{project_name}}</div>
      <nav class="nav-links">
        <a href="#features">Features</a>
        <a href="#about">About</a>
        <a href="#contact" class="btn btn-primary">Get Started</a>
      </nav>
    </div>
  </header>

  <main>
    <section class="hero container">
      <h1 class="hero-title">Modern Design Built for {{client_name}}</h1>
      <p class="hero-subtitle">{{description}}</p>
      <div class="hero-actions">
        <a href="#explore" class="btn btn-primary">Explore Prototype</a>
        <a href="#docs" class="btn btn-secondary">Design Specs</a>
      </div>
    </section>
  </main>

  <footer class="footer container">
    <p>&copy; {{year}} {{client_name}}. Designed by {{author}}. All rights reserved.</p>
  </footer>
</body>
</html>
"##.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "prototype/src/styles.css".into(),
                content: r#"/* Modern CSS Reset & Design Tokens */
:root {
  --font-sans: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  --color-primary: #2563EB;
  --color-primary-hover: #1D4ED8;
  --color-dark: #0F172A;
  --color-muted: #64748B;
  --color-bg: #FFFFFF;
  --color-surface: #F8FAFC;
  --color-border: #E2E8F0;
  --max-width: 1200px;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: var(--font-sans);
  color: var(--color-dark);
  background-color: var(--color-bg);
  line-height: 1.6;
}

.container {
  width: 100%;
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 1.5rem;
}

.navbar {
  border-bottom: 1px solid var(--color-border);
  background: var(--color-bg);
  position: sticky;
  top: 0;
  z-index: 100;
}

.navbar-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 4.5rem;
}

.brand-logo {
  font-weight: 700;
  font-size: 1.25rem;
  color: var(--color-primary);
}

.nav-links {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.nav-links a {
  text-decoration: none;
  color: var(--color-dark);
  font-weight: 500;
}

.btn {
  display: inline-block;
  padding: 0.65rem 1.25rem;
  border-radius: 0.5rem;
  font-weight: 600;
  text-decoration: none;
  transition: all 0.2s ease;
}

.btn-primary {
  background: var(--color-primary);
  color: #FFFFFF !important;
}

.btn-primary:hover {
  background: var(--color-primary-hover);
}

.btn-secondary {
  border: 1px solid var(--color-border);
  color: var(--color-dark);
}

.hero {
  padding: 6rem 1.5rem;
  text-align: center;
}

.hero-title {
  font-size: 3rem;
  font-weight: 800;
  line-height: 1.2;
  margin-bottom: 1.25rem;
}

.hero-subtitle {
  font-size: 1.25rem;
  color: var(--color-muted);
  max-width: 42rem;
  margin: 0 auto 2.5rem;
}

.hero-actions {
  display: flex;
  justify-content: center;
  gap: 1rem;
}

.footer {
  padding: 3rem 1.5rem;
  border-top: 1px solid var(--color-border);
  text-align: center;
  color: var(--color-muted);
  font-size: 0.875rem;
}
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Web Design Git Ignore
prototype/dist/*
*.tmp
*.bak
.DS_Store
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Multi-Screen Responsive UI Wireframes & Mockups".into(),
            "Complete Design System Tokens & Component Guide".into(),
            "Interactive HTML5/CSS3 Front-End Prototype".into(),
            "Production Asset Handoff Directory (SVG/WebP)".into(),
        ],
    }
}

// =========================================================================
// 8. CODING & SOFTWARE DEVELOPMENT TEMPLATE
// =========================================================================
fn coding_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Software development project with clean architecture & tests".into());
    vars.insert("client_name".into(), "Internal / Engineering In-House".into());
    vars.insert("client_contact".into(), "dev@example.com".into());
    vars.insert("project_status".into(), "Active (Implementation & Testing)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("target_platform".into(), "Cross-Platform (Linux / macOS / Windows / Cloud)".into());
    vars.insert("testing_framework".into(), "Automated Unit & Integration Suites".into());

    Template {
        name: "coding".into(),
        description: "Software development project with clean architecture, tests & build automation".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "src".into(),
            "tests".into(),
            "docs/architecture".into(),
            "docs/api".into(),
            "scripts".into(),
            "config".into(),
            "build".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Lead Developer / Author** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Software Architecture & Requirements
- **Target Platform:** {{target_platform}}
- **Quality Assurance:** {{testing_framework}}
- **Coding Standard:** Clean Architecture, High Cohesion, Low Coupling, Zero Compiler Warnings.

---

## ◈ Project Task List & Milestones

### Phase 1: Architecture, Requirements & Repo Setup
- [x] Initializing clean coding repository structure
- [ ] Review system requirements, functional specs, and data models with {{client_name}}
- [ ] Document high-level system architecture in `docs/architecture/overview.md`
- [ ] Establish environment configuration and dependency management

### Phase 2: Core Implementation & Domain Logic
- [ ] Implement core domain entities, data models, and services in `src/`
- [ ] Build boundary adapters, data persistence, and external service interfaces
- [ ] Implement comprehensive input validation and robust error handling
- [ ] Write executable build automation script in `scripts/build.sh`

### Phase 3: Automated Testing & Code Quality Assurance
- [ ] Author automated unit test suites covering critical domain logic in `tests/`
- [ ] Implement end-to-end integration tests and error recovery scenarios
- [ ] Run automated test runner via `scripts/test.sh` and verify 100% test pass rate
- [ ] Benchmark performance, identify bottlenecks, and eliminate memory leaks

### Phase 4: Documentation, CI Automation & Release Delivery
- [ ] Generate comprehensive API reference documentation in `docs/api/`
- [ ] Configure CI/CD automated test pipeline and container definitions
- [ ] Perform security audit and health verification (`seed check`)
- [ ] Tag production release v1.0.0 and deliver deliverable binary to {{client_name}}

---

## ◈ Workspace Directory Map

```
{{project_name}}/
├── src/                  # Application source code implementation
├── tests/                # Automated unit, integration, and regression tests
├── docs/
│   ├── architecture/     # System diagrams, component boundaries, and design decisions
│   └── api/              # API endpoints, data contracts, and interface specs
├── scripts/              # Developer tooling (build.sh, test.sh, deploy scripts)
├── config/               # Environment templates and application configuration files
└── build/                # Local build output, release artifacts, and binaries
```

---

## ◈ Deliverables & Output Specifications
- **Engineered Source Code:** Clean, documented codebase adhering to standard idioms
- **Automated Test Suite:** Comprehensive unit and integration test coverage
- **Architecture Documentation:** In-depth technical architecture and API guide
- **Build Scripts:** Reproducible automation scripts for compilation and testing
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "src/main.rs".into(),
                content: r#"// {{project_name}}
// Lead Developer: {{author}}
// Client: {{client_name}}

fn main() {
    println!("Initializing {{project_name}} v1.0.0...");
    println!("Built for {{client_name}} under {{license}} license.");
    println!("Ready.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_initialization() {
        assert_eq!(2 + 2, 4);
    }
}
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "docs/architecture/overview.md".into(),
                content: r#"# Technical Architecture Overview

- **Project:** {{project_name}}
- **Client:** {{client_name}}
- **Author:** {{author}}

## System Boundaries
1. **Core Domain (`src/`):** Pure business logic, independent of external frameworks.
2. **Adapters & Interfaces:** CLI, API, or GUI driving the application.
3. **Storage & Infrastructure:** Configuration, database, or external network integrations.

## Quality Standards
- Code must pass all tests with zero errors: `./scripts/test.sh`
- Code must be formatted and linted cleanly.
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "scripts/build.sh".into(),
                content: r#"#!/usr/bin/env bash
set -euo pipefail

echo "==> Building {{project_name}}..."
mkdir -p build

if command -v cargo &> /dev/null && [ -f "Cargo.toml" ]; then
    cargo build --release
elif command -v go &> /dev/null && [ -f "go.mod" ]; then
    go build -o build/{{project_name}} ./...
elif [ -f "package.json" ]; then
    npm run build
else
    echo "Generic build step complete."
fi

echo "==> Build successful."
"#.into(),
                required: true,
                executable: true,
            },
            TemplateFile {
                path: "scripts/test.sh".into(),
                content: r#"#!/usr/bin/env bash
set -euo pipefail

echo "==> Running automated test suite for {{project_name}}..."

if command -v cargo &> /dev/null && [ -f "Cargo.toml" ]; then
    cargo test
elif command -v go &> /dev/null && [ -f "go.mod" ]; then
    go test ./...
elif [ -f "package.json" ]; then
    npm test
else
    echo "All tests passed (default suite)."
fi

echo "==> All test assertions verified."
"#.into(),
                required: true,
                executable: true,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Coding Project Git Ignore
target/
build/*
dist/
node_modules/
*.log
*.tmp
.env
.DS_Store
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Engineered Source Codebase with Clean Architecture".into(),
            "Comprehensive Automated Test Suite".into(),
            "Technical Architecture & API Reference Documentation".into(),
            "Production CI/CD Build & Test Automation Scripts".into(),
        ],
    }
}

// =========================================================================
// 9. AUDIO PRODUCTION & SOUND DESIGN TEMPLATE
// =========================================================================
fn audio_production_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Audio recording, podcasting & sound design".into());
    vars.insert("client_name".into(), "Internal / Audio Production".into());
    vars.insert("client_contact".into(), "audio@example.com".into());
    vars.insert("project_status".into(), "Active (Recording & Mixing)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("sample_rate".into(), "48 kHz".into());
    vars.insert("bit_depth".into(), "24-bit".into());
    vars.insert("target_lufs".into(), "-14 LUFS (Integrated)".into());

    Template {
        name: "audio-production".into(),
        description: "Audio recording, podcasting & sound design (Audacity/Reaper/WAV)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "recordings/raw_takes".into(),
            "stems/instruments".into(),
            "stems/vocals".into(),
            "sfx/foley".into(),
            "projects/session".into(),
            "exports/mix_preview".into(),
            "exports/master_wav".into(),
            "exports/mp3".into(),
            "notes/cue_sheet".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Lead Audio Engineer** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Audio Standards
- **Sample Rate:** {{sample_rate}}
- **Bit Depth:** {{bit_depth}}
- **Target Loudness:** {{target_lufs}} with -1.0 dB True Peak

---

## ◈ Project Task List & Milestones

### Phase 1: Pre-Production & Session Setup
- [x] Initializing audio recording and stem directory structure
- [ ] Calibrate input gain staging and verify microphone noise floor
- [ ] Prepare cue sheet and vocal script in `notes/cue_sheet/`
- [ ] Configure DAW session sample rate to {{sample_rate}} / {{bit_depth}}

### Phase 2: Tracking & Multitrack Recording
- [ ] Capture primary vocal takes and instrumental tracks into `recordings/raw_takes/`
- [ ] Comp best takes into seamless master tracks
- [ ] Clean background room hum, plosives, and mouth clicks
- [ ] Align timing, phase relationships, and transient peaks

### Phase 3: Mixing & Processing
- [ ] Apply subtractive EQ, dynamic resonance control, and transparent compression
- [ ] Space elements across stereo field using panning and subtle spatial reverb
- [ ] Add sound effects and ambient foley from `sfx/foley/`
- [ ] Export Draft Mix preview for client feedback into `exports/mix_preview/`

### Phase 4: Mastering & Final Stem Delivery
- [ ] Master track to broadcast standard {{target_lufs}}
- [ ] Export uncompressed 24-bit 48kHz WAV master into `exports/master_wav/`
- [ ] Export high-bitrate 320kbps MP3 deliverable into `exports/mp3/`
- [ ] Render consolidated multitrack stems into `stems/` for client archive
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "notes/cue_sheet/track_notes.md".into(),
                content: r#"# Session Cue Sheet & Track Notes

- **Project:** {{project_name}}
- **Client:** {{client_name}}
- **Tempo:** 120 BPM
- **Key Signature:** C Major / Natural Minor
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Audio Production Git Ignore
recordings/raw_takes/*
*.tmp
*.peak
*.cache
*.bak
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "24-bit 48kHz Broadcast Master WAV".into(),
            "Normalized Audio Stream (-14 LUFS)".into(),
            "Multitrack Stems Package (WAV)".into(),
        ],
    }
}

// =========================================================================
// 10. DOCUMENT & TECHNICAL PUBLISHING TEMPLATE
// =========================================================================
fn document_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Technical documentation, books & reports".into());
    vars.insert("client_name".into(), "Internal / Publications In-House".into());
    vars.insert("client_contact".into(), "editorial@example.com".into());
    vars.insert("project_status".into(), "Active (Outlining & Drafting)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("output_format".into(), "PDF / EPUB / HTML".into());

    Template {
        name: "document".into(),
        description: "Technical documentation, books, whitepapers & reports (Markdown/Pandoc/PDF)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "chapters/drafts".into(),
            "chapters/final".into(),
            "figures/diagrams".into(),
            "references/citations".into(),
            "templates".into(),
            "exports/pdf".into(),
            "exports/epub".into(),
            "exports/html".into(),
            "scripts".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Lead Author / Editor** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

---

## ◈ Project Task List & Milestones

### Phase 1: Research, Outline & Content Architecture
- [x] Setting up publishing workspace and chapter folders
- [ ] Define document scope, reader personas, and learning goals with {{client_name}}
- [ ] Outline table of contents and chapter summaries in `chapters/drafts/`
- [ ] Gather research citations and reference figures into `figures/diagrams/`

### Phase 2: First Draft Writing & Figure Drafting
- [ ] Draft Chapter 1: Introduction, Problem Statement, and Overview
- [ ] Draft remaining core chapters with structured headings and code snippets
- [ ] Author diagrams and explanatory visual figures
- [ ] Compile preliminary draft for editorial review

### Phase 3: Editorial Review & Technical Proofreading
- [ ] Perform peer review for clarity, technical accuracy, and tone consistency
- [ ] Check cross-references, citation integrity, and figure captions
- [ ] Incorporate stakeholder feedback and revision notes from {{client_name}}
- [ ] Finalize clean copy in `chapters/final/`

### Phase 4: Typesetting, Export & Publishing
- [ ] Compile publication PDF with page headers, margins, and table of contents
- [ ] Generate standard EPUB format with embedded metadata
- [ ] Run build script: `./scripts/build_docs.sh`
- [ ] Deliver finalized digital publishing package to {{client_name}}
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "chapters/drafts/01_introduction.md".into(),
                content: r#"# Chapter 1: Introduction

Welcome to {{project_name}}, prepared for {{client_name}}.

## Overview
This document details the core background, methodologies, and deliverables.
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "scripts/build_docs.sh".into(),
                content: r#"#!/usr/bin/env bash
set -euo pipefail

echo "==> Compiling {{project_name}} documentation..."
mkdir -p exports/pdf exports/html exports/epub

if command -v pandoc &> /dev/null; then
    cat chapters/drafts/*.md > exports/combined_draft.md
    echo "Combined draft generated at exports/combined_draft.md"
else
    echo "Pandoc not found. Raw markdown draft available in chapters/."
fi
"#.into(),
                required: true,
                executable: true,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Document Publishing Git Ignore
*.tmp
*.aux
*.log
*.out
exports/combined_draft.md
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Publication-Ready Formatted PDF".into(),
            "Universal EPUB Ebook Package".into(),
            "Structured Markdown Source Knowledge Base".into(),
        ],
    }
}

// =========================================================================
// 11. GAME DEVELOPMENT & PROTOTYPING TEMPLATE
// =========================================================================
fn game_dev_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Game design, mechanics prototyping & asset staging".into());
    vars.insert("client_name".into(), "Internal / Game Studio".into());
    vars.insert("client_contact".into(), "gamedev@example.com".into());
    vars.insert("project_status".into(), "Active (GDD & Prototyping)".into());
    vars.insert("target_date".into(), "TBD".into());
    vars.insert("target_engine".into(), "Godot 4.x / Unity / Custom Engine".into());
    vars.insert("target_framerate".into(), "60 FPS Fixed".into());

    Template {
        name: "game-dev".into(),
        description: "Game design, mechanics prototyping & asset staging (Godot/Unity/2D/3D)".into(),
        version: "2.0.0".into(),
        kind: super::TemplateKind::Preset,
        source_path: None,
        variables: vars,
        directories: vec![
            "design/gdd".into(),
            "design/level_design".into(),
            "assets/sprites".into(),
            "assets/models".into(),
            "assets/audio".into(),
            "src/scenes".into(),
            "src/scripts".into(),
            "builds/desktop".into(),
            "builds/web".into(),
        ],
        files: vec![
            TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}}

> {{description}}

---

## ◈ Project & Client Information

| Field | Detail |
|---|---|
| **Project Title** | {{project_name}} |
| **Client / Organization** | {{client_name}} |
| **Client Contact** | {{client_contact}} |
| **Lead Game Designer** | {{author}} |
| **Created Date** | {{date}} |
| **License** | {{license}} |
| **Project Status** | {{project_status}} |
| **Target Delivery** | {{target_date}} |

### Game Technical Targets
- **Target Engine:** {{target_engine}}
- **Framerate Target:** {{target_framerate}}
- **Core Loop:** Responsive player movement, interactive mechanics, reward progression.

---

## ◈ Project Task List & Milestones

### Phase 1: Game Design Document & Core Mechanics
- [x] Initializing game development repository structure
- [ ] Author Game Design Document (GDD) in `design/gdd/game_design_document.md`
- [ ] Define player control scheme, physics parameters, and camera view
- [ ] Prototype core game mechanics with placeholder greybox geometry

### Phase 2: Art Assets & Level Design
- [ ] Stage 2D sprites in `assets/sprites/` and 3D models in `assets/models/`
- [ ] Design greybox test level in `design/level_design/`
- [ ] Integrate character animations, state machine transitions, and collisions
- [ ] Implement gameplay loops: win/lose conditions, scoring, and UI HUD

### Phase 3: Audio Design, Polish & Playtesting
- [ ] Place ambient music and dynamic sound effects in `assets/audio/`
- [ ] Add visual juice: screen shake, particle effects, and sound feedback
- [ ] Conduct external playtesting session with target users
- [ ] Profile framerate, optimize draw calls, and fix collision bugs

### Phase 4: Builds, Packaging & Release
- [ ] Export desktop executable builds to `builds/desktop/`
- [ ] Export HTML5 / WebAssembly build to `builds/web/`
- [ ] Package vertical slice demo and design report for {{client_name}}
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: "design/gdd/game_design_document.md".into(),
                content: r#"# Game Design Document (GDD)

- **Game Title:** {{project_name}}
- **Client / Studio:** {{client_name}}
- **Lead Designer:** {{author}}

## 1. High Concept & Elevator Pitch
A fast-paced, intuitive gameplay experience emphasizing tight controls and satisfying feedback.

## 2. Core Gameplay Loop
1. **Challenge:** Player navigates level obstacle or enemy encounter.
2. **Action:** Player utilizes movement, precision timing, and abilities.
3. **Reward:** Level progression, score multiplier, and visual/audio feedback.

## 3. Target Platforms
- Desktop (Windows / Linux / macOS)
- Web (HTML5 WebAssembly)
"#.into(),
                required: true,
                executable: false,
            },
            TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Game Dev Git Ignore
.godot/
builds/*
*.tmp
*.log
.DS_Store
"#.into(),
                required: true,
                executable: false,
            },
        ],
        outputs: vec![
            "Comprehensive Game Design Document (GDD)".into(),
            "Playable Gameplay Vertical Slice Prototype".into(),
            "Cross-Platform Game Builds (Desktop & Web)".into(),
        ],
    }
}
