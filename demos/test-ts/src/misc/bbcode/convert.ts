import { CodeNodeTypes, CodeVNode, NodeTypes, VNode } from './types';

const BGM_HOST_ARR = [
  'chii.in',
  'bangumi.tv',
  'www.chii.in',
  'www.bangumi.tv',
  'bgm.tv',
  'www.bgm.tv',
];
function isExternalLink(url: string) {
  try {
    const urlObj = new URL(url);
    return !BGM_HOST_ARR.includes(urlObj.host);
  } catch (error) {
    return true;
  }
}

function convertImgNode(node: CodeVNode): VNode {
  const src = node.children[0] as string;
  const vnode: VNode = {
    type: 'img',
    props: {
      src,
    },
    className: 'code',
  };
  if (isExternalLink(src)) {
    vnode.props = {
      ...vnode.props,
      referrerpolicy: 'no-referrer',
      ref: 'noreferrer',
    };
  }
  return vnode;
}

function convertUrlNode(node: CodeVNode): VNode {
  const href = node.props.url as string;
  const vnode: VNode = {
    type: 'a',
    props: {
      href,
    },
    className: 'l',
  };
  if (isExternalLink(href)) {
    vnode.props = {
      ...vnode.props,
      target: '_blank',
      ref: 'nofollow external noopener noreferrer',
    };
  }
  return vnode;
}

// function convertStickerNode(node: CodeVNode): NodeTypes {
//   let id = +node.props.smileid;
// }

// function converUnknownNode(node: CodeVNode): NodeTypes {}

export function convert(node: CodeNodeTypes): NodeTypes {
  if (typeof node === 'string') {
    return node;
  }
  const type = node.type;
  let vnode = {} as VNode;
  switch (type) {
    case 'b':
      vnode.type = 'strong';
      break;
    case 'i':
      vnode.type = 'em';
      break;
    case 'u':
      vnode.type = 'span';
      vnode.style = {
        'text-decoration': 'underline',
      };
      break;
    case 's':
      vnode.type = 'span';
      vnode.style = {
        'text-decoration': 'line-through',
      };
      break;
    case 'mask':
      vnode.type = 'span';
      vnode.style = {
        'background-color': '#555',
        color: '#555',
        border: '1px solid #555',
      };
      break;
    case 'color':
      vnode.type = 'span';
      vnode.style = {
        color: node.props.color as string,
      };
      break;
    case 'size':
      vnode.type = 'span';
      vnode.style = {
        'font-size': node.props.size + 'px',
        'line-height': node.props.size + 'px',
      };
      break;
    case 'url':
      vnode = convertUrlNode(node);
      break;
    case 'img':
      return convertImgNode(node);
    default:
      vnode = {
        ...node,
      };
      break;
  }
  if (node.children) {
    vnode.children = node.children.map((c) => convert(c));
  }
  return vnode;
}
