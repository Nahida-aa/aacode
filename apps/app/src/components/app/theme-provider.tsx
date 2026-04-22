import {
	createContext,
	createEffect,
	createSignal,
	type JSX,
	onCleanup,
	onMount,
	useContext,
} from 'solid-js';

export type Theme = 'dark' | 'light' | 'system';

type ThemeProviderState = {
	theme: () => Theme; // Solid 中状态通常作为 getter 函数访问
	setTheme: (theme: Theme) => void;
	systemTheme: () => 'light' | 'dark';
};

// 1. 创建 Context，初始值设为 undefined
const ThemeProviderContext = createContext<ThemeProviderState>();

export function ThemeProvider(props: {
	children: JSX.Element;
	defaultTheme?: Theme;
	storageKey?: string;
}) {
	const storageKey = props.storageKey ?? 'vite-ui-theme';

	// 2. 使用 Signals 代替 useState
	const [theme, _setTheme] = createSignal<Theme>(
		props.defaultTheme ?? 'system',
	);

	const [systemTheme, setSystemTheme] = createSignal<'light' | 'dark'>(
		window.matchMedia('(prefers-color-scheme: dark)').matches
			? 'dark'
			: 'light',
	);

	// 3. 初始化：从 localStorage 读取
	onMount(() => {
		const savedTheme = localStorage.getItem(storageKey) as Theme;
		if (savedTheme) {
			_setTheme(savedTheme);
		}

		// 4. 监听系统主题变化
		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		const handleChange = () =>
			setSystemTheme(mediaQuery.matches ? 'dark' : 'light');

		mediaQuery.addEventListener('change', handleChange);
		onCleanup(() => mediaQuery.removeEventListener('change', handleChange));
	});

	// 5. 应用样式到 documentElement (相当于 React 的 useEffect)
	// createEffect 会自动追踪 theme() 和 systemTheme() 的变化
	createEffect(() => {
		const root = window.document.documentElement;
		const activeTheme = theme() === 'system' ? systemTheme() : theme();

		root.classList.remove('light', 'dark');
		root.classList.add(activeTheme);
	});

	const value = {
		theme,
		setTheme: (newTheme: Theme) => {
			localStorage.setItem(storageKey, newTheme);
			_setTheme(newTheme);
		},
		systemTheme,
	};

	return (
		<ThemeProviderContext.Provider value={value}>
			{props.children}
		</ThemeProviderContext.Provider>
	);
}

// 6. Hook 导出
export const useTheme = () => {
	const context = useContext(ThemeProviderContext);
	if (!context) throw new Error('useTheme must be used within a ThemeProvider');
	return context;
};
