export default function App() {
  return (
    <main className="place min-h-screen bg-base fg-fg font-ui">
      <section className="stack(center gap-4) p-6 bg-surface round-lg hover:(bg-[color-mix(in_srgb,var(--color-surface),white_8%)] scale-105)">
        <p className="text-sm fg-accent">beamcss</p>
        <h1 className="text-xl">Focused styles, zero scatter.</h1>
        <p className="text-base fg-muted max-w-[36rem]">
          Edit <code>src/App.tsx</code> and write Beam classes inline.
        </p>
      </section>
    </main>
  )
}
