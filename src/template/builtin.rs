use super::Template;
use std::collections::HashMap;

pub fn get_builtin_templates() -> Vec<Template> {
    vec![
        // 1. Creative Media & Content Outputs
        yt_video_template(),
        podcast_template(),
        music_template(),
        photo_editing_template(),

        // 2. Publishing, Documents & Career Outputs
        book_template(),
        newsletter_template(),
        presentation_template(),
        research_paper_template(),
        portfolio_template(),

        // 3. Design, 3D & Physical Fabrication Outputs
        cad_maker_template(),
        blender_3d_template(),
        vector_design_template(),
        creative_coding_template(),

        // 4. Interactive & Client Deliverable Outputs
        game_template(),
        client_project_template(),

        // 5. Software & Application Deliverable Outputs
        react_template(),
        fastapi_template(),
        data_science_template(),
        docker_template(),
        docs_template(),
        rust_template(),
        python_template(),
        go_template(),
        cpp_template(),
        c_template(),
        java_template(),
        bash_template(),
        web_template(),
        typescript_template(),
        minimal_template(),
    ]
}

pub fn find_builtin_template(name: &str) -> Option<Template> {
    let lower = name.to_lowercase();
    get_builtin_templates().into_iter().find(|t| {
        t.name.to_lowercase() == lower
            || match lower.as_str() {
                // Output-focused Aliases
                "podcast" | "pod" | "audio-show" => t.name == "podcast",
                "book" | "novel" | "author" | "manuscript" | "ebook" => t.name == "book",
                "game" | "gamedev" | "indie-game" | "godot" => t.name == "game",
                "music" | "track" | "beat" | "album" | "audio" => t.name == "music",
                "presentation" | "slides" | "deck" | "marp" | "keynote" => t.name == "presentation",
                "research-paper" | "paper" | "latex" | "research" | "thesis" => t.name == "research-paper",
                "newsletter" | "article" | "blog" | "substack" => t.name == "newsletter",
                "portfolio" | "resume" | "cv" | "bio" => t.name == "portfolio",
                "yt" | "video" | "youtube" | "content" => t.name == "yt-video",
                "3d" | "blender" | "model" | "3d-model" => t.name == "3d-blender",
                "cad" | "maker" | "3dprint" | "freecad" | "openscad" => t.name == "cad-maker",
                "vector" | "design" | "inkscape" | "penpot" | "graphic" => t.name == "vector-design",
                "photo" | "gimp" | "krita" | "darktable" => t.name == "photo-editing",
                "creative" | "generative" | "shader" | "art" => t.name == "creative-coding",
                "client" | "client-work" => t.name == "client-project",

                // Software & Tool Aliases
                "react" | "frontend" | "vite" | "next" => t.name == "react",
                "fastapi" | "api" | "rest" | "backend" => t.name == "fastapi",
                "ds" | "data-science" | "ml" | "notebook" | "jupyter" => t.name == "data-science",
                "docker" | "devops" | "container" | "compose" => t.name == "docker",
                "docs" | "documentation" | "mdbook" | "mkdocs" | "wiki" => t.name == "docs",
                "rs" => t.name == "rust",
                "py" => t.name == "python",
                "golang" => t.name == "go",
                "c" => t.name == "c",
                "cpp" | "c++" | "cmake" => t.name == "cpp",
                "java" | "maven" => t.name == "java",
                "bash" | "sh" | "shell" | "script" => t.name == "bash",
                "html" | "static" => t.name == "web",
                "ts" | "node" => t.name == "typescript",
                "mini" | "manual" | "scratch" | "blank" | "custom" => t.name == "minimal",
                _ => false,
            }
    })
}

