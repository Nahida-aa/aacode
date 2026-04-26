import { Link } from '@tanstack/react-router';
import { User2Icon } from 'lucide-react';
import { buttonVariants } from '#/components/ui/button.tsx';
import type { NavItem } from '#/components/uix/nav/index.tsx';

export function DmSide() {
	return (
		<aside className="border-r  min-h-0 grid ">
			<nav>
				<Link to="/demo/chat/dm" search className={buttonVariants()}>
					Friend
				</Link>
			</nav>
		</aside>
	);
}
