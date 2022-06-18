import { convert } from '../convert';
import { CodeNodeTypes, NodeTypes, VNode } from '../types';

describe('convert bbcode to html vnode', () => {
  test('basic bbcode', () => {
    const nodes: CodeNodeTypes[] = [
      '啊aあ\n)',
      {
        type: 'b',
        children: ['加粗'],
      },
      {
        type: 'i',
        children: ['斜体'],
      },
      {
        type: 'u',
        children: ['下划线'],
      },
      {
        type: 's',
        children: ['删除线'],
      },
      {
        type: 'mask',
        children: ['mask'],
      },
      {
        type: 'color',
        props: {
          color: 'red',
        },
        children: ['红'],
      },
      {
        type: 'size',
        props: {
          size: '18',
        },
        children: ['大小'],
      },
    ];
    const vnodes: NodeTypes[] = [
      '啊aあ\n)',
      {
        type: 'strong',
        children: ['加粗'],
      },
      {
        type: 'em',
        children: ['斜体'],
      },
      {
        type: 'span',
        style: {
          'text-decoration': 'underline',
        },
        children: ['下划线'],
      },
      {
        type: 'span',
        style: {
          'text-decoration': 'line-through',
        },
        children: ['删除线'],
      },
      {
        type: 'span',
        style: {
          'background-color': '#555',
          color: '#555',
          border: '1px solid #555',
        },
        children: ['mask'],
      },
      {
        type: 'span',
        style: {
          color: 'red',
        },
        children: ['红'],
      },
      {
        type: 'span',
        style: {
          'font-size': '18px',
          'line-height': '18px',
        },
        children: ['大小'],
      },
    ];
    for (let i = 0; i < nodes.length; i++) {
      const node = nodes[i];
      expect(convert(node)).toEqual(vnodes[i]);
    }
  });
  test('url bbcode', () => {
    const nodes: CodeNodeTypes[] = [
      {
        type: 'url',
        props: {
          url: 'http://chii.in/',
        },
        children: ['http://chii.in/'],
      },
      {
        type: 'url',
        props: {
          url: 'http://test.com/',
        },
        children: ['测试'],
      },
    ];
    const vnodes: NodeTypes[] = [
      {
        type: 'a',
        props: {
          href: 'http://chii.in/',
        },
        className: 'l',
        children: ['http://chii.in/'],
      },
      {
        type: 'a',
        props: {
          href: 'http://test.com/',
          target: '_blank',
          ref: 'nofollow external noopener noreferrer',
        },
        className: 'l',
        children: ['测试'],
      },
    ];
    for (let i = 0; i < nodes.length; i++) {
      const node = nodes[i];
      expect(convert(node)).toEqual(vnodes[i]);
    }
  });
  test('img bbcode', () => {
    const nodes: CodeNodeTypes[] = [
      {
        type: 'img',
        children: ['http://chii.in/img/ico/bgm88-31.gif'],
      },
      {
        type: 'img',
        children: ['http://test.com/xx.png'],
      },
    ];
    const vnodes: NodeTypes[] = [
      {
        type: 'img',
        props: {
          src: 'http://chii.in/img/ico/bgm88-31.gif',
        },
        className: 'code',
      },
      {
        type: 'img',
        props: {
          src: 'http://test.com/xx.png',
          referrerpolicy: 'no-referrer',
          ref: 'noreferrer',
        },
        className: 'code',
      },
    ];
    for (let i = 0; i < nodes.length; i++) {
      const node = nodes[i];
      expect(convert(node)).toEqual(vnodes[i]);
    }
  });
  test('nest bbcode', () => {
    const node: CodeNodeTypes = {
      type: 'b',
      children: [
        {
          type: 'i',
          children: [
            {
              type: 'u',
              children: [
                {
                  type: 's',
                  children: ['测试文字'],
                },
              ],
            },
          ],
        },
      ],
    };
    const vnode: VNode = {
      type: 'strong',
      children: [
        {
          type: 'em',
          children: [
            {
              type: 'span',
              style: { 'text-decoration': 'underline' },
              children: [
                {
                  type: 'span',
                  style: { 'text-decoration': 'line-through' },
                  children: ['测试文字'],
                },
              ],
            },
          ],
        },
      ],
    };
    expect(convert(node)).toEqual(vnode);
  });
});
