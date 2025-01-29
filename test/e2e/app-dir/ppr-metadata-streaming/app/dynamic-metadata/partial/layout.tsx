import { Suspense } from 'react'

export default function Layout({ children }) {
  return (
    <div>
      <h2>Suspenseful Layout</h2>
      <Suspense fallback={<div>Loading...</div>}>{children}</Suspense>
    </div>
  )
}
