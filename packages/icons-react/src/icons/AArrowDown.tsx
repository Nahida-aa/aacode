/**
 * @component
 * @name AArrowDown
 * @description Custom SVG icon component.
 *
 * @preview ![img](data:image/svg+xml;base64,PHN2ZyAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiAgd2lkdGg9IjI0IiAgaGVpZ2h0PSIyNCIgIHZpZXdCb3g9IjAgMCAyNCAyNCIgIGZpbGw9Im5vbmUiICBzdHJva2U9IiMwMDAiIHN0eWxlPSJiYWNrZ3JvdW5kLWNvbG9yOiAjZmZmOyBib3JkZXItcmFkaXVzOiAycHgiICBzdHJva2Utd2lkdGg9IjIiICBzdHJva2UtbGluZWNhcD0icm91bmQiICBzdHJva2UtbGluZWpvaW49InJvdW5kIj4gIDxwYXRoIGQ9Im0xNCAxMiA0IDQgNC00IiAvPiAgPHBhdGggZD0iTTE4IDE2VjciIC8+ICA8cGF0aCBkPSJtMiAxNiA0LjAzOS05LjY5YS41LjUgMCAwIDEgLjkyMyAwTDExIDE2IiAvPiAgPHBhdGggZD0iTTMuMzA0IDEzaDYuMzkyIiAvPjwvc3ZnPg==)
 */
export const AArrowDown = (props: { size?: number; className?: string }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={props.size || 24}
    height={props.size || 24}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth={2}
    strokeLinecap="round"
    strokeLinejoin="round"
    className={props.className}
  >
    <path d="m14 12 4 4 4-4" />
    <path d="M18 16V7" />
    <path d="m2 16 4.039-9.69a.5.5 0 0 1 .923 0L11 16" />
    <path d="M3.304 13h6.392" />
  </svg>
);