function removeDuplicateLetters(s: string): string {
  const dict: Record<string, number> = {};
  for (const c of s) {
    if (dict[c] !== undefined) {
      dict[c] += 1;
    } else {
      dict[c] = 1;
    }
  }
  const stack: string[] = [];
  for (const c of s) {
    if (!stack.includes(c)) {
      while (
        stack.length > 0 &&
        c < stack[stack.length - 1] &&
        dict[stack[stack.length - 1]] > 0
      ) {
        stack.pop();
      }
      stack.push(c);
    }
    dict[c] -= 1;
  }
  return stack.join('');
}

// eacb
console.log(removeDuplicateLetters('ecbacba'));
