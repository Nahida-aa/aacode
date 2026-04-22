import { createFileRoute, Outlet } from '@tanstack/solid-router';
import { Palette, Settings } from 'lucide-solid';
import { Button } from '#/components/ui/button.tsx';
import { NoStyleLink } from '#/components/uix/link.tsx';
import { ModalOnRoute } from '#/components/uix/modal/modal.tsx';
import { scrollbarDefault } from '#/css.ts';
import { cn } from '#/lib/utils.ts';

export const Route = createFileRoute('/settings')({
	component: RouteComponent,
});

function RouteComponent() {
	const baseItems = [
		{
			value: '/settings',
			label: '常规',
			icon: Settings,
		},
		{
			value: '/settings/appearance',
			label: '外观',
			icon: Palette,
		},
	] as const;
	return (
		<ModalOnRoute
			size="5xl"
			className={cn('p-0 grid grid-cols-[auto_1fr] min-h-0 h-full w-full')}
		>
			<aside class="min-w-40 grid p-3 max-h-full h-full grid-rows-[1fr_auto] border-r border-border">
				<div>
					<nav class="flex gap-1 flex-col">
						{baseItems.map((item) => (
							<NoStyleLink
								key={item.value}
								to={item.value}
								search={true}
								replace
								class="data-[status=active]:text-primary"
							>
								<Button
									value={item.value}
									variant="ghost"
									class={cn('w-full justify-start')}
								>
									{item.icon && <item.icon size={16} />}
									{item.label}
								</Button>
							</NoStyleLink>
						))}
					</nav>
				</div>
			</aside>
			<div class="h-full max-h-full min-h-0 pl-3 pt-3 grid grid-rows-[auto_1fr]">
				<h2 class="text-base font-medium mb-2 h-6">TitleString</h2>
				<div class={cn('min-h-0 overflow-y-auto ', scrollbarDefault)}>
					<Outlet />
				</div>
			</div>
		</ModalOnRoute>
	);
}
