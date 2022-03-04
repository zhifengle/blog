import { Circle, Group, Rect, Text } from 'zrender';
import { createAnchor, createBoundingRect } from '../anchor';
import { initEvent } from './shape';
import Node from './node';

function createCircleNode(circle?: Circle) {
  if (!circle) {
    circle = new Circle({
      textContent: new Text({
        style: {
          text: '13',
        },
      }),
      textConfig: {
        position: 'inside',
      },
      shape: {
        cx: 40,
        cy: 40,
        r: 40,
      },
      style: {
        fill: '#fff',
        stroke: '#F00',
      },
    });
  }

  return new Node(circle);
}

export default createCircleNode;
