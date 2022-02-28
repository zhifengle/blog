// https://www.30secondsofcode.org/js/s/create-event-hub
// emit on off

export const getURLParameters = (url: string) =>
  (url.match(/([^?=&]+)(=([^&]*))/g) || []).reduce(
    (a, v) => (
      // @ts-ignore
      (a[v.slice(0, v.indexOf('='))] = v.slice(v.indexOf('=') + 1)), a
    ),
    {}
  );

// https://www.30secondsofcode.org/articles/s/copy-text-to-clipboard-with-javascript
const copyToClipboard = (str: string) => {
  if (navigator && navigator.clipboard && navigator.clipboard.writeText)
    return navigator.clipboard.writeText(str);
  return Promise.reject('The Clipboard API is not available.');
};

const copyToClipboard2 = (str: string) => {
  const el = document.createElement('textarea');
  el.value = str;
  el.setAttribute('readonly', '');
  el.style.position = 'absolute';
  el.style.left = '-9999px';
  document.body.appendChild(el);
  el.select();
  document.execCommand('copy');
  document.body.removeChild(el);
};

// https://www.30secondsofcode.org/js/s/run-async
// 使用 Worker +  URL.createObjectURL postMessage

const serializeForm = (form: HTMLFormElement) =>
  Array.from(new FormData(form), (field) =>
    // @ts-ignore
    field.map(encodeURIComponent).join('=')
  ).join('&');
