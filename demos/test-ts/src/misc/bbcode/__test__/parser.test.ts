import { BBCODE_REGEXP, Parser, NodeTypes } from '../parser';

function getNodes(input: string): NodeTypes[] {
  const p = new Parser(input);
  return p.parseNodes();
}

describe('bbcode parser', () => {
  test('bbcode regexp', () => {
    const url = `[url=http://chii.in]bgm[/url]`;
    expect(url).toMatch(BBCODE_REGEXP);
    let m = url.match(BBCODE_REGEXP);
    expect(m[1]).toBe('url');
    expect(m[2]).toBe('=http://chii.in');
    expect(m[3]).toBe('bgm');
    expect(m[4]).toBe('url');

    const bolded = `[b]粗体字[/b]`;
    expect(bolded).toMatch(BBCODE_REGEXP);
    m = bolded.match(BBCODE_REGEXP);
    expect(m[1]).toBe('b');
    expect(m[2]).toBeUndefined();
    expect(m[3]).toBe('粗体字');
    expect(m[4]).toBe('b');
  });
  test('text', () => {
    const input = `啊aあ\n)`;
    const tests: NodeTypes[] = [input];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
  test('bangumi sticker', () => {
    const input = `(bgm38)`;
    const id = '38';
    const tests: NodeTypes[] = [
      {
        type: 'img',
        props: {
          'sticker-id': id,
          // smileid: id,
          // alt: `(bgm${id})`,
        },
      },
    ];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
  test('bold bbcode', () => {
    const input = `[b]bgm[/b]`;
    // const input2 = `[url=http://chii.in]bgm[/url]`;
    // const input3 = ` [url]http://chii.in/[/url]`;
    const tests: NodeTypes[] = [
      {
        type: 'b',
        children: ['bgm'],
      },
    ];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
  test('url bbcode', () => {
    const input = `[url=http://chii.in]bgm[/url]`;
    const tests: NodeTypes[] = [
      {
        type: 'url',
        props: {
          url: 'http://chii.in',
        },
        children: ['bgm'],
      },
    ];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
  test('url bbcode; type 2', () => {
    const input = ` [url]http://chii.in/[/url]`;
    const tests: NodeTypes[] = [
      {
        type: 'url',
        children: ['http://chii.in/'],
      },
    ];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
  test('invalid bbcode', () => {
    const input = `[ba]bgm[/b](bg38)`;
    // const input = `[b]bgm[url]sss[/ual][/b](bg38)`;
    expect(getNodes(input).join('')).toEqual(input);
  });
  test('img bbcode', () => {
    const input = `存放于其他网络服务器的图片：
[img]http://chii.in/img/ico/bgm88-31.gif[/img]`;
    const tests: NodeTypes[] = [
      '存放于其他网络服务器的图片：\n',
      {
        type: 'img',
        children: ['http://chii.in/img/ico/bgm88-31.gif'],
      },
    ];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
  test('simple bbcode', () => {
    const input = `我是[mask]马赛克文字[/mask]
[s]删除线文字[/s]
[u]下划线文字[/u]
[i]斜体字[/i]
[b]粗体字[/b]
[color=red]彩[/color][color=green]色[/color][color=blue]的[/color][color=orange]哟[/color]
`;
    const tests: NodeTypes[] = [
      '我是',
      {
        type: 'mask',
        children: ['马赛克文字'],
      },
      '\n',
      {
        type: 's',
        children: ['删除线文字'],
      },
      '\n',
      {
        type: 'u',
        children: ['下划线文字'],
      },
      '\n',
      {
        type: 'i',
        children: ['斜体字'],
      },
      '\n',
      {
        type: 'b',
        children: ['粗体字'],
      },
      '\n',
      {
        type: 'color',
        props: {
          color: 'red',
        },
        children: ['彩'],
      },
      {
        type: 'color',
        props: {
          color: 'green',
        },
        children: ['色'],
      },
      {
        type: 'color',
        props: {
          color: 'blue',
        },
        children: ['的'],
      },
      {
        type: 'color',
        props: {
          color: 'orange',
        },
        children: ['哟'],
      },
      '\n',
    ];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
  test('nest bbcode', () => {
    const input = `[color=green]nest[size=16]更新：[/size][/color]`;
    const tests: NodeTypes[] = [
      {
        type: 'color',
        props: { color: 'green' },
        children: [
          'nest',
          {
            type: 'size',
            props: {
              size: '16',
            },
            children: ['更新：'],
          },
        ],
      },
    ];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
  test('nest same bbcode', () => {
    const input = `[color=green][color=blue]更新：[/color][/color]`;
    const tests: NodeTypes[] = [
      {
        type: 'color',
        props: { color: 'green' },
        children: [
          {
            type: 'color',
            props: {
              color: 'blue',
            },
            children: ['更新：'],
          },
        ],
      },
    ];
    expect(getNodes(input)).toEqual(expect.arrayContaining(tests));
  });
});
