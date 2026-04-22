import { Link } from '@tanstack/solid-router';
import packageJson from '#/../package.json';
import {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarGroup,
	SidebarHeader,
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
} from '#/components/ui/sidebar';
import { UxTooltip } from '#/components/uix/tooltip.tsx';

export function AppSidebar() {
	return (
		<Sidebar>
			<SidebarHeader class="flex-row">
				<UxTooltip content={`Version ${packageJson.version}`}>
					<Link to="/">
						<h1 class="flex gap-1">
							<span>AA</span>
							<span class="text-muted-foreground">Code</span>
						</h1>
					</Link>
				</UxTooltip>
			</SidebarHeader>
			<SidebarContent>
				<SidebarGroup />
				<SidebarGroup />
			</SidebarContent>
			<SidebarFooter>
				<Link to="/settings" search>
					<SidebarMenuButton>Settings</SidebarMenuButton>
				</Link>
			</SidebarFooter>
		</Sidebar>
	);
}
