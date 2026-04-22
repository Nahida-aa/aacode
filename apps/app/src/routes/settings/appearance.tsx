import { createFileRoute } from '@tanstack/solid-router'

export const Route = createFileRoute('/settings/appearance')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/settings/appearance"!</div>
}
