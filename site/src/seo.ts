import { docsPages } from './docs/docsNav'

export const SITE_URL = 'https://beamcss.dev'
export const SITE_NAME = 'Beam CSS'
export const OG_IMAGE_URL = `${SITE_URL}/og.png`

type JsonLd = Record<string, unknown>

export interface RouteSeo {
  path: string
  title: string
  description: string
  canonicalUrl: string
  structuredData: JsonLd
}

const docDescriptions: Record<string, string> = {
  introduction:
    'What Beam CSS is and why variant grouping removes the wall of classes from utility CSS.',
  installation:
    'Install Beam CSS with Vite, PostCSS, or the CLI — a working setup in under a minute.',
  configuration:
    'Configure beam.config.ts: tokens, shortcuts, recipes, presets, and tree-shakeable utility modules.',
  syntax:
    'The Beam class-string grammar: variant grouping, utility grouping, values, dynamic CSS variables, and color algebra.',
  utilities:
    'The Beam utilities reference: spacing, sizing, colors, typography, layout, border, grid, and effects.',
  tooling:
    'Beam CLI, Vite and PostCSS plugins, the native Node binding, and agent-native surfaces (MCP, llms.txt).',
}

type RouteSeoInput = Omit<RouteSeo, 'canonicalUrl' | 'structuredData'>

const HOME: RouteSeoInput = {
  path: '/',
  title: 'Beam CSS — Atomic CSS without the class wall',
  description:
    "Tailwind's authoring speed, without the wall of classes. A utility-first CSS framework with a Rust compiler — group repeated variants and ship zero-runtime atomic CSS.",
}

const WEBSITE_ID = `${SITE_URL}/#website`
const SOFTWARE_ID = `${SITE_URL}/#software`
const IMAGE_ID = `${OG_IMAGE_URL}#image`

export function normalizePathname(pathname: string): string {
  const [pathOnly] = pathname.split(/[?#]/)
  const withSlash = pathOnly.startsWith('/') ? pathOnly : `/${pathOnly}`
  const normalized = withSlash.length > 1 ? withSlash.replace(/\/+$/, '') : '/'

  return normalized === '/docs' ? '/docs/introduction' : normalized
}

export function canonicalUrlForPath(pathname: string): string {
  const normalized = normalizePathname(pathname)
  return normalized === '/' ? `${SITE_URL}/` : `${SITE_URL}${normalized}`
}

export function structuredDataForRoute(
  route: Pick<RouteSeo, 'title' | 'description' | 'canonicalUrl'>,
): JsonLd {
  return {
    '@context': 'https://schema.org',
    '@graph': [
      {
        '@type': 'WebSite',
        '@id': WEBSITE_ID,
        url: `${SITE_URL}/`,
        name: SITE_NAME,
        description: 'A utility-first CSS framework with a Rust compiler.',
      },
      {
        '@type': 'SoftwareApplication',
        '@id': SOFTWARE_ID,
        name: SITE_NAME,
        url: `${SITE_URL}/`,
        applicationCategory: 'DeveloperApplication',
        operatingSystem: 'Cross-platform',
        offers: { '@type': 'Offer', price: '0', priceCurrency: 'USD' },
        description:
          "Tailwind's authoring speed, without the wall of classes. A utility-first CSS framework with a Rust compiler — variant grouping, atomic output, zero runtime.",
        softwareVersion: '0.1.1',
        license: 'https://github.com/garrettsiegel/beamcss/blob/main/LICENSE',
        sameAs: [
          'https://github.com/garrettsiegel/beamcss',
          'https://www.npmjs.com/package/beamcss',
        ],
      },
      {
        '@type': 'ImageObject',
        '@id': IMAGE_ID,
        url: OG_IMAGE_URL,
        width: 1200,
        height: 630,
        caption: 'Beam CSS social preview',
      },
      {
        '@type': 'WebPage',
        '@id': `${route.canonicalUrl}#webpage`,
        url: route.canonicalUrl,
        name: route.title,
        description: route.description,
        isPartOf: { '@id': WEBSITE_ID },
        about: { '@id': SOFTWARE_ID },
        primaryImageOfPage: { '@id': IMAGE_ID },
        inLanguage: 'en',
      },
    ],
  }
}

function makeRouteSeo(route: RouteSeoInput): RouteSeo {
  const path = normalizePathname(route.path)
  const canonicalUrl = canonicalUrlForPath(path)
  const withCanonical = { ...route, path, canonicalUrl }

  return {
    ...withCanonical,
    structuredData: structuredDataForRoute(withCanonical),
  }
}

/** Every prerendered route, with its title + description. */
export const routesSeo: RouteSeo[] = [
  makeRouteSeo(HOME),
  ...docsPages.map((p) => ({
    path: `/docs/${p.slug}`,
    title: `${p.title} — ${SITE_NAME}`,
    description: docDescriptions[p.slug] ?? HOME.description,
  })).map(makeRouteSeo),
]

/** Look up SEO for a path (used for client-side title updates). */
export function seoForPath(pathname: string): RouteSeo {
  const normalized = normalizePathname(pathname)
  return routesSeo.find((r) => r.path === normalized) ?? routesSeo[0]
}

function setContent(selector: string, value: string): void {
  document.querySelector(selector)?.setAttribute('content', value)
}

function setHref(selector: string, value: string): void {
  document.querySelector(selector)?.setAttribute('href', value)
}

export function applyRouteSeo(pathname: string): void {
  const seo = seoForPath(pathname)

  document.title = seo.title
  setContent('meta[name="description"]', seo.description)
  setHref('link[rel="canonical"]', seo.canonicalUrl)
  setContent('meta[property="og:url"]', seo.canonicalUrl)
  setContent('meta[property="og:title"]', seo.title)
  setContent('meta[property="og:description"]', seo.description)
  setContent('meta[name="twitter:title"]', seo.title)
  setContent('meta[name="twitter:description"]', seo.description)

  const jsonLd = document.querySelector<HTMLScriptElement>('script[data-seo-jsonld]')
  if (jsonLd) {
    jsonLd.textContent = JSON.stringify(seo.structuredData)
  }
}
