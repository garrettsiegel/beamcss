import { Navigate, Route, Routes } from 'react-router-dom'
import { Landing } from './pages/Landing'
import { DocsLayout } from './docs/DocsLayout'
import { docsPages, DEFAULT_DOC } from './docs/docsNav'

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Landing />} />
      <Route path="/docs" element={<Navigate to={`/docs/${DEFAULT_DOC}`} replace />} />
      <Route path="/docs/*" element={<DocsLayout />}>
        {docsPages.map(({ slug, Component }) => (
          <Route key={slug} path={slug} element={<Component />} />
        ))}
      </Route>
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  )
}
