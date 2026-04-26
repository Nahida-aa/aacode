import path from 'path';
import { type IconMetadata } from '../types.ts';
import { readSvgDirectory } from '../helpers/readSvgDirectory.ts';

export async function getIconMetaData(iconDirectory: string): Promise<Record<string, IconMetadata>> {
  const iconJsons = await readSvgDirectory(iconDirectory, '.json');
  const aliasesEntries = await Promise.all(
    iconJsons.map(async (jsonFile: string) => {
      /** eslint-disable */
      const file = await import(path.join(iconDirectory, jsonFile), { with: { type: 'json' } });
      return [path.basename(jsonFile, '.json'), file.default];
    }),
  );

  return Object.fromEntries(aliasesEntries);
}
