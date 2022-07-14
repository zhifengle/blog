import { execSync } from 'child_process';
import path from 'path';
// 需要 python -m venv .venv
export function executePython(
  folder: string,
  script: string,
  args: string = ''
): string {
  const py = path.join(folder, '.venv/Scripts/python');
  const scriptPath = path.join(folder, script);
  // stdio: inherit 是直接打印
  return execSync(`${py} ${scriptPath} ${args}`).toString();
}
