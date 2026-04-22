import { createFileRoute } from '@tanstack/solid-router'

export const Route = createFileRoute(
  '/workspaces/$projectId/sessions/$sessionId',
)({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/workspaces/$projectId/sessions/$sessionId"!</div>
}
