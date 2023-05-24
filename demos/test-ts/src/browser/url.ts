export function patchURL(url: string, query: Record<string, string>): string {
  const urlObj = new URL(url);
  const params = new URLSearchParams(urlObj.search);
  for (const key in query) {
    params.set(key, query[key]);
  }
  urlObj.search = params.toString();
  return urlObj.href;
}
