const principles = [
  'Composable tokens',
  'Small, readable utilities',
  'Framework-friendly output',
]

function App() {
  return (
    <main className="app-shell">
      <section className="intro">
        <p className="eyebrow">beamcss</p>
        <h1>A front end for shaping a sharper CSS framework.</h1>
        <p className="lede">
          Use this Vite React app as the public face for docs, demos, release notes,
          and early experiments while the package API takes form.
        </p>
        <div className="actions" aria-label="Primary actions">
          <a href="https://vercel.com/docs" target="_blank" rel="noreferrer">
            Vercel docs
          </a>
          <a href="https://vite.dev/guide/" target="_blank" rel="noreferrer">
            Vite guide
          </a>
        </div>
      </section>

      <section className="panel" aria-label="Project focus">
        <div>
          <p className="panel-label">Current focus</p>
          <h2>Build the framework, then let the site prove it.</h2>
        </div>
        <ul>
          {principles.map((item) => (
            <li key={item}>{item}</li>
          ))}
        </ul>
      </section>
    </main>
  )
}

export default App