// =========================================================================
// 1. YOUTUBE & VIDEO CONTENT CREATION (Kdenlive / Shotcut / OBS / Audacity)
// =========================================================================
fn yt_video_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "YouTube video and content production project".into());
    vars.insert("fps".into(), "60".into());
    vars.insert("resolution".into(), "3840x2160 (4K UHD)".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "yt-video".into(),
        description: "YouTube & video production studio (scripts, footage, audio, thumbnails)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "4K/1080p Video (MP4)".into(),
            "Video Script (Markdown)".into(),
            "Thumbnails (WebP/PNG)".into(),
            "YouTube Metadata & Chapters".into(),
        ],
        directories: vec![
            "scripts".into(),
            "assets/footage/a-roll".into(),
            "assets/footage/b-roll".into(),
            "assets/audio/voiceover".into(),
            "assets/audio/music".into(),
            "assets/audio/sfx".into(),
            "assets/graphics/overlays".into(),
            "assets/thumbnails".into(),
            "projects/kdenlive".into(),
            "exports/drafts".into(),
            "exports/final".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "scripts/script.md".into(),
                content: r#"# Video Script: {{project_name}}

**Target Duration:** ~10-12 mins  
**Resolution:** {{resolution}} | **FPS:** {{fps}}  
**Author:** {{author}}

---

## 1. Hook (0:00 - 0:45)
- **Visual:** High-energy visual or intriguing result / b-roll teaser.
- **Audio:** Fast-paced hook question.
> "What if you could build..."

## 2. Intro & Problem (0:45 - 2:00)
- **Visual:** A-Roll talking head with overlay graphics.
- State the common challenge or creative concept.
- Quick animated title stinger.

## 3. Deep Dive / Build Process (2:00 - 8:30)
- **Step 1:** Concept & setup
- **Step 2:** Hands-on walkthrough / time-lapse
- **Step 3:** Overcoming gotchas / pro tips

## 4. Call to Action & Value Delivery (8:30 - 9:30)
- Like & Subscribe cue
- Links to resources in description / GitHub / template repo

## 5. Outro & End Screen (9:30 - 10:30)
- Final thoughts / community question
- End screen cards for related videos
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "scripts/metadata.md".into(),
                content: r#"# YouTube Video Metadata: {{project_name}}

## Title Ideas (Pick the best CTR title)
1. How I Built {{project_name}} in 24 Hours
2. Stop Doing This! The Modern Way to {{project_name}}
3. The Ultimate Open-Source Guide to {{project_name}}

## Thumbnail Concepts
- [ ] Concept A: High-contrast close-up with 3-word bold text
- [ ] Concept B: "Before vs After" split composition
- [ ] Concept C: Surprised reaction face + highlighted 3D/Design render

## Description Box
```text
In this video, we build {{project_name}} from scratch using free open-source tools!

Links & Resources:
• Project Files: [Link]
• Free Templates: [Link]

Timestamps & Chapters:
0:00 - Intro & The Problem
0:45 - The Big Idea
2:00 - Step-by-Step Walkthrough
7:15 - Mistakes to Avoid
9:30 - Final Results & Download

#creativity #opensource #artcraft #diy
```

## Tags & Keywords
`{{project_name}}`, `open source`, `tutorial`, `creative coding`, `maker`, `3d modeling`, `graphic design`
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Raw video footage & heavy audio (store locally or on NAS)
*.mp4
*.mov
*.mkv
*.avi
*.m4v
*.wav
*.flac
*.aac

# Video editor autosaves and proxies
**/.kdenlive*
**/proxy/
**/thumbs/
*.autosave

# Render exports (keep final in release/cloud)
exports/drafts/*
exports/final/*
!exports/.gitkeep
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Video Production Workspace

Created by **{{author}}**.

## Directory Structure
- `scripts/`: Video scripts, shot lists, and YouTube metadata (SEO, titles, chapters).
- `assets/footage/`: A-roll and B-roll recordings.
- `assets/audio/`: Voiceovers, background music tracks, and sound effects (SFX).
- `assets/graphics/`: Overlays, lower-thirds, animations, and thumbnail mockups.
- `projects/`: Project files for Kdenlive, DaVinci Resolve, or Shotcut.
- `exports/`: Video render drafts and final uploaded cuts.

## Recommended Open-Source Stack
- **Editing:** Kdenlive / Shotcut
- **Recording & Streaming:** OBS Studio
- **Audio Processing:** Audacity / Tenacity
- **Transcoding:** HandBrake / ffmpeg
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 2. 3D MODELING & ANIMATION (Blender / Godot / Textures / Renders)
// =========================================================================
fn blender_3d_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "3D Modeling, Texturing & Rendering Studio".into());
    vars.insert("poly_budget".into(), "50,000 tris (Realtime) / Unlimited (Cycles)".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "3d-blender".into(),
        description: "3D modeling, texturing, rendering & assets (Blender/Godot/ArmorPaint)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Game-Ready 3D Meshes (GLTF/OBJ)".into(),
            "High-Res Cycles/Eevee Renders".into(),
            "PBR Material Maps (2K/4K)".into(),
        ],
        directories: vec![
            "scenes".into(),
            "models/high_poly".into(),
            "models/low_poly".into(),
            "textures/pbr".into(),
            "textures/references".into(),
            "materials".into(),
            "renders/stills".into(),
            "renders/animation".into(),
            "exports/gltf".into(),
            "exports/fbx".into(),
            "exports/obj".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "models/specs.md".into(),
                content: r#"# 3D Asset Technical Specifications: {{project_name}}

## Scale & Dimensions
- **Units:** Metric (1 Unit = 1 Meter)
- **Origin (Pivot):** Base center / Bottom aligned (Z = 0)
- **Target Polycount Budget:** {{poly_budget}}

## PBR Textures (Metallic-Roughness Workflow)
- **Resolution:** 2048x2048 (or 4096x4096 for hero assets)
- **Maps:**
  - `BaseColor` (sRGB)
  - `Roughness` (Non-Color)
  - `Metallic` (Non-Color)
  - `Normal` (OpenGL format, Non-Color)
  - `AmbientOcclusion` / `ORM` Channel-packed (Non-Color)
  - `Emission` (sRGB, optional)

## Quality Checklist
- [ ] Transforms applied (Location: 0, Rotation: 0, Scale: 1.0)
- [ ] No N-Gons (Clean Quads and Triangles only)
- [ ] UV unwrapped with uniform texel density
- [ ] Normals oriented correctly (no flipped faces)
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Blender autosaves and backup files
*.blend1
*.blend2
*.blend@
*.blend@@

# Heavy baked caches (fluid, smoke, cloth, physics)
**/cache_fluid/
**/cache_smoke/
**/cache_physics/

# Heavy render sequences
renders/animation/*.png
renders/animation/*.exr
renders/stills/*.exr
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - 3D Modeling & Rendering Project

Created by **{{author}}**.

## Organization
- `scenes/`: Main `.blend` scene and staging environments.
- `models/`: High-poly sculpts and low-poly retopologized meshes.
- `textures/`: Reference moodboards and PBR material maps.
- `renders/`: High-resolution Cycles/Eevee image and animation outputs.
- `exports/`: Game-ready `.gltf`/`.glb`, `.fbx`, and `.obj` packages.

## Recommended Open-Source Apps
- **3D Creation Suite:** Blender
- **3D Texture Painting:** ArmorPaint / Material Maker
- **Game Engine & Asset Testing:** Godot Engine
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 3. CAD & MAKER (FreeCAD / OpenSCAD / 3D Printing / Laser Cutting / CNC)
// =========================================================================
fn cad_maker_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "CAD, 3D printing & hardware maker project".into());
    vars.insert("tolerance".into(), "+/- 0.2 mm".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "cad-maker".into(),
        description: "CAD, 3D printing, laser cutting & maker engineering (FreeCAD/OpenSCAD)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "3D Print Files (STL/3MF)".into(),
            "CAD Exchange (STEP)".into(),
            "Laser Cut Profiles (DXF/SVG)".into(),
            "Bill of Materials (BOM)".into(),
        ],
        directories: vec![
            "cad/models".into(),
            "cad/drawings".into(),
            "cad/step_exports".into(),
            "slicer/3d_print_stl".into(),
            "slicer/3mf_profiles".into(),
            "cnc_laser/dxf_svg".into(),
            "bom".into(),
            "docs".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "bom/bill_of_materials.md".into(),
                content: r#"# Bill of Materials (BOM): {{project_name}}

| Part # | Description | Quantity | Material / Source | Notes |
|---|---|---|---|---|
| P-01 | Main Chassis | 1 | PETG / PLA 3D Print | 20% infill, 4 perimeters |
| P-02 | Top Cover Plate | 1 | 3mm Acrylic Laser Cut | Matte Black |
| H-01 | M3x12mm Hex Socket Screws | 8 | Stainless Steel | Hardware store |
| H-02 | M3 Brass Heat-set Threaded Inserts | 8 | Brass (OD 4.2mm) | Heat press into chassis |
| E-01 | USB-C Power Jack Board | 1 | Electronics supplier | 5V 3A rated |

**Total Estimated Hardware Cost:** ~$15.00
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "docs/assembly_guide.md".into(),
                content: r#"# Assembly Instructions: {{project_name}}

## Required Tools
- Soldering iron (for brass heat-set inserts at ~220°C)
- 2.5mm Hex Allen wrench
- Precision tweezers

## Steps
1. **Heat-set Inserts Installation:**
   - Place M3 brass inserts onto the holes of the 3D-printed chassis.
   - Gently press with the soldering iron tip until flush with the plastic surface.
2. **Electronics Mounting:**
   - Seat the power PCB into the designated alignment slots.
3. **Closing the Enclosure:**
   - Align the laser-cut top plate and fasten using the M3x12mm screws.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# CAD backup files
*.FCBak
*.FCStd1
*.FCStd2
*~
*.swp

# Slicer generated machine code (keep STLs/3MF, ignore raw gcode)
*.gcode
*.gcode.zip
*.qoi
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Maker & CAD Engineering

Designed by **{{author}}**.

## Fabrication Specifications
- **Design Tolerance:** {{tolerance}}
- **Recommended 3D Filament:** PETG, ABS, or Tough PLA
- **Laser Cutting Thickness:** 3.0 mm (Acrylic or Birch Plywood)

## Directory Structure
- `cad/models/`: Native Parametric CAD files (FreeCAD `.FCStd`, OpenSCAD `.scad`).
- `cad/step_exports/`: Universal STEP files for CAD exchange.
- `slicer/`: 3D printing STL files and configured 3MF slicer project files.
- `cnc_laser/`: 2D vector DXF and SVG profiles for laser cutters and CNC mills.
- `bom/`: Bill of materials, fastener specs, and component sourcing links.
- `docs/`: Assembly manuals, schematics, and wiring diagrams.

## Open-Source Maker Apps
- **Parametric 3D CAD:** FreeCAD / OpenSCAD / SolveSpace
- **2D Technical Drafting:** LibreCAD / QCAD
- **Slicing:** OrcaSlicer / PrusaSlicer
- **CAM & Laser:** LaserWeb / CNCjs
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 4. VECTOR DESIGN & GRAPHIC ART (Inkscape / Penpot / Branding)
// =========================================================================
fn vector_design_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Vector graphic design, branding & iconography".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "vector-design".into(),
        description: "Vector graphics, illustration, UI & branding (Inkscape/Penpot/Krita)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Vector Artwork (SVG)".into(),
            "Visual Brand Guidelines (PDF)".into(),
            "Print CMYK Exports".into(),
            "Icon Glyphs".into(),
        ],
        directories: vec![
            "source/svg".into(),
            "source/penpot".into(),
            "assets/fonts".into(),
            "assets/colors".into(),
            "assets/icons".into(),
            "mockups".into(),
            "exports/web".into(),
            "exports/print".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "branding_guidelines.md".into(),
                content: r#"# Visual Brand Guidelines: {{project_name}}

## Color Palette
| Swatch | Name | HEX | RGB | Use Case |
|---|---|---|---|---|
| [Primary] | Primary Brand | `#10b981` | rgb(16, 185, 129) | Main CTA, Logo, Headers |
| [Accent] | Accent Blue | `#3b82f6` | rgb(59, 130, 246) | Links, Badges, Highlights |
| [Dark] | Dark Slate | `#0f172a` | rgb(15, 23, 42) | Primary Dark Background |
| [Light] | Clean White | `#f8fafc` | rgb(248, 250, 252) | Primary Text & Contrast |

## Typography
- **Headings:** Inter Display / Space Grotesk (Bold, 700)
- **Body:** Inter / Roboto (Regular, 400)
- **Code & Tech:** JetBrains Mono (Medium, 500)

## Logo Clearspace Rules
- Always preserve padding equal to the height of the brand icon around the full wordmark.
- Do not distort, rotate, or apply non-standard drop shadows to the vector mark.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Vector design backups
*.svg.202*
*.autosave.*
*~
.DS_Store

# Temporary export renders
exports/web/*.tmp
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Vector Graphic Design Workspace

Created by **{{author}}**.

## Directory Structure
- `source/svg/`: Raw Inkscape SVG master artwork.
- `source/penpot/`: UI/UX design mockups and collaborative Penpot components.
- `assets/colors/`: Color palette swatches and design tokens.
- `assets/icons/`: Scalable icon sets and symbol glyphs.
- `exports/web/`: Optimized web SVGs, WebP illustrations, and PNGs.
- `exports/print/`: Print-ready CMYK PDF and high-DPI assets.

## Open-Source Design Stack
- **Vector Illustration:** Inkscape
- **UI/UX & Design Systems:** Penpot (Open-source Figma alternative)
- **Raster Painting:** Krita / GIMP
- **Desktop Publishing:** Scribus
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 5. PHOTO EDITING & DIGITAL DARKROOM (Darktable / RawTherapee / GIMP)
// =========================================================================
fn photo_editing_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Photo editing, RAW grading & digital retouching".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "photo-editing".into(),
        description: "Photo editing, RAW developing & retouching (Darktable/GIMP/RawTherapee)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "High-Res Print JPEGs".into(),
            "Web-Compressed WebP".into(),
            "Color Grading Presets / LUTs".into(),
            "Session Shot Logs".into(),
        ],
        directories: vec![
            "raw".into(),
            "selects".into(),
            "edits/gimp_xcf".into(),
            "edits/krita_kra".into(),
            "exports/full_res".into(),
            "exports/web_compressed".into(),
            "presets".into(),
            "metadata".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "metadata/shot_list.md".into(),
                content: r#"# Photography Session & Shot List: {{project_name}}

**Photographer:** {{author}}  
**Date:** {{date}}

## Shot Checklist
- [ ] Establishing shot / Wide environmental view
- [ ] Medium close-up product / subject shot
- [ ] Detail macro texture shot
- [ ] Action / motion shot

## Camera Settings Reference
- **Color Space:** Adobe RGB / ProPhoto RGB
- **Format:** Uncompressed RAW
- **White Balance:** Daylight 5600K
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Raw camera files (keep local/cold storage, ignore from git)
*.CR2
*.CR3
*.NEF
*.ARW
*.DNG
*.RAF
*.RW2

# Editing scratch caches
*.xcf~
*.kra~
.thumbnails/
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Photo Editing Workspace

Managed by **{{author}}**.

## Workflow Structure
1. `raw/`: Imported unprocessed camera RAW files.
2. `selects/`: 5-star culled keepers chosen for final grading.
3. `edits/`: Layered non-destructive retouching files (`.xcf` for GIMP, `.kra` for Krita).
4. `presets/`: Darktable styles, LUTs, and color grading presets.
5. `exports/`: High-resolution prints and optimized web JPEGs/WebP.

## Open-Source Photography Stack
- **RAW Development:** Darktable / RawTherapee
- **Advanced Retouching:** GIMP / Krita
- **Digital Asset Management:** DigiKam
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 6. CREATIVE CODING & GENERATIVE ART (p5.js / GLSL / Python / Shaders)
// =========================================================================
fn creative_coding_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Generative art, GLSL shaders & creative algorithmic visuals".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "creative-coding".into(),
        description: "Creative coding, generative art & GLSL shaders (p5.js/Canvas/Shaders)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Generative Postcard Renders (PNG)".into(),
            "Loop Animations (MP4)".into(),
            "GLSL Shaders".into(),
        ],
        directories: vec![
            "src".into(),
            "shaders".into(),
            "palettes".into(),
            "outputs/stills".into(),
            "outputs/video".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "index.html".into(),
                content: r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{project_name}} - Creative Canvas</title>
  <script src="https://cdn.jsdelivr.net/npm/p5@1.9.0/lib/p5.min.js"></script>
  <style>
    body { margin: 0; padding: 0; background: #000; display: flex; justify-content: center; align-items: center; height: 100vh; overflow: hidden; }
    canvas { box-shadow: 0 0 30px rgba(0,255,180,0.2); }
  </style>
</head>
<body>
  <script src="src/sketch.js"></script>
</body>
</html>
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/sketch.js".into(),
                content: r##"// Generative Art Sketch: {{project_name}}
// Author: {{author}}

let seed;
const PALETTE = ["#050505", "#00f0ff", "#ff0055", "#ffe600", "#ffffff"];

function setup() {
  createCanvas(1080, 1080);
  seed = floor(random(999999));
  randomSeed(seed);
  noiseSeed(seed);
  noLoop();
  console.log("Seed:", seed);
}

function draw() {
  background(PALETTE[0]);
  noFill();
  strokeWeight(2);

  const numLines = 120;
  for (let i = 0; i < numLines; i++) {
    stroke(random(PALETTE.slice(1)));
    beginShape();
    for (let x = 0; x <= width; x += 20) {
      const n = noise(x * 0.005, i * 0.05);
      const y = map(n, 0, 1, height * 0.2, height * 0.8) + sin(x * 0.02) * 50;
      vertex(x, y);
    }
    endShape();
  }
}

function keyPressed() {
  if (key === 's' || key === 'S') {
    saveCanvas(`{{project_name}}_${seed}`, 'png');
    console.log("Canvas saved!");
  } else if (key === 'r' || key === 'R') {
    seed = floor(random(999999));
    randomSeed(seed);
    noiseSeed(seed);
    redraw();
  }
}
"##
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "shaders/basic.frag".into(),
                content: r#"#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform float u_time;

void main() {
    vec2 st = gl_FragCoord.xy / u_resolution.xy;
    vec3 color = 0.5 + 0.5 * cos(u_time + st.xyx + vec3(0.0, 2.0, 4.0));
    gl_FragColor = vec4(color, 1.0);
}
"#
                .into(),
                required: false,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Creative Coding & Generative Art

Created by **{{author}}**.

## How to Run
1. Open `index.html` in any web browser (or serve with a live server).
2. Press **`R`** to generate a new random seed.
3. Press **`S`** to export a high-resolution PNG snapshot to `outputs/stills/`.

## Stack & Tools
- **Framework:** p5.js / HTML5 Canvas / WebGL
- **Shaders:** GLSL Fragment Shaders
- **Outputs:** Generative posters, wallpapers, and loop animations
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 7. RUST CODING TEMPLATE
// =========================================================================
fn rust_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "A modern Rust application".into());
    vars.insert("version".into(), "0.1.0".into());
    vars.insert("edition".into(), "2024".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "rust".into(),
        description: "Rust binary app with CLI support and tests".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Standalone Compiled Binary".into(),
            "Unit & Integration Test Suite".into(),
            "Cargo Package".into(),
        ],
        directories: vec![
            "src".into(),
            "tests".into(),
            ".github/workflows".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "Cargo.toml".into(),
                content: r#"[package]
name = "{{project_name}}"
version = "{{version}}"
edition = "{{edition}}"
description = "{{description}}"
authors = ["{{author}}"]
license = "{{license}}"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
colored = "2.1"
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/main.rs".into(),
                content: r#"use anyhow::Result;
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(name = "{{project_name}}", version = "{{version}}", about = "{{description}}")]
struct Cli {
    #[arg(short, long, default_value = "World")]
    name: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("{} {}", "Hello,".green().bold(), cli.name);
    Ok(())
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "/target\n**/*.rs.bk\n*.log\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\n```bash\ncargo run\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 8. PYTHON CODING TEMPLATE
// =========================================================================
fn python_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Modern Python package with pyproject.toml".into());
    vars.insert("version".into(), "0.1.0".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "python".into(),
        description: "Modern Python package with pyproject.toml & pytest".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Installable Python Package (Wheel)".into(),
            "Pytest Suite".into(),
        ],
        directories: vec![
            "src/{{project_name}}".into(),
            "tests".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "pyproject.toml".into(),
                content: r#"[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "{{project_name}}"
version = "{{version}}"
description = "{{description}}"
authors = [{ name = "{{author}}" }]
readme = "README.md"
requires-python = ">=3.10"
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/{{project_name}}/__init__.py".into(),
                content: "__version__ = '{{version}}'\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/{{project_name}}/main.py".into(),
                content: "def main():\n    print('Hello from {{project_name}}!')\n\nif __name__ == '__main__':\n    main()\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "__pycache__/\n*.py[cod]\n.venv/\nvenv/\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 9. TYPESCRIPT / NODE TEMPLATE
// =========================================================================
fn typescript_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "TypeScript Node.js application".into());
    vars.insert("version".into(), "1.0.0".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "typescript".into(),
        description: "TypeScript Node.js project with Vitest".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Compiled JavaScript (dist/)".into(),
            "TypeScript Type Declarations (.d.ts)".into(),
            "Vitest Suite".into(),
        ],
        directories: vec![
            "src".into(),
            "tests".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "package.json".into(),
                content: r#"{
  "name": "{{project_name}}",
  "version": "{{version}}",
  "description": "{{description}}",
  "main": "dist/index.js",
  "scripts": {
    "build": "tsc",
    "start": "node dist/index.js"
  },
  "author": "{{author}}",
  "license": "{{license}}"
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/index.ts".into(),
                content: "console.log('Hello from {{project_name}}!');\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "node_modules/\ndist/\n*.log\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 10. GO TEMPLATE
// =========================================================================
fn go_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Go CLI application".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "go".into(),
        description: "Go CLI application with standard layout".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Standalone Go Executable Binary".into(),
            "Go Module & Tests".into(),
        ],
        directories: vec![
            "cmd/{{project_name}}".into(),
            "pkg".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "go.mod".into(),
                content: "module {{project_name}}\n\ngo 1.23\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "cmd/{{project_name}}/main.go".into(),
                content: "package main\n\nimport \"fmt\"\n\nfunc main() {\n\tfmt.Println(\"Hello from {{project_name}}!\")\n}\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "bin/\n*.exe\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 11. WEB TEMPLATE
// =========================================================================
fn web_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Clean HTML5, CSS & JS responsive starter".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "web".into(),
        description: "Clean HTML5, modern CSS & vanilla JS web starter".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Responsive Webpage (HTML/CSS/JS)".into(),
            "Web Production Assets".into(),
        ],
        directories: vec![
            "css".into(),
            "js".into(),
            "assets".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "index.html".into(),
                content: r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>{{project_name}}</title>
  <link rel="stylesheet" href="css/style.css">
</head>
<body>
  <h1>Welcome to {{project_name}}</h1>
  <script src="js/app.js"></script>
</body>
</html>
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "css/style.css".into(),
                content: "body { font-family: system-ui, sans-serif; background: #0f172a; color: #f8fafc; padding: 2rem; }\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "js/app.js".into(),
                content: "console.log('{{project_name}} loaded.');\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 12. MINIMAL TEMPLATE
// =========================================================================
fn minimal_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Minimal structure".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "minimal".into(),
        description: "Minimal structure with src, docs and README".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Clean Project Structure".into(),
        ],
        directories: vec![
            "src".into(),
            "docs".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "*.tmp\n*.bak\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 13. CLIENT PROJECT TEMPLATE
// =========================================================================
fn client_project_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Professional client work project".into());
    vars.insert("client_name".into(), "Client".into());
    vars.insert("deadline".into(), "TBD".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "client-project".into(),
        description: "Client work (briefs, assets, review drafts, invoices, deliverables)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Client Handover Package (ZIP)".into(),
            "Project Brief & Contract Specs".into(),
            "Itemized Invoices (PDF)".into(),
            "Delivery Checklist".into(),
        ],
        directories: vec![
            "brief_and_contracts".into(),
            "assets/client_provided".into(),
            "assets/working".into(),
            "drafts_for_review".into(),
            "deliverables_final".into(),
            "invoices".into(),
            "notes".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "brief_and_contracts/project_brief.md".into(),
                content: r#"# Client Project Brief: {{project_name}}

**Client:** {{client_name}}  
**Lead:** {{author}}  
**Target Deadline:** {{deadline}}  

## Project Objectives & Scope
- Deliverable 1: 
- Deliverable 2: 

## Revision Policy
- 2 rounds of revisions included. Additional revisions billed at standard hourly rate.

## Final Delivery Formats
- [ ] High-resolution master exports
- [ ] Source files (SVG, Blender, CAD, or Code repo)
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "deliverables_final/checklist.md".into(),
                content: r#"# Final Delivery Checklist: {{project_name}}

- [ ] All client deliverables verified against brief
- [ ] High-resolution / production renders exported
- [ ] Source assets packaged
- [ ] Final invoice generated in `invoices/`
- [ ] Client sign-off received
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}} (Client: {{client_name}})\n\n{{description}}\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "invoices/confidential*\n*.tmp\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 14. C APPLICATION (C11 / Makefile / Standard layout)
// =========================================================================
fn c_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "A high-performance C application".into());
    vars.insert("standard".into(), "c11".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "c".into(),
        description: "C application with C11, Makefile, and standard layout".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Compiled C Binary (bin/)".into(),
            "Unit Test Suite".into(),
        ],
        directories: vec![
            "src".into(),
            "include".into(),
            "tests".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "Makefile".into(),
                content: r#"CC = gcc
CFLAGS = -Wall -Wextra -pedantic -std={{standard}} -O2 -Iinclude
TARGET = bin/{{project_name}}
SRCS = $(wildcard src/*.c)
OBJS = $(SRCS:src/%.c=build/%.o)

all: $(TARGET)

$(TARGET): $(OBJS) | bin
	$(CC) $(CFLAGS) -o $@ $^

build/%.o: src/%.c | build
	$(CC) $(CFLAGS) -c $< -o $@

bin build:
	mkdir -p $@

test: all
	$(CC) $(CFLAGS) tests/test_main.c src/lib.c -o bin/test_suite
	./bin/test_suite

clean:
	rm -rf bin build

.PHONY: all test clean
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "include/lib.h".into(),
                content: r#"#pragma once

#define APP_VERSION "0.1.0"

int add(int a, int b);
void print_banner(const char* name);
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/main.c".into(),
                content: r#"#include <stdio.h>
#include "lib.h"

int main(void) {
    print_banner("{{project_name}}");
    printf("1 + 2 = %d\n", add(1, 2));
    return 0;
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/lib.c".into(),
                content: r#"#include <stdio.h>
#include "lib.h"

int add(int a, int b) {
    return a + b;
}

void print_banner(const char* name) {
    printf("[APP] Welcome to %s (v%s)\n", name, APP_VERSION);
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "tests/test_main.c".into(),
                content: r#"#include <stdio.h>
#include <assert.h>
#include "lib.h"

int main(void) {
    printf("[TEST] Running unit tests...\n");
    assert(add(2, 3) == 5);
    assert(add(-1, 1) == 0);
    printf("[TEST] All tests passed successfully.\n");
    return 0;
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "bin/\nbuild/\n*.o\n*.a\n*.so\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\nAuthor: {{author}}\n\n## Build & Run\n\n```bash\nmake\n./bin/{{project_name}}\nmake test\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 15. C++ APPLICATION (C++20 / CMake)
// =========================================================================
fn cpp_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Modern C++ application with CMake".into());
    vars.insert("standard".into(), "20".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "cpp".into(),
        description: "Modern C++20 project with CMake, include dirs, and tests".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Compiled C++ Executable & Library".into(),
            "CTest Test Runner".into(),
        ],
        directories: vec![
            "src".into(),
            "include/{{project_name}}".into(),
            "tests".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "CMakeLists.txt".into(),
                content: r#"cmake_minimum_required(VERSION 3.20)
project({{project_name}} VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD {{standard}})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library({{project_name}}_core
    src/core.cpp
)
target_include_directories({{project_name}}_core PUBLIC include)

add_executable({{project_name}}
    src/main.cpp
)
target_link_libraries({{project_name}} PRIVATE {{project_name}}_core)

enable_testing()
add_executable(test_suite
    tests/test_main.cpp
)
target_link_libraries(test_suite PRIVATE {{project_name}}_core)
add_test(NAME CoreTest COMMAND test_suite)
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "include/{{project_name}}/core.hpp".into(),
                content: r#"#pragma once

#include <string>
#include <string_view>

namespace {{project_name}} {

class Service {
public:
    explicit Service(std::string name);
    [[nodiscard]] std::string greet(std::string_view target) const;
    [[nodiscard]] const std::string& name() const noexcept;

private:
    std::string name_;
};

} // namespace {{project_name}}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/core.cpp".into(),
                content: r#"#include "{{project_name}}/core.hpp"
#include <utility>

namespace {{project_name}} {

Service::Service(std::string name) : name_(std::move(name)) {}

std::string Service::greet(std::string_view target) const {
    return "Hello, " + std::string(target) + "! Welcome to " + name_;
}

const std::string& Service::name() const noexcept {
    return name_;
}

} // namespace {{project_name}}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/main.cpp".into(),
                content: r#"#include <iostream>
#include "{{project_name}}/core.hpp"

int main() {
    {{project_name}}::Service app("{{project_name}}");
    std::cout << app.greet("Developer") << std::endl;
    return 0;
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "tests/test_main.cpp".into(),
                content: r#"#include <cassert>
#include <iostream>
#include "{{project_name}}/core.hpp"

int main() {
    std::cout << "[TEST] Running {{project_name}} tests...\n";
    {{project_name}}::Service s("TestApp");
    assert(s.name() == "TestApp");
    std::string msg = s.greet("World");
    assert(!msg.empty());
    std::cout << "[TEST] All tests passed!\n";
    return 0;
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "build/\ncmake-build-*/\n*.o\n*.a\n*.so\ncompile_commands.json\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\nAuthor: {{author}}\n\n## Build & Test\n\n```bash\ncmake -B build -DCMAKE_BUILD_TYPE=Release\ncmake --build build\nctest --test-dir build --output-on-failure\n./build/{{project_name}}\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 16. REACT TEMPLATE (React + TypeScript + Vite)
// =========================================================================
fn react_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Modern React with TypeScript and Vite".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "react".into(),
        description: "Modern React + TypeScript + Vite single-page application".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Production Web Bundle (dist/)".into(),
            "Optimized Static Assets".into(),
        ],
        directories: vec![
            "src/components".into(),
            "src/assets".into(),
            "public".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "package.json".into(),
                content: r#"{
  "name": "{{project_name}}",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "react": "^18.3.1",
    "react-dom": "^18.3.1"
  },
  "devDependencies": {
    "@types/react": "^18.3.3",
    "@types/react-dom": "^18.3.0",
    "@vitejs/plugin-react": "^4.3.0",
    "typescript": "^5.4.5",
    "vite": "^5.2.11"
  }
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "vite.config.ts".into(),
                content: r#"import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    open: true,
  },
});
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "tsconfig.json".into(),
                content: r#"{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"]
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "index.html".into(),
                content: r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{project_name}}</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/main.tsx".into(),
                content: r#"import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.tsx';
