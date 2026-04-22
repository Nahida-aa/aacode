import { createFileRoute } from '@tanstack/solid-router';
import { createSignal } from 'solid-js';
import { ModeToggle } from '#/components/app/ModeToggle.tsx';

export const Route = createFileRoute('/settings/')({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<div>
			<div class="bg-muted rounded-md p-3">
				<div class="mb-3 flex items-center justify-between">
					<div>
						<h3>主题</h3>
						<p>选择主题</p>
					</div>
					<ModeToggle />
				</div>
			</div>
		</div>
	);
}
