import { toCamelCase } from './toCamelCase';

/**
 * Convert a type string from camelCase to PascalCase
 *
 * @example
 * type Test = CamelToPascal<'fooBar'> // 'FooBar'
 */
export type CamelToPascal<T extends string> = T extends `${infer FirstChar}${infer Rest}`
  ? `${Capitalize<FirstChar>}${Rest}`
  : never;
/**
 * Converts string to pascal case
 *
 * @param {string} string
 * @returns {string} A pascalized string
 */
export const toPascalCase = <T extends string>(string: T): CamelToPascal<T> => {
  const camelCase = toCamelCase(string);

  return (camelCase.charAt(0).toUpperCase() + camelCase.slice(1)) as CamelToPascal<T>;
};