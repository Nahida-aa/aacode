import { Link, type LinkProps } from '@tanstack/solid-router';
import type { ComponentProps } from 'solid-js';
import { cn } from '#/lib/utils';

export const NoStyleLink = (props: ComponentProps<'a'> & LinkProps) => {
	return (
		<Link
			{...props}
			class={cn(
				'text-inherit no-underline inline-flex items-center',
				props.class,
			)}
		/>
	);
};

export const UserLink = ({ username }: { username: string }) => (
	<NoStyleLink
		href={`/user/${username}`}
		class="hover:underline hover:text-primary"
	>
		{username}
	</NoStyleLink>
);
