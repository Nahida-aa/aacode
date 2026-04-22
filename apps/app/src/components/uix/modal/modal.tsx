'use client';
import { useRouter } from '@tanstack/solid-router';
import type { VariantProps } from 'class-variance-authority';
import { ArrowLeft, ArrowLeftIcon } from 'lucide-solid';
import { createSignal, type JSX } from 'solid-js';
import {} from 'solid-js/web';
import { Button } from '#/components/ui/button';
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogHeader,
	DialogTitle,
	DialogTrigger,
} from '#/components/ui/dialog';
import type { dialogContentVariants } from '#/css.ts';
import { cn } from '#/lib/utils';

export type ModalProps = {
	children?: JSX.Element;
	title?: JSX.Element;
	description?: JSX.Element;
	open?: boolean;
	defaultOpen?: boolean;
	onOpenChange?: (open: boolean) => void;
	closeOnOverlayClick?: boolean;
	showCloseButton?: boolean;
	pending?: boolean;
	className?: string;
	titleClassName?: string;
	descriptionClassName?: string;
	Trigger?: JSX.Element;
} & VariantProps<typeof dialogContentVariants>;
export function Modal({
	children,
	title,
	description,
	open,
	defaultOpen = false,
	onOpenChange,
	closeOnOverlayClick = true, // 是否 使通过点击遮罩层 关闭模态框 失效
	size = 'md',
	showCloseButton = true,
	pending = false,
	className = 'px-4 pb-4',
	titleClassName,
	descriptionClassName,
	Trigger,
}: ModalProps) {
	return (
		<Dialog
			open={open}
			defaultOpen={defaultOpen}
			onOpenChange={(open) => {
				if (pending) {
					return;
				}
				onOpenChange?.(open);
			}}
			// modal={false}
			// disablePointerDismissal={closeOnOverlayClick}
		>
			{Trigger && <DialogTrigger children={Trigger} />}
			<DialogContent
				class={className}
				showCloseButton={showCloseButton}
				size={size}
				onInteractOutside={(e) => {
					if (!closeOnOverlayClick) {
						e.preventDefault();
					}
				}}
				onPointerDownOutside={(e) => {
					const target = e.target as HTMLElement;
					console.log(target);
					if (
						target.closest('[data-sonner-toast]') ||
						target.closest('[data-testid=tanstack_devtools]')
					) {
						e.preventDefault();
					}
				}}
				pending={pending}
			>
				{title && (
					<DialogHeader class="h-fit pb-2">
						<DialogTitle
							class={cn('text-xl flex pt-3 justify-center', titleClassName)}
						>
							{title}
						</DialogTitle>
						{description && (
							<DialogDescription class={descriptionClassName}>
								{description}
							</DialogDescription>
						)}
					</DialogHeader>
				)}

				{/* <div className="py-2">{children}</div> */}
				{/* 'px-6 py-4 my-2' */}
				{/* <section
            className={cn('max-h-full max-w-full', contentClassName)}
          >
          </section> */}

				{children}
			</DialogContent>
		</Dialog>
	);
}
interface ModalOnRouteProps extends ModalProps {
	showBackButton?: boolean;
}
export function ModalOnRoute({
	defaultOpen = true,
	showBackButton = false, // 控制 是否显示返回按钮
	closeOnOverlayClick = false,
	onOpenChange,
	children,
	size,
	...props
}: ModalOnRouteProps) {
	const router = useRouter();
	if (!onOpenChange) {
		onOpenChange = (open) => router.history.back();
	}
	return (
		<Modal
			defaultOpen={defaultOpen}
			onOpenChange={(open) => {
				console.log('onOpenChange', open);
				onOpenChange(open);
			}}
			size={size}
			closeOnOverlayClick={closeOnOverlayClick}
			{...props}
		>
			{children}
			{showBackButton ||
				(size === 'full' && (
					<Button
						variant="ghost"
						size="icon-sm"
						class="absolute top-1 left-1 p-0 rounded-full"
						onClick={() => router.history.back()}
					>
						<ArrowLeftIcon />
					</Button>
				))}
		</Modal>
	);
}
interface ModalWithCloseProps extends Omit<ModalProps, 'children'> {
	children?: JSX.Element | ((close: () => void) => JSX.Element);
}
export function ModalWithClose({ children, ...props }: ModalWithCloseProps) {
	const [open, setOpen] = createSignal(props.defaultOpen);
	return (
		<Modal {...props} open={open()} onOpenChange={setOpen}>
			{
				typeof children === 'function'
					? children(() => {
							console.log('close');
							setOpen(false);
						}) // 如果是函数，执行并传入参数
					: children // 如果是普通节点，直接渲染
			}
		</Modal>
	);
}
