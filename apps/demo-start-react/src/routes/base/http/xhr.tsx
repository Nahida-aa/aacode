import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/base/http/xhr')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/base/http/xhr"!</div>
}
