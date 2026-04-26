import {
	mergeClasses,
	toKebabCase,
	toPascalCase,
} from '@repo/build-icons/utils';
import { createElement, forwardRef } from 'react';
import Icon from './Icon';
import type { IconNode, LucideProps } from './types';

/**
 * Create a Lucide icon component
 * @param {string} iconName
 * @param {array} iconNode
 * @returns {ForwardRefExoticComponent} LucideIcon
 */
const createLucideIcon = (iconName: string, iconNode: IconNode) => {
	const Component = forwardRef<SVGSVGElement, LucideProps>(
		({ className, ...props }, ref) =>
			createElement(Icon, {
				ref,
				iconNode,
				className: mergeClasses(
					`lucide-${toKebabCase(toPascalCase(iconName))}`,
					`lucide-${iconName}`,
					className,
				),
				...props,
			}),
	);

	Component.displayName = toPascalCase(iconName);

	return Component;
};

export default createLucideIcon;
