type IObj = {
  value?: string
  children?: IObj
}

function parseArrStr(str: string): IObj {
  const stack = []
  let i = 0
  let v = ''
  let res: IObj = undefined
  while (i < str.length) {
    const c = str[i]
    if (c === '[') {
      if (v) {
        stack.push(v)
        v = ''
      }
    } else if (c === ']') {
      if (v) {
        res = {
          value: v,
        }
        v = ''
      } else {
        const current = stack.pop()
        res = {
          value: current,
          children: res,
        }
      }
    } else {
      v += c
    }
    i++
    if (i === str.length && v) {
      stack.push(v)
      v = ''
    }
  }
  let p = ''
  while ((p = stack.pop())) {
    res = {
      value: p,
      children: res,
    }
  }
  return res
}

// console.log(JSON.stringify(parseArrStr('gg[abc[bcd[def]]]')))
// console.log(parseArrStr('gg'))

// str.split(/[\[\]]/g).filter(Boolean)
