// 创建隐藏的 DOM; ref: el-scrollbar
export function createHiddenDOM(tag: string) {
  const outer = document.createElement(tag);
  outer.className = 'el-scrollbar__wrap';
  outer.style.visibility = 'hidden';
  outer.style.width = '100px';
  outer.style.position = 'absolute';
  outer.style.top = '-9999px';
  document.body.appendChild(outer);
  return outer;
}
