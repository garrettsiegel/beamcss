import { renderToString } from 'react-dom/server'
import { StaticRouter } from 'react-router-dom'
import App from './App'
import { routesSeo } from './seo'

/** Render a route to static HTML for prerendering. */
export function render(url: string): string {
  return renderToString(
    <StaticRouter location={url}>
      <App />
    </StaticRouter>,
  )
}

/** Route list consumed by prerender.mjs. */
export const routes = routesSeo
