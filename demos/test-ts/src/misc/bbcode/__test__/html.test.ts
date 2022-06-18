import { render, renderNode } from '../html';
import { VNode } from '../types';

describe('html render', () => {
  test('render multi style', () => {
    const vnode: VNode = {
      type: 'span',
      style: {
        'font-size': '18px',
        'line-height': '18px',
      },
      className: 'l',
      children: ['文字\n换行了'],
    };
    expect(renderNode(vnode)).toBe(
      '<span style="font-size:18px;line-height:18px" class="l">文字<br/>换行了</span>'
    );
  });
  test('render link', () => {
    const vnode = {
      type: 'a',
      props: {
        href: 'http://chii.in/',
        target: '_blank',
        ref: 'nofollow external noopener noreferrer',
      },
      className: ['l', 'test'],
    };
    expect(renderNode(vnode)).toBe(
      '<a href="http://chii.in/" target="_blank" ref="nofollow external noopener noreferrer" class="l test"/>'
    );
  });
  test('render sticker node', () => {
    const id = '38';
    const vnode = {
      type: 'img',
      props: {
        'sticker-id': id,
        smileid: id,
        alt: `(bgm${id})`,
      },
    };
    expect(renderNode(vnode)).toBe(
      '<img sticker-id="38" smileid="38" alt="(bgm38)"/>'
    );
  });
  test('render nodes', () => {
    const vnodes = [
      {
        type: 'strong',
        children: ['粗体字'],
      },
      {
        type: 'em',
        children: ['斜体字'],
      },
      {
        type: 'span',
        style: {
          'text-decoration': 'underline',
        },
        children: ['下划线文字'],
      },
      {
        type: 'span',
        style: {
          'text-decoration': 'line-through',
        },
        children: ['删除线文字'],
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
          'font-size': '18pt',
        },
        children: ['文字'],
      },
    ];
    expect(render(vnodes)).toBe(
      '<strong>粗体字</strong><em>斜体字</em><span style="text-decoration:underline">下划线文字</span><span style="text-decoration:line-through">删除线文字</span><span style="color:red">红</span><span style="font-size:18pt">文字</span>'
    );
  });
  test('render nest', () => {
    const sizeNode: VNode = {
      type: 'span',
      style: {
        'font-size': '18pt',
      },
      children: [
        {
          type: 'a',
          props: {
            href: 'http://chii.in/',
          },
          className: ['l'],
        },
      ],
    };
    const vnode: VNode = {
      type: 'span',
      style: {
        color: 'red',
      },
      children: [sizeNode],
    };
    expect(renderNode(sizeNode)).toBe(
      '<span style="font-size:18pt"><a href="http://chii.in/" class="l"/></span>'
    );
    expect(renderNode(vnode)).toBe(
      '<span style="color:red"><span style="font-size:18pt"><a href="http://chii.in/" class="l"/></span></span>'
    );
  });
  test('render mask', () => {
    const vnode: VNode = {
      type: 'span',
      style: {
        'background-color': '#555',
        color: '#555',
        border: '1px solid #555',
      },
      className: 'l',
      children: ['文字\n换行'],
    };
    expect(renderNode(vnode)).toBe(
      '<span style="background-color:#555;color:#555;border:1px solid #555" class="l">文字<br/>换行</span>'
    );
  });
});
