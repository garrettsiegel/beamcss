import { useEffect } from 'react'
import { Nav } from '../components/Nav'
import { Hero } from '../components/Hero'
import { Comparison } from '../components/Comparison'
import { Capabilities } from '../components/Capabilities'
import { GetStarted } from '../components/GetStarted'
import { Footer } from '../components/Footer'
import { applyRouteSeo } from '../seo'

export function Landing() {
  useEffect(() => {
    applyRouteSeo('/')
  }, [])

  return (
    <div className="bg-base text-fg">
      <Nav />
      <Hero />
      <Comparison />
      <Capabilities />
      <GetStarted />
      <Footer />
    </div>
  )
}