import './index.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/App.tsx".into(),
                content: r#"import { useState } from 'react';
import './App.css';

export default function App() {
  const [count, setCount] = useState(0);

  return (
    <div className="container">
      <header className="header">
        <h1>{{project_name}}</h1>
        <p className="subtitle">{{description}}</p>
      </header>
      <main className="card">
        <button className="button" onClick={() => setCount((c) => c + 1)}>
          Count is {count}
        </button>
        <p>Edit <code>src/App.tsx</code> and save to test HMR.</p>
      </main>
    </div>
  );
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/App.css".into(),
                content: r#".container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}
.header {
  margin-bottom: 2rem;
}
.subtitle {
  color: #64748b;
  font-size: 1.1rem;
}
.card {
  padding: 2rem;
  border-radius: 8px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
}
.button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  background-color: #0f172a;
  color: #fff;
  cursor: pointer;
  transition: opacity 0.25s;
}
.button:hover {
  opacity: 0.85;
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/index.css".into(),
                content: r#":root {
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  color: #0f172a;
  background-color: #ffffff;
}
body {
  margin: 0;
  display: flex;
  min-height: 100vh;
}
#root {
  width: 100%;
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "node_modules/\ndist/\n*.local\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\nAuthor: {{author}}\n\n## Development\n\n```bash\nnpm install\nnpm run dev\n```\n\n## Production Build\n\n```bash\nnpm run build\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 17. FASTAPI TEMPLATE (Python REST API Backend)
// =========================================================================
fn fastapi_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Production-ready FastAPI RESTful web service".into());
    vars.insert("port".into(), "8000".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "fastapi".into(),
        description: "FastAPI Python REST backend with Pydantic schemas and pytest".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "FastAPI REST API Service".into(),
            "Interactive Swagger / OpenAPI Docs".into(),
            "Production Docker Image".into(),
        ],
        directories: vec![
            "app/api/routes".into(),
            "app/core".into(),
            "app/models".into(),
            "app/schemas".into(),
            "tests".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "app/main.py".into(),
                content: r#"from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from app.api.routes import items
from app.core.config import settings

app = FastAPI(
    title=settings.PROJECT_NAME,
    version="0.1.0",
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(items.router, prefix="/api/v1/items", tags=["items"])

@app.get("/health")
def health_check():
    return {"status": "healthy", "app": settings.PROJECT_NAME}

@app.get("/")
def root():
    return {"message": f"Welcome to {settings.PROJECT_NAME}"}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "app/core/config.py".into(),
                content: r#"class Settings:
    PROJECT_NAME: str = "{{project_name}}"
    API_V1_STR: str = "/api/v1"

settings = Settings()
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "app/schemas/item.py".into(),
                content: r#"from pydantic import BaseModel
from typing import Optional

class ItemBase(BaseModel):
    title: str
    description: Optional[str] = None
    price: float

class ItemCreate(ItemBase):
    pass

class ItemResponse(ItemBase):
    id: int
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "app/api/routes/items.py".into(),
                content: r#"from fastapi import APIRouter
from typing import List
from app.schemas.item import ItemCreate, ItemResponse

router = APIRouter()

items_db: List[dict] = [
    {"id": 1, "title": "First Sample", "description": "Pre-loaded item", "price": 19.99}
]

@router.get("/", response_model=List[ItemResponse])
def get_items():
    return items_db

@router.post("/", response_model=ItemResponse)
def create_item(item: ItemCreate):
    new_item = {"id": len(items_db) + 1, **item.model_dump()}
    items_db.append(new_item)
    return new_item
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "tests/test_main.py".into(),
                content: r#"from fastapi.testclient import TestClient
from app.main import app

client = TestClient(app)

def test_root():
    response = client.get("/")
    assert response.status_code == 200
    assert "Welcome" in response.json()["message"]

def test_health():
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json()["status"] == "healthy"

def test_get_items():
    response = client.get("/api/v1/items/")
    assert response.status_code == 200
    assert isinstance(response.json(), list)
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "requirements.txt".into(),
                content: "fastapi>=0.110.0\nuvicorn[standard]>=0.28.0\npydantic>=2.6.0\npytest>=8.0.0\nhttpx>=0.27.0\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "Dockerfile".into(),
                content: r#"FROM python:3.12-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
EXPOSE {{port}}
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "{{port}}"]
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "__pycache__/\n*.py[cod]\n.venv/\nvenv/\n.env\n.pytest_cache/\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\nAuthor: {{author}}\n\n## Running Locally\n\n```bash\npython3 -m venv .venv\nsource .venv/bin/activate\npip install -r requirements.txt\nuvicorn app.main:app --reload --port {{port}}\n```\n\nInteractive Swagger API docs available at: `http://localhost:{{port}}/docs`\n\n## Tests\n\n```bash\npytest\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 18. DATA SCIENCE TEMPLATE (Python / Notebooks / Pipeline)
// =========================================================================
fn data_science_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Data analysis, exploration, and machine learning workspace".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "data-science".into(),
        description: "Data science and ML workspace (data, notebooks, src, models)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Exploratory Data Analysis Notebooks".into(),
            "Trained Machine Learning Models".into(),
            "Visual Figures & Reports".into(),
        ],
        directories: vec![
            "data/raw".into(),
            "data/processed".into(),
            "notebooks".into(),
            "src/data".into(),
            "src/models".into(),
            "models".into(),
            "reports/figures".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "notebooks/01_exploration.ipynb".into(),
                content: r##"{
 "cells": [
  {
   "cell_type": "markdown",
   "metadata": {},
   "source": [
    "# {{project_name}} - Exploratory Data Analysis\n",
    "\n",
    "Author: {{author}}"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": null,
   "metadata": {},
   "outputs": [],
   "source": [
    "import pandas as pd\n",
    "import numpy as np\n",
    "import matplotlib.pyplot as plt\n",
    "import seaborn as sns\n",
    "\n",
    "print('Data Science environment initialized.')"
   ]
  }
 ],
 "metadata": {
  "language_info": {
   "name": "python"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 2
}
"##
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/__init__.py".into(),
                content: "\"\"\"{{project_name}} package.\"\"\"\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/data/load_data.py".into(),
                content: r#"from pathlib import Path
import pandas as pd

DATA_DIR = Path(__file__).resolve().parents[2] / "data"

def load_raw_dataset(filename: str) -> pd.DataFrame:
    file_path = DATA_DIR / "raw" / filename
    if not file_path.exists():
        raise FileNotFoundError(f"Dataset not found at {file_path}")
    return pd.read_csv(file_path)
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/models/train.py".into(),
                content: r#"def train_baseline_model():
    print("Training baseline model...")
    return {"accuracy": 0.95}

if __name__ == "__main__":
    train_baseline_model()
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "requirements.txt".into(),
                content: "numpy>=1.26.0\npandas>=2.2.0\nscikit-learn>=1.4.0\nmatplotlib>=3.8.0\nseaborn>=0.13.0\njupyterlab>=4.1.0\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "data/raw/*\n!data/raw/.gitkeep\ndata/processed/*\n!data/processed/.gitkeep\nmodels/*.pkl\n__pycache__/\n.ipynb_checkpoints/\n.venv/\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\nAuthor: {{author}}\n\n## Setup\n\n```bash\npython3 -m venv .venv\nsource .venv/bin/activate\npip install -r requirements.txt\njupyter lab\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 19. JAVA TEMPLATE (Maven)
// =========================================================================
fn java_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Java application with Maven".into());
    vars.insert("group_id".into(), "com.example".into());
    vars.insert("java_version".into(), "21".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "java".into(),
        description: "Java application with Maven, standard directory layout, and JUnit 5".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Packaged JAR Application".into(),
            "JUnit 5 Test Report".into(),
        ],
        directories: vec![
            "src/main/java/com/example".into(),
            "src/main/resources".into(),
            "src/test/java/com/example".into(),
            "src/test/resources".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "pom.xml".into(),
                content: r#"<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>{{group_id}}</groupId>
    <artifactId>{{project_name}}</artifactId>
    <version>0.1.0</version>
    <packaging>jar</packaging>

    <name>{{project_name}}</name>

    <properties>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
        <maven.compiler.source>{{java_version}}</maven.compiler.source>
        <maven.compiler.target>{{java_version}}</maven.compiler.target>
        <junit.version>5.10.2</junit.version>
    </properties>

    <dependencies>
        <dependency>
            <groupId>org.junit.jupiter</groupId>
            <artifactId>junit-jupiter</artifactId>
            <version>${junit.version}</version>
            <scope>test</scope>
        </dependency>
    </dependencies>

    <build>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-compiler-plugin</artifactId>
                <version>3.13.0</version>
            </plugin>
        </plugins>
    </build>
</project>
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/main/java/com/example/App.java".into(),
                content: r#"package com.example;

public class App {
    public static void main(String[] args) {
        System.out.println("Hello from {{project_name}}!");
    }

    public int add(int a, int b) {
        return a + b;
    }
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/test/java/com/example/AppTest.java".into(),
                content: r#"package com.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class AppTest {
    @Test
    void testAdd() {
        App app = new App();
        assertEquals(5, app.add(2, 3));
    }
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "target/\n*.class\n.idea/\n*.iml\n.vscode/\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\nAuthor: {{author}}\n\n## Build & Test\n\n```bash\nmvn clean compile\nmvn test\nmvn package\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 20. BASH CLI & AUTOMATION TEMPLATE
// =========================================================================
fn bash_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "POSIX shell CLI tool and automation script suite".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "bash".into(),
        description: "POSIX shell CLI tool and automation script suite with Makefile".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Executable CLI Shell Tool".into(),
            "Shell Test Suite".into(),
            "System Installation (Makefile)".into(),
        ],
        directories: vec![
            "bin".into(),
            "lib".into(),
            "tests".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "bin/{{project_name}}".into(),
                content: r#"#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LIB_DIR="${SCRIPT_DIR}/../lib"

source "${LIB_DIR}/utils.sh"

VERSION="0.1.0"

show_help() {
    cat <<EOF
Usage: {{project_name}} [COMMAND] [OPTIONS]

{{description}}

Commands:
    run       Execute standard workflow
    version   Display version
    help      Show this help message

Options:
    -v, --verbose   Enable debug output
EOF
}

cmd_run() {
    log_info "Executing {{project_name}} standard task..."
    log_success "Task completed."
}

main() {
    local cmd="${1:-help}"
    case "$cmd" in
        run)
            cmd_run
            ;;
        version|-V|--version)
            echo "{{project_name}} v${VERSION}"
            ;;
        help|-h|--help)
            show_help
            ;;
        *)
            log_error "Unknown command: $cmd"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
"#
                .into(),
                required: true,
                executable: true,
            },
            super::TemplateFile {
                path: "lib/utils.sh".into(),
                content: r#"#!/usr/bin/env bash

log_info() {
    echo "[INFO] $*"
}

log_success() {
    echo "[OK] $*"
}

log_error() {
    echo "[ERROR] $*" >&2
}

check_cmd() {
    command -v "$1" >/dev/null 2>&1 || { log_error "Required command '$1' not found."; exit 1; }
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "tests/test_cli.sh".into(),
                content: r#"#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN="${SCRIPT_DIR}/../bin/{{project_name}}"

echo "[TEST] Running CLI tests..."
"${BIN}" version | grep -q "{{project_name}}"
"${BIN}" help | grep -q "Usage"
echo "[TEST] All CLI tests passed."
"#
                .into(),
                required: true,
                executable: true,
            },
            super::TemplateFile {
                path: "Makefile".into(),
                content: r#"BIN = bin/{{project_name}}

test:
	bash tests/test_cli.sh

install:
	mkdir -p $(HOME)/.local/bin
	cp $(BIN) $(HOME)/.local/bin/

.PHONY: test install
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "*.tmp\n*.log\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\nAuthor: {{author}}\n\n## Usage\n\n```bash\n./bin/{{project_name}} run\n./bin/{{project_name}} help\nmake test\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 21. DOCKER & DEVOPS TEMPLATE
// =========================================================================
fn docker_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Containerized service environment with compose and proxy".into());
    vars.insert("port".into(), "8080".into());
    vars.insert("service_name".into(), "app".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "docker".into(),
        description: "Dockerized microservice environment with compose and reverse proxy".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Multi-Container Docker Environment".into(),
            "Nginx Reverse Proxy Configuration".into(),
            "Deployment Scripts".into(),
        ],
        directories: vec![
            "src".into(),
            "nginx".into(),
            "scripts".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "docker-compose.yml".into(),
                content: r#"services:
  {{service_name}}:
    build:
      context: .
      dockerfile: Dockerfile
    container_name: {{project_name}}-app
    restart: unless-stopped
    ports:
      - "{{port}}:{{port}}"
    env_file:
      - .env
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:{{port}}/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  proxy:
    image: nginx:alpine
    container_name: {{project_name}}-proxy
    restart: unless-stopped
    ports:
      - "80:80"
    volumes:
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - {{service_name}}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "Dockerfile".into(),
                content: r#"FROM alpine:3.19
RUN apk add --no-cache curl ca-certificates
WORKDIR /app
COPY src/ .
EXPOSE {{port}}
USER guest
CMD ["sh", "-c", "echo '{{project_name}} running on port {{port}}' && sleep infinity"]
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "nginx/nginx.conf".into(),
                content: r#"events { worker_connections 1024; }

http {
    upstream backend {
        server {{service_name}}:{{port}};
    }

    server {
        listen 80;
        location / {
            proxy_pass http://backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
        }
    }
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".env.example".into(),
                content: "ENVIRONMENT=production\nAPP_PORT={{port}}\nLOG_LEVEL=info\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "scripts/deploy.sh".into(),
                content: r#"#!/usr/bin/env bash
set -euo pipefail
echo "[DEPLOY] Building and starting {{project_name}} stack..."
docker compose up -d --build
echo "[DEPLOY] Stack online."
"#
                .into(),
                required: true,
                executable: true,
            },
            super::TemplateFile {
                path: ".dockerignore".into(),
                content: ".git/\n.env\n*.tmp\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}}\n\n{{description}}\n\nAuthor: {{author}}\n\n## Deployment\n\n```bash\ncp .env.example .env\ndocker compose up -d --build\ndocker compose ps\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 22. DOCS TEMPLATE (Markdown & MkDocs Site)
// =========================================================================
fn docs_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("description".into(), "Technical documentation and knowledge base site".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "docs".into(),
        description: "Documentation and knowledge base site with Markdown structure".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Static HTML Documentation Site (site/)".into(),
            "Searchable Technical Guides".into(),
        ],
        directories: vec![
            "docs/getting-started".into(),
            "docs/guides".into(),
            "docs/reference".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "mkdocs.yml".into(),
                content: r#"site_name: {{project_name}}
site_description: {{description}}
site_author: {{author}}

theme:
  name: material
  palette:
    primary: indigo
    accent: cyan

nav:
  - Home: index.md
  - Getting Started:
      - Quickstart: getting-started/quickstart.md
  - Guides:
      - Architecture: guides/architecture.md
  - Reference:
      - API: reference/api.md
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "docs/index.md".into(),
                content: r#"# Welcome to {{project_name}}

{{description}}

Managed by **{{author}}**.

## Overview
This documentation site provides setup guides, system architecture details, and reference materials.

## Quick Links
- [Quickstart Guide](getting-started/quickstart.md)
- [Architecture](guides/architecture.md)
- [API Reference](reference/api.md)
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "docs/getting-started/quickstart.md".into(),
                content: r#"# Quickstart Guide

Get up and running with {{project_name}} in under 5 minutes.

## Prerequisites
- Git
- Standard tooling

## Installation
Follow the project installation steps in the main README.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "docs/guides/architecture.md".into(),
                content: r#"# Architecture Overview

This document outlines the core components and design patterns used in {{project_name}}.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "docs/reference/api.md".into(),
                content: r#"# API Reference

Detailed technical reference for {{project_name}}.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "site/\n.cache/\n*.tmp\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: "# {{project_name}} Documentation\n\n{{description}}\n\n## Preview Docs Locally\n\n```bash\npip install mkdocs-material\nmkdocs serve\n```\n\nBuild static HTML site:\n\n```bash\nmkdocs build\n```\n".into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 23. PODCAST SERIES & EPISODES (Audio / Show Notes / Transcripts / Art)
// =========================================================================
fn podcast_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("show_title".into(), "The Creator Broadcast".into());
    vars.insert("frequency".into(), "Weekly".into());
    vars.insert("description".into(), "Podcast audio series and episode production studio".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "podcast".into(),
        description: "Podcast series & episode studio (audio, show notes, transcripts, artwork)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Mastered Episodes (MP3/WAV)".into(),
            "Show Notes & Episode Guides (Markdown)".into(),
            "Timestamps & Full Transcripts (TXT)".into(),
            "Cover Artwork (3000x3000px RGB)".into(),
        ],
        directories: vec![
            "episodes/ep01/raw_audio".into(),
            "episodes/ep01/edit_session".into(),
            "episodes/ep01/exports".into(),
            "assets/music_stems".into(),
            "assets/sound_effects".into(),
            "assets/artwork/cover".into(),
            "assets/artwork/social".into(),
            "transcripts".into(),
            "show_notes".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "show_notes/template.md".into(),
                content: r#"# Episode [Number]: [Episode Title]

**Host:** {{author}}  
**Show:** {{show_title}}  
**Release Date:** [YYYY-MM-DD]  
**Audio Runtime:** [00:00:00]  

---

## Episode Summary
A brief 2-3 sentence teaser capturing the core topic and value delivery for listeners.

## Key Takeaways
- Key Insight 1: [Core takeaway]
- Key Insight 2: [Actionable advice]
- Key Insight 3: [Common pitfall to avoid]

## Timestamps & Chapters
- 00:00 - Introduction & Cold Open
- 02:15 - Guest Introduction & Background
- 08:30 - Deep Dive: The Core Framework
- 24:10 - Listener Questions & Live Troubleshooting
- 38:45 - Closing Thoughts & Next Week's Teaser

## Mentioned Links & Resources
- Website: [Link]
- Recommended Tool: [Link]
- Guest Socials: [Link]

## Sponsors
- **Sponsor A:** [Offer Code & URL]
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "episodes/ep01/plan.md".into(),
                content: r#"# Episode 01: Pre-Production & Recording Plan

**Episode Topic:** Introduction to {{show_title}}  
**Target Runtime:** 30 minutes  

## Interview & Discussion Flow
1. **Hook:** State the bold premise of the podcast.
2. **Context:** Why this podcast exists now.
3. **Core Topic:** 3 big ideas explored in this first edition.
4. **Call to Action:** Subscribe on Apple Podcasts & Spotify.

## Audio Engineering Setup
- Microphone: Cardioid dynamic (e.g. Shure SM7B / Rode PodMic)
- Target Gain: Peaks at -12 dBFS during speech
- Room: Sound absorption panels active
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "assets/artwork/cover/specs.md".into(),
                content: r#"# Podcast Cover Art Technical Specifications

In accordance with Apple Podcasts and Spotify directory standards:

- **Dimensions:** 3000 x 3000 pixels (Square 1:1)
- **Minimum Resolution:** 1400 x 1400 pixels
- **Resolution:** 72 DPI
- **Color Profile:** sRGB
- **File Format:** JPEG or PNG
- **Typography:** Ensure title remains legible down to 55x55 pixel thumbnail size on mobile lock screens.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Heavy uncompressed raw recordings
*.wav
*.aiff
*.flac

# DAW session backups and undo caches
**/.reaper-bak*
**/Autosave*
*.pk
*.aup3-shm
*.aup3-wal

# Rendered episode exports
episodes/*/exports/*.mp3
episodes/*/exports/*.wav
!episodes/*/exports/.gitkeep
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Podcast Production Studio

**Show Title:** {{show_title}}  
**Host:** {{author}}  
**Frequency:** {{frequency}}  

## Expected Outputs
- `episodes/*/exports/`: Mastered MP3 (128kbps mono or 192kbps stereo, -16 LUFS).
- `show_notes/`: Formatted markdown show notes for RSS feed and episode description.
- `transcripts/`: Full-text SEO transcripts for blog and accessibility.
- `assets/artwork/`: Square 3000x3000px cover art and 1080x1920px social reel clips.

## Audio Mastering Targets
- **Integrated Loudness:** -16 LUFS (+/- 1.0 LUFS) for stereo, -19 LUFS for mono.
- **True Peak:** Maximum -1.0 dBTP to avoid clipping on transcoding.
- **Noise Floor:** Below -60 dBFS during silence.
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 24. BOOK & NOVEL (eBook / EPUB / PDF Manuscript / Chapters)
// =========================================================================
fn book_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("subtitle".into(), "From Vision to Reality".into());
    vars.insert("genre".into(), "Non-Fiction / Practical Craft".into());
    vars.insert("target_words".into(), "50,000".into());
    vars.insert("description".into(), "Book, novel & eBook manuscript with chapters and EPUB/PDF export".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "book".into(),
        description: "Book, novel & eBook manuscript with outline, chapters and EPUB/PDF export".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "eBook Publication (EPUB)".into(),
            "Print-Ready Manuscript (PDF)".into(),
            "Formatted Chapters (Markdown)".into(),
            "Character & Worldbuilding Bible".into(),
        ],
        directories: vec![
            "manuscript/front_matter".into(),
            "manuscript/chapters".into(),
            "manuscript/back_matter".into(),
            "planning/outline".into(),
            "planning/characters".into(),
            "planning/worldbuilding".into(),
            "research/notes".into(),
            "exports/epub".into(),
            "exports/pdf".into(),
            "cover".into(),
            "scripts".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "planning/outline/book_outline.md".into(),
                content: r#"# Book Outline: {{project_name}}

**Subtitle:** {{subtitle}}  
**Author:** {{author}}  
**Genre:** {{genre}}  
**Target Word Count:** {{target_words}} words  

---

## Central Premise & Core Message
State the single governing concept that guides every chapter of this manuscript.

## Act Structure & Chapter Arc

### Part I: The Foundation
- **Chapter 1: The Inciting Reality** — Establish the status quo and common obstacles.
- **Chapter 2: Shifting the Paradigm** — Unveiling the underlying mechanics.
- **Chapter 3: Setting the Ground Rules** — Essential tools, mindset, and environment.

### Part II: The Craft & Execution
- **Chapter 4: The Core Method** — Step-by-step breakdown of the primary process.
- **Chapter 5: Navigating Turbulence** — Overcoming failure modes and blockades.
- **Chapter 6: Case Studies & Field Notes** — Real-world demonstrations and benchmarks.

### Part III: Mastery & Expansion
- **Chapter 7: Systems of Leverage** — Scaling the results sustainably.
- **Chapter 8: The Long Horizon** — Maintaining quality over years.
- **Chapter 9: The Final Synthesis** — Bringing every thread together into action.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "manuscript/front_matter/title_page.md".into(),
                content: r#"# {{project_name}}

### {{subtitle}}

**{{author}}**

---

*First Edition*  
*Copyright © 2026 by {{author}}. All rights reserved.*  
*Published via Independent Publishing.*
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "manuscript/chapters/ch01_the_beginning.md".into(),
                content: r#"# Chapter 1: The Inciting Reality

Every significant undertaking begins not with a blueprint, but with friction. 
Before the structure rises, before the words align, there is an initial moment 
where the existing way of doing things proves insufficient.

Write the compelling opening hook here. Introduce the stakes and ground the reader in the immediate narrative.

## The Problem Beneath the Surface

Explore the hidden friction that most people overlook. Why does standard advice fail? 
Provide concrete examples and personal observation.

## The Turning Point

Transition from the status quo to the new path forward. End the chapter with a bridge 
that compels the reader to turn the page.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "scripts/build_ebook.sh".into(),
                content: r#"#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR}/.."

echo "[BUILD] Compiling manuscript into EPUB and PDF..."
mkdir -p "${ROOT_DIR}/exports/epub" "${ROOT_DIR}/exports/pdf"

if command -v pandoc >/dev/null 2>&1; then
    pandoc "${ROOT_DIR}/manuscript/front_matter/"*.md \
           "${ROOT_DIR}/manuscript/chapters/"*.md \
           "${ROOT_DIR}/manuscript/back_matter/"*.md \
           -o "${ROOT_DIR}/exports/epub/{{project_name}}.epub" \
           --metadata title="{{project_name}}" \
           --metadata author="{{author}}" \
           --toc --toc-depth=2
    echo "[OK] Generated: exports/epub/{{project_name}}.epub"
else
    echo "[WARN] pandoc is not installed. Install pandoc to compile EPUB directly."
fi
"#
                .into(),
                required: true,
                executable: true,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Build exports
exports/epub/*.epub
exports/pdf/*.pdf
!exports/epub/.gitkeep
!exports/pdf/.gitkeep

# Word processor backups
*.bak
*.tmp
*~
.DS_Store
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Book & Novel Manuscript Studio

**Author:** {{author}}  
**Subtitle:** {{subtitle}}  
**Target:** {{target_words}} words  

## Expected Deliverables
- `exports/epub/{{project_name}}.epub`: Validated eBook for Kindle, Apple Books, and Kobo.
- `exports/pdf/{{project_name}}.pdf`: Formatted interior manuscript for print-on-demand (KDP/IngramSpark).
- `cover/`: Print cover wrap (Front, Spine, Back) at 300 DPI CMYK.

## Compiling Manuscript
Run the included build script:
```bash
./scripts/build_ebook.sh
```
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 25. INDIE GAME PROJECT (GDD / Sprites / Audio / Builds)
// =========================================================================
fn game_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("genre".into(), "2D Action Platformer / Adventure".into());
    vars.insert("target_platform".into(), "PC / Web (WASM)".into());
    vars.insert("engine".into(), "Godot / Raylib".into());
    vars.insert("description".into(), "Indie game project with Game Design Document, assets, and builds".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "game".into(),
        description: "Indie game project with Game Design Document (GDD), assets, levels, and builds".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Playable Game Distribution Builds".into(),
            "Game Design Document (GDD)".into(),
            "Sprite Sheets & 3D Models".into(),
            "Sound Effects & Background Music".into(),
        ],
        directories: vec![
            "design/gdd".into(),
            "design/level_design".into(),
            "assets/sprites".into(),
            "assets/models".into(),
            "assets/audio/music".into(),
            "assets/audio/sfx".into(),
            "assets/fonts".into(),
            "src/scenes".into(),
            "src/scripts".into(),
            "builds/desktop".into(),
            "builds/web".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "design/gdd/game_design_document.md".into(),
                content: r#"# Game Design Document (GDD): {{project_name}}

**Genre:** {{genre}}  
**Target Platform:** {{target_platform}}  
**Engine:** {{engine}}  
**Lead Designer:** {{author}}  

---

## 1. Executive Summary & Core Hook
- **Elevator Pitch:** A concise 1-sentence description that explains why this game is unique.
- **Core Hook:** The primary mechanic or emotional tension that makes the player keep playing.
- **Target Audience:** Fans of [comparable titles].

## 2. Core Gameplay Loop
```text
[Explore / Traverse] ---> [Encounter Challenge / Puzzle] ---> [Acquire Upgrade / Resource]
        ^                                                               |
        └────────────────────── [Progress to Next Area] ────────────────┘
```

## 3. Player Mechanics & Controls
- **Movement:** Running, responsive jump physics with coyote time and jump buffer.
- **Abilities:** Primary action, secondary action, dash/evade.
- **Health & Resources:** Hit points, stamina, inventory system.

## 4. Art & Aesthetic Direction
- Visual Style: Pixel art, stylized low-poly, or hand-drawn vector.
- Palette: Defined color script corresponding to game world mood.

## 5. Audio Direction
- Music: Dynamic adaptive soundtrack with exploration and combat state layers.
- SFX: Punchy, tactile feedback on every jump, hit, and item pickup.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "design/level_design/level_01_concept.md".into(),
                content: r#"# Level 01: Concept & Pacing Blueprint

**Setting:** The Abandoned Outpost  
**Primary Objective:** Restore power to the main elevator.  

## Flow & Beats
1. **Tutorial Zone (Safe):** Introduce basic left/right movement and camera look.
2. **First Obstacle:** Single-jump gap with zero fail penalty.
3. **Escalation:** Moving hazard requiring timed movement.
4. **Reward:** First key item / battery pack.
5. **Climax:** Mini-encounter testing the acquired mechanics.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Engine import caches and temporary data
.godot/
.import/
*.tmp
*.log

# Distribution build binaries
builds/desktop/*
builds/web/*
!builds/desktop/.gitkeep
!builds/web/.gitkeep

# System files
.DS_Store
Thumbs.db
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Indie Game Project

**Lead:** {{author}}  
**Engine:** {{engine}}  
**Target Platform:** {{target_platform}}  

## Expected Outputs
- `builds/desktop/`: Standalone playable executables (Windows `.exe`, Linux binary, macOS `.app`).
- `builds/web/`: WebAssembly / HTML5 package for browser distribution (Itch.io / Game Jolt).
- `design/gdd/`: Complete Game Design Document and mechanical references.
- `assets/`: Original sprite sheets, 3D meshes, SFX, and soundtrack.
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 26. MUSIC PRODUCTION (DAW / Stems / MIDI / Masters)
// =========================================================================
fn music_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("bpm".into(), "120".into());
    vars.insert("key_signature".into(), "C Minor".into());
    vars.insert("genre".into(), "Electronic / Synthwave / Cinematic".into());
    vars.insert("description".into(), "Music production studio with DAW sessions, stems, MIDI, and mastering".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "music".into(),
        description: "Music production studio (DAW tracks, stems, MIDI, mastering, album artwork)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Mastered Stereo Audio (WAV 24-bit 48kHz)".into(),
            "Individual Track Stems (Drums, Bass, Synths, Vocals)".into(),
            "MIDI Arrangements & Chord Progressions".into(),
            "Album Artwork & Release Mockups".into(),
        ],
        directories: vec![
            "projects/daw_sessions".into(),
            "recordings/vocals".into(),
            "recordings/instruments".into(),
            "stems/drums".into(),
            "stems/bass".into(),
            "stems/melodic".into(),
            "midi".into(),
            "masters/lossless_wav".into(),
            "masters/compressed_mp3".into(),
            "artwork/album_cover".into(),
            "artwork/promotional".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "track_notes.md".into(),
                content: r#"# Track Production Sheet: {{project_name}}

**Producer / Artist:** {{author}}  
**Tempo:** {{bpm}} BPM  
**Key Signature:** {{key_signature}}  
**Genre:** {{genre}}  

---

## Arrangement Map
```text
[Intro]      (Bars 1-8)   : Atmospheric pad + filtered arpeggio
[Verse 1]    (Bars 9-24)  : Introduce kick, bassline, and lead vocal / motif
[Pre-Chorus] (Bars 25-32) : Rising snare roll, sub drop, filter sweep
[Chorus 1]   (Bars 33-48) : Full energy drop, wide supersaws, active hi-hats
[Verse 2]    (Bars 49-64) : Stripped-back groove with new counter-melody
[Chorus 2]   (Bars 65-80) : Chorus layer with added vocal chops & harmony
[Bridge]     (Bars 81-96) : Half-time breakdown, harmonic modulation
[Outro]      (Bars 97-112): Fade out elements, solo synth tail
```

## Sound Design & Plugin Choices
- Synth Leads: Serum / Vital (Saw oscillator with unison detune)
- Bass: Analog style sub-bass + saturated mid-range layer
- Drums: Custom 909-style punchy acoustic-hybrid transient
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "mastering_checklist.md".into(),
                content: r#"# Audio Mastering Quality Checklist

- [ ] Target Integrated Loudness: -14 LUFS (+/- 0.5 LUFS) for streaming (Spotify, Apple Music).
- [ ] True Peak Ceiling: Set to -1.0 dBTP to eliminate inter-sample distortion.
- [ ] Low-End Mono Check: Frequencies below 100 Hz mono-summed for vinyl/club subwoofers.
- [ ] High-Frequency Balance: Harsh sibilance tamed at 6-8 kHz using dynamic EQ.
- [ ] Dynamic Range (DR): Adequate micro-dynamics preserved (minimum DR8-DR10).
- [ ] Fades: Clean 5ms zero-crossing fades on start and release tails.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"# Heavy raw multi-track audio recordings
recordings/vocals/*.wav
recordings/instruments/*.wav
projects/daw_sessions/*.bak
projects/daw_sessions/*.als~
*.pkf
*.peak

# Temporary stems
stems/*/*.wav
!stems/*/.gitkeep
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Music Production Studio

**Artist:** {{author}}  
**BPM:** {{bpm}} | **Key:** {{key_signature}} | **Genre:** {{genre}}  

## Expected Outputs
- `masters/lossless_wav/`: 24-bit 48kHz stereo master (uncompressed master).
- `masters/compressed_mp3/`: 320kbps MP3 with full ID3 metadata (Artist, Title, Artwork).
- `stems/`: Isolated grouped WAV stems for remixing, sync licensing, and live performance.
- `artwork/album_cover/`: 3000x3000px high-resolution album cover art.
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 27. PRESENTATION & SLIDE DECK (Marp / PDF / HTML)
// =========================================================================
fn presentation_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("event_name".into(), "Keynote 2026".into());
    vars.insert("theme".into(), "gaia".into());
    vars.insert("description".into(), "Slide deck and keynote presentation with Marp and PDF/HTML export".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "presentation".into(),
        description: "Slide deck and keynote presentation with Markdown, Marp and PDF/HTML export".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Slide Deck Presentation (PDF)".into(),
            "Interactive Presentation (HTML)".into(),
            "Speaker Notes & Audience Handouts".into(),
        ],
        directories: vec![
            "slides".into(),
            "assets/images".into(),
            "assets/diagrams".into(),
            "assets/theme".into(),
            "handouts".into(),
            "exports/pdf".into(),
            "exports/html".into(),
            "scripts".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "slides/presentation.md".into(),
                content: r#"---
marp: true
theme: {{theme}}
paginate: true
header: "{{project_name}} | {{event_name}}"
footer: "{{author}} • 2026"
style: |
  section {
    font-family: 'Inter', system-ui, sans-serif;
  }
---

# {{project_name}}
### Delivering High-Impact Results

**{{author}}**  
*{{event_name}}*

<!-- note
Speaker note:
Welcome the audience warmly. State your name, role, and the single big idea of today's talk.
-->

---

## The Challenge

- **Friction Point 1:** Legacy systems add unnecessary overhead.
- **Friction Point 2:** Complex workflows decrease creative momentum.
- **Friction Point 3:** Delivery timelines are compressed.

> "Simplicity is prerequisite for reliability." — Edsger W. Dijkstra

<!-- note
Speaker note:
Pause after reading the quote. Ask the audience if they have felt this friction in their projects.
-->

---

## The Solution: A Structured Pipeline

```text
[Input Specification] ---> [Autonomous Engine] ---> [Verified Deliverable]
```

1. **Standardization:** Consistent, reproducible directory architecture.
2. **Speed:** Scaffolding in sub-second execution time.
3. **Quality:** Built-in health audits and automated checks.

---

## Tangible Results & Takeaways

| Metric | Previous Baseline | With {{project_name}} |
|---|---|---|
| Setup Time | 45 minutes | < 5 seconds |
| Structure Errors | ~18% | 0% |
| Project Velocity | 1x | 4.5x |

---

# Thank You!

### Questions & Discussion

- **Email:** contact@example.com
- **Repository:** https://github.com/example/{{project_name}}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "handouts/summary.md".into(),
                content: r#"# Executive Handout: {{project_name}}

**Presented by:** {{author}} at {{event_name}}  

## Key Takeaways
1. **Core Insight:** Structured workflows eliminate cognitive friction.
2. **Action Item:** Standardize project foundations before writing code or producing media.
3. **Contact:** Reach out at contact@example.com for further discussion.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "scripts/export_slides.sh".into(),
                content: r#"#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR}/.."

echo "[EXPORT] Exporting presentation slides..."
mkdir -p "${ROOT_DIR}/exports/pdf" "${ROOT_DIR}/exports/html"

if command -v npx >/dev/null 2>&1; then
    npx @marp-team/marp-cli "${ROOT_DIR}/slides/presentation.md" -o "${ROOT_DIR}/exports/pdf/{{project_name}}.pdf" --pdf
    npx @marp-team/marp-cli "${ROOT_DIR}/slides/presentation.md" -o "${ROOT_DIR}/exports/html/{{project_name}}.html"
    echo "[OK] Generated: exports/pdf/{{project_name}}.pdf and exports/html/{{project_name}}.html"
else
    echo "[INFO] Install Node.js & Marp CLI to compile slide deck to PDF/HTML."
fi
"#
                .into(),
                required: true,
                executable: true,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"exports/pdf/*.pdf
exports/html/*.html
!exports/pdf/.gitkeep
!exports/html/.gitkeep
.DS_Store
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Slide Deck & Presentation Studio

**Speaker:** {{author}}  
**Event:** {{event_name}}  

## Expected Outputs
- `exports/pdf/{{project_name}}.pdf`: Standalone presentation slides ready for projection.
- `exports/html/{{project_name}}.html`: Interactive web-based slides with speaker notes view.
- `handouts/summary.md`: Printable attendee summary sheet.

## Compiling Slides
Run the export script:
```bash
./scripts/export_slides.sh
```
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 28. RESEARCH PAPER & ACADEMIC REPORT (LaTeX / BibTeX / PDF)
// =========================================================================
fn research_paper_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("target_venue".into(), "IEEE / ACM / arXiv".into());
    vars.insert("description".into(), "Academic research paper and technical report with LaTeX and BibTeX".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "research-paper".into(),
        description: "Academic research paper & technical report with LaTeX, BibTeX and PDF output".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Camera-Ready Publication (PDF)".into(),
            "BibTeX Citation Database (.bib)".into(),
            "Publication Figures (Vector SVG/PDF)".into(),
        ],
        directories: vec![
            "src/sections".into(),
            "src/figures".into(),
            "bibliography".into(),
            "data/tables".into(),
            "output".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "src/main.tex".into(),
                content: r#"\documentclass[10pt,journal,compsoc]{IEEEtran}

\usepackage{amsmath,amssymb,amsfonts}
\usepackage{algorithmic}
\usepackage{graphicx}
\usepackage{textcomp}
\usepackage{xcolor}
\usepackage{booktabs}
\usepackage{hyperref}

\begin{document}

\title{{{project_name}}: A Framework for Reliable System Architecture}

\author{{{author}}%
\thanks{Manuscript prepared for {{target_venue}}.}}

\IEEEtitleabstractindextext{%
\begin{abstract}
Modern computational and creative systems suffer from fragmentation and maintenance decay. 
In this work, we present {{project_name}}, a formal methodology for deterministic project 
scaffolding, health verification, and long-term workspace archiving. Our experimental 
evaluation demonstrates an order-of-magnitude reduction in setup overhead while 
guaranteeing structural integrity across diverse domains.
\end{abstract}

\begin{IEEEkeywords}
System architecture, reproducible workflows, project management, automated verification.
\end{IEEEkeywords}}

\maketitle
\IEEEdisplaynontitleabstractindextext
\IEEEpeerreviewmaketitle

\input{sections/01_introduction}
\input{sections/02_related_work}
\input{sections/03_methodology}
\input{sections/04_evaluation}
\input{sections/05_conclusion}

\bibliographystyle{IEEEtran}
\bibliography{../bibliography/references}

\end{document}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/sections/01_introduction.tex".into(),
                content: r#"\section{Introduction}
\IEEEPARstart{T}{he} initial configuration of any complex project establishes constraints 
that persist across its entire operational lifetime. In traditional development environments, 
directory structures and dependencies are frequently established in an ad-hoc manner, 
resulting in cognitive friction, redundant assets, and maintenance debt.

In this paper, we make the following contributions:
\begin{itemize}
    \item We define a formal grammar for guaranteed workspace hierarchies.
    \item We design an automated health inspection engine that audits structural integrity.
    \item We validate our approach across real-world workloads, showing zero structural deviations.
\end{itemize}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/sections/02_related_work.tex".into(),
                content: r#"\section{Related Work}
Prior investigations into software development hygiene have explored boilerplate generators 
and build automation tools. However, existing utilities treat workspace layout as an ephemeral 
side effect rather than a first-class architectural invariant.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/sections/03_methodology.tex".into(),
                content: r#"\section{Methodology}
Our architecture separates concerns into four orthogonal domains:
personal exploratory tasks, client deliverables with contract isolation, shared master asset 
libraries, and cold-storage yearly archiving.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/sections/04_evaluation.tex".into(),
                content: r#"\section{Evaluation}
We evaluated {{project_name}} against standard baseline workflows. Experimental results indicate 
a 94\% reduction in project initialization latency and complete elimination of namespace collisions.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "src/sections/05_conclusion.tex".into(),
                content: r#"\section{Conclusion}
We have presented {{project_name}}, a rigorous framework for workspace lifecycle maintenance. 
Future research will investigate continuous automated telemetry and predictive archiving.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "bibliography/references.bib".into(),
                content: r#"@article{dijkstra1972humble,
  author    = {Edsger W. Dijkstra},
  title     = {The Humble Programmer},
  journal   = {Communications of the ACM},
  volume    = {15},
  number    = {10},
  pages     = {859--866},
  year      = {1972}
}
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "Makefile".into(),
                content: r#"MAIN = main
SRC_DIR = src
OUT_DIR = output

all: $(OUT_DIR)/$(MAIN).pdf

$(OUT_DIR)/$(MAIN).pdf: $(SRC_DIR)/$(MAIN).tex
	mkdir -p $(OUT_DIR)
	cd $(SRC_DIR) && pdflatex -output-directory=../$(OUT_DIR) $(MAIN).tex
	cd $(OUT_DIR) && bibtex $(MAIN) || true
	cd $(SRC_DIR) && pdflatex -output-directory=../$(OUT_DIR) $(MAIN).tex
	cd $(SRC_DIR) && pdflatex -output-directory=../$(OUT_DIR) $(MAIN).tex

clean:
	rm -rf $(OUT_DIR)/*.aux $(OUT_DIR)/*.bbl $(OUT_DIR)/*.blg $(OUT_DIR)/*.log $(OUT_DIR)/*.out $(OUT_DIR)/*.toc

.PHONY: all clean
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: r#"output/
*.aux
*.bbl
*.blg
*.log
*.out
*.toc
.DS_Store
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Academic Research Paper

**Author:** {{author}}  
**Target Venue:** {{target_venue}}  

## Expected Outputs
- `output/main.pdf`: Camera-ready IEEE/ACM formatted research paper PDF.
- `bibliography/references.bib`: Standard BibTeX citations database.
- `src/figures/`: High-resolution vector figures and data plots.

## Compilation
```bash
make
```
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 29. NEWSLETTER & EDITORIAL STUDIO (Issues / Essays / Archive)
// =========================================================================
fn newsletter_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("newsletter_name".into(), "The Creative Signal".into());
    vars.insert("frequency".into(), "Weekly".into());
    vars.insert("description".into(), "Newsletter publication and long-form article writing studio".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "newsletter".into(),
        description: "Newsletter publication & content studio (issues, essays, archive, subscriber assets)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "Published Newsletter Editions (Markdown/HTML)".into(),
            "Issue Header Banners & Visual Graphics".into(),
            "Social Teaser Snippets".into(),
            "Curated Resource Links Archive".into(),
        ],
        directories: vec![
            "drafts".into(),
            "issues/2026".into(),
            "assets/headers".into(),
            "assets/infographics".into(),
            "archive".into(),
            "templates".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "templates/issue_template.md".into(),
                content: r#"# Issue #[Number]: [Compelling Subject Line Here]

*Published by {{author}} • {{newsletter_name}}*

---

## The Big Idea
[Hook the reader in the first 2 sentences. Describe a counter-intuitive observation or lesson learned this week.]

[Deep-dive analysis: 3-4 paragraphs breaking down the core thesis with practical examples.]

---

## Tools of the Week
1. **[Tool Name]:** What it is and why it saved 5 hours of work this week.
2. **[Resource Name]:** A standout reference, repo, or creative asset.

---

## Question for the Community
[A thought-provoking question to encourage subscriber replies.]

*Enjoyed this issue? Forward it to a fellow creator.*
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "issues/2026/issue_001.md".into(),
                content: r#"# Issue #001: The Power of Finished Projects

*Welcome to the inaugural edition of {{newsletter_name}}.*

Most creative ventures do not fail from a lack of talent or ambition. They fail from an inability to bring artifacts across the finish line into concrete reality.

In this first edition, we examine the structural disciplines that allow independent creators and engineers to consistently ship polished deliverables.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "assets/**/*.psd\nassets/**/*.xcf\n*.tmp\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - {{newsletter_name}} Studio

**Publisher:** {{author}}  
**Cadence:** {{frequency}}  

## Expected Outputs
- `issues/`: Ready-to-send Markdown and HTML editions formatted for Substack, Beehiiv, or Mailchimp.
- `assets/headers/`: 1200x630px social cards and issue header illustrations.
- `drafts/`: Incubating ideas and reader feedback archive.
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}

// =========================================================================
// 30. PORTFOLIO & RESUME (ATS Resume / Case Studies / Press Kit)
// =========================================================================
fn portfolio_template() -> Template {
    let mut vars = HashMap::new();
    vars.insert("role_title".into(), "Staff Engineer & Creative Builder".into());
    vars.insert("location".into(), "Remote / Global".into());
    vars.insert("contact_email".into(), "hello@example.com".into());
    vars.insert("description".into(), "Professional portfolio, ATS resume, and case studies package".into());

    Template {
        kind: super::TemplateKind::Preset,
        source_path: None,
        name: "portfolio".into(),
        description: "Professional portfolio & career package (ATS resume, case studies, press kit)".into(),
        version: "1.0.0".into(),
        variables: vars,
        outputs: vec![
            "ATS-Compliant Resume (PDF & Markdown)".into(),
            "Deep-Dive Project Case Studies".into(),
            "Media Press Kit & Headshots".into(),
            "Interactive Portfolio Site Structure".into(),
        ],
        directories: vec![
            "resume".into(),
            "case_studies/project_01".into(),
            "case_studies/project_02".into(),
            "press_kit/headshots".into(),
            "press_kit/bio".into(),
            "credentials/certifications".into(),
            "site".into(),
        ],
        files: vec![
            super::TemplateFile {
                path: "resume/resume.md".into(),
                content: r#"# {{author}}
**{{role_title}}**  
Email: {{contact_email}} | Location: {{location}} | Portfolio: https://example.com

---

## Executive Summary
Results-driven builder with expertise in architecting resilient systems, leading cross-functional creative initiatives, and delivering production-grade software and media products.

## Core Competencies
- **Architecture & Engineering:** Rust, Modern Systems, Distributed Workflows, Performance Optimization.
- **Creative & Technical Direction:** Scaffolding, Media Pipelines, Automated Auditing, User Interface Design.
- **Leadership & Communication:** Technical Writing, Mentorship, Cross-Discipline Collaboration.

## Professional Experience

### Senior Lead Builder | Tech Company (2023 - Present)
- Architected mission-critical tools reducing project onboarding latency by 90%.
- Authored standard operating procedures adopted across 12 product teams.

### Software Engineer | Studio (2020 - 2023)
- Engineered scalable media processing pipelines serving over 100k daily active users.
- Automated code health inspections increasing release stability.

## Key Projects
- **{{project_name}}:** Flagship open-source tooling suite with automated workspace lifecycle management.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "case_studies/project_01/case_study.md".into(),
                content: r#"# Case Study: Building a High-Performance Workspace Manager

## Problem
Developers and digital creators frequently lost hours to fragmented folder structures, lost deliverables, and manual project setup.

## Solution & Architecture
Designed a zero-dependency Rust CLI and full-screen TUI application that enforces strict directory invariants and offers sub-second project seeding.

## Quantitative Results
- **Velocity:** Initialization time dropped from minutes to sub-second.
- **Adoption:** Over 500 active workspace initializations in the first quarter.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "press_kit/bio/speaker_bio.md".into(),
                content: r#"# Speaker & Press Bios: {{author}}

### 50-Word Bio
{{author}} is a {{role_title}} specializing in high-performance developer tooling and creative systems. Known for building resilient open-source tools that streamline digital craftsmanship.

### 150-Word Bio
{{author}} is a {{role_title}} based in {{location}}. With over a decade of experience bridging software engineering and creative media production, {{author}} designs tools that empower independent builders to produce polished deliverables without cognitive friction.
"#
                .into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: ".gitignore".into(),
                content: "resume/*.pdf\npress_kit/headshots/*.raw\n*.tmp\n.DS_Store\n".into(),
                required: true,
                executable: false,
            },
            super::TemplateFile {
                path: "README.md".into(),
                content: r#"# {{project_name}} - Career Portfolio & Resume Package

**Professional:** {{author}}  
**Role:** {{role_title}}  
**Contact:** {{contact_email}}  

## Expected Outputs
- `resume/resume.md`: Clean ATS-optimized markdown resume ready to export to PDF.
- `case_studies/`: In-depth problem/solution case studies demonstrating tangible business impact.
- `press_kit/`: Speaker bios (short/long) and media assets for conference speaking and interviews.
"#
                .into(),
                required: true,
                executable: false,
            },
        ],
    }
}
