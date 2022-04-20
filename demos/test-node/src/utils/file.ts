import { opendir } from 'fs/promises';
import path from 'path';

export async function* walk(
  dir: string,
  recursive: boolean
): AsyncGenerator<{ name: string; path: string }, void, void> {
  for await (const d of await opendir(dir)) {
    if (!recursive) {
      yield {
        name: d.name,
        path: dir,
      };
    } else if (d.isDirectory()) {
      const entry = path.join(dir, d.name);
      yield* walk(entry, true);
    } else if (d.isFile())
      yield {
        name: d.name,
        path: dir,
      };
  }
}
