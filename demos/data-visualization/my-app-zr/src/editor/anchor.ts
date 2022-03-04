import { Circle, Element as ZElement, PointLike, Rect } from 'zrender';
import { ANCHOR_COLOR, BG_COLOR } from './constants';

function genRectPoints(node: ZElement) {
  const points: PointLike[] = [];
  const box = node.getBoundingRect();
  const t = {
    x: box.x + box.width / 2,
    y: box.y,
  };
  const r = {
    x: box.x + box.width,
    y: box.y + box.height / 2,
  };
  const b = {
    x: box.x + box.width / 2,
    y: box.y + box.height,
  };
  const l = {
    x: box.x,
    y: box.y + box.height / 2,
  };
  points.push(t, r, b, l);
  return points;
}

export function createAnchor(node: ZElement) {
  var points = genRectPoints(node);
  return points.map((p) => {
    return new Circle({
      style: {
        fill: BG_COLOR,
        stroke: ANCHOR_COLOR,
        lineWidth: 1,
      },
      shape: {
        cx: p.x,
        cy: p.y,
        r: 3,
      },
      cursor: 'crosshair',
      z: 30001,
    });
  });
}

export function createBoundingRect(node: ZElement) {
  const rect = node.getBoundingRect();
  return new Rect({
    shape: {
      // cx: 0,
      // cy: 0,
      x: rect.x,
      y: rect.y,
      width: rect.width,
      height: rect.height,
    },
    style: {
      fill: 'none',
      stroke: '#14f1ff',
    },
  });
}
