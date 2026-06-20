import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const dist = path.join(__dirname, 'dist')

const template = fs.readFileSync(path.join(dist, 'index.html'), 'utf8')
const { render, routes } = await import(path.join(dist, 'server', 'entry-server.js'))

const esc = (s) =>
  s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')

const jsonForScript = (value) => JSON.stringify(value, null, 2).replace(/</g, '\\u003c')

function applyHead(html, { title, description, canonicalUrl, structuredData }) {
  const t = esc(title)
  const d = esc(description)
  return html
    .replace(/<title>[\s\S]*?<\/title>/, `<title>${t}</title>`)
    .replace(/(<meta name="description" content=")[^"]*(")/, `$1${d}$2`)
    .replace(/(<link rel="canonical" href=")[^"]*(")/, `$1${canonicalUrl}$2`)
    .replace(/(<meta property="og:url" content=")[^"]*(")/, `$1${canonicalUrl}$2`)
    .replace(/(<meta property="og:title" content=")[^"]*(")/, `$1${t}$2`)
    .replace(/(<meta property="og:description" content=")[^"]*(")/, `$1${d}$2`)
    .replace(/(<meta name="twitter:title" content=")[^"]*(")/, `$1${t}$2`)
    .replace(/(<meta name="twitter:description" content=")[^"]*(")/, `$1${d}$2`)
    .replace(
      /(<script type="application\/ld\+json" data-seo-jsonld>\s*)[\s\S]*?(\s*<\/script>)/,
      `$1${jsonForScript(structuredData)}$2`,
    )
}

for (const r of routes) {
  const appHtml = render(r.path)
  let html = template.replace('<div id="root"></div>', `<div id="root">${appHtml}</div>`)
  html = applyHead(html, r)
  const outDir = r.path === '/' ? dist : path.join(dist, r.path)
  fs.mkdirSync(outDir, { recursive: true })
  fs.writeFileSync(path.join(outDir, 'index.html'), html)

  if (r.path !== '/') {
    const flatFile = path.join(dist, `${r.path.replace(/^\//, '')}.html`)
    fs.mkdirSync(path.dirname(flatFile), { recursive: true })
    fs.writeFileSync(flatFile, html)
  }

  console.log('prerendered', r.path)
}

// Sitemap, generated from the same route list (stays in sync)
const sitemap =
  `<?xml version="1.0" encoding="UTF-8"?>\n` +
  `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n` +
  routes.map((r) => `  <url><loc>${r.canonicalUrl}</loc></url>`).join('\n') +
  `\n</urlset>\n`
fs.writeFileSync(path.join(dist, 'sitemap.xml'), sitemap)
console.log('wrote sitemap.xml')

// Drop the SSR bundle from the deployable output
fs.rmSync(path.join(dist, 'server'), { recursive: true, force: true })
console.log('prerender complete')
