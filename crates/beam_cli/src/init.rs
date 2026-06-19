use std::{fs, path::Path};

pub(crate) fn init(args: &[String]) -> Result<(), String> {
    let mut template = InitTemplate::Basic;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--template" | "-t" => {
                index += 1;
                template = match args.get(index).map(String::as_str) {
                    Some("basic") => InitTemplate::Basic,
                    Some("vite") => InitTemplate::Vite,
                    Some(value) => return Err(format!("unknown init template `{value}`")),
                    None => return Err("--template expects `basic` or `vite`".to_owned()),
                };
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            value => return Err(format!("unexpected init argument `{value}`")),
        }
        index += 1;
    }

    match template {
        InitTemplate::Basic => init_basic()?,
        InitTemplate::Vite => init_vite()?,
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum InitTemplate {
    Basic,
    Vite,
}

pub(crate) fn init_basic() -> Result<(), String> {
    write_new_file(Path::new("beam.config.ts"), default_config())?;
    println!("created beam.config.ts");
    Ok(())
}

pub(crate) fn init_vite() -> Result<(), String> {
    write_new_file(Path::new("beam.config.ts"), vite_starter_config())?;
    write_new_file(Path::new("package.json"), vite_package_json())?;
    write_new_file(Path::new("index.html"), vite_index_html())?;
    write_new_file(Path::new("src/main.tsx"), vite_main_tsx())?;
    write_new_file(Path::new("src/App.tsx"), vite_app_tsx())?;
    write_new_file(Path::new("vite.config.ts"), vite_config_ts())?;
    println!("created Beam Vite starter");
    Ok(())
}

fn write_new_file(path: &Path, contents: &str) -> Result<(), String> {
    if path.exists() {
        return Err(format!("{} already exists", path.display()));
    }

    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }

    fs::write(path, contents)
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

pub(crate) fn default_config() -> &'static str {
    r##"import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    // Beam ships no color palette — define your own tokens here, then use them
    // as bg-<name>, text-<name>, border-<name>. Modify inline with the color algebra:
    //   bg-brand/50  (alpha)   bg-brand+10 / bg-brand-10  (lighten / darken)
    //   bg-brand~ink (mix two tokens)
    color: {},
    spacing: { card: "1rem", section: "2rem" },
    radius: { sm: "4px", md: "8px", lg: "16px", full: "9999px" },
    text: { sm: "14px", base: "16px", lg: "20px", xl: "28px" },
    font: { ui: "Inter, system-ui, sans-serif", mono: "ui-monospace, monospace" },
    screens: {
      tablet: "48rem",
      desktop: "64rem",
      wide: "80rem",
      "mobile-landscape": "(max-width:47.999rem) and (orientation:landscape)",
    },
  },
  // Point these at two of your color tokens to paint the page <body>:
  // background: "page",
  // foreground: "ink",
})
"##
}

fn vite_starter_config() -> &'static str {
    r##"import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    color: {
      page: "#0b0b0c",
      surface: "#16161a",
      ink: "#e8e8ea",
      muted: "#6b7280",
      brand: "#3b82f6",
      "on-brand": "#ffffff",
    },
    spacing: { card: "1rem", section: "2rem" },
    radius: { sm: "4px", md: "8px", lg: "16px", full: "9999px" },
    text: { sm: "14px", base: "16px", lg: "20px", xl: "28px" },
    font: { ui: "Inter, system-ui, sans-serif", mono: "ui-monospace, monospace" },
    screens: {
      tablet: "48rem",
      desktop: "64rem",
      wide: "80rem",
      "mobile-landscape": "(max-width:47.999rem) and (orientation:landscape)",
    },
  },
  background: "page",
  foreground: "ink",
})
"##
}

fn vite_package_json() -> &'static str {
    r#"{
  "name": "beamcss-app",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "@beamcss/vite": "latest",
    "@vitejs/plugin-react": "latest",
    "beamcss": "latest",
    "react": "latest",
    "react-dom": "latest",
    "vite": "latest"
  },
  "devDependencies": {
    "@types/react": "latest",
    "@types/react-dom": "latest",
    "typescript": "latest"
  }
}
"#
}

fn vite_index_html() -> &'static str {
    r#"<div id="root"></div>
<script type="module" src="/src/main.tsx"></script>
"#
}

fn vite_main_tsx() -> &'static str {
    r#"import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import App from "./App"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
"#
}

fn vite_app_tsx() -> &'static str {
    r#"export default function App() {
  return (
    <main className="grid place-center min-h-screen bg-page text-ink font-ui">
      <section className="flex direction-column align-center gap-4 p-6 bg-surface rounded-lg hover:(bg-surface+8 scale-105)">
        <p className="text-sm text-brand">beamcss</p>
        <h1 className="text-xl">Focused styles, zero scatter.</h1>
        <p className="text-base text-muted max-w-[36rem]">
          Edit <code>src/App.tsx</code> and write Beam classes inline.
        </p>
      </section>
    </main>
  )
}
"#
}

fn vite_config_ts() -> &'static str {
    r#"import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"
import { beamcss } from "@beamcss/vite"

export default defineConfig({
  plugins: [
    react(),
    beamcss({
      config: "beam.config.ts",
      content: ["index.html", "src"],
    }),
  ],
})
"#
}
