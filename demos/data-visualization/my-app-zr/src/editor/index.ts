import {
  Group,
  init,
  registerPainter,
  ZRenderType,
  Displayable,
  Line,
} from 'zrender';
import CanvasPainter from 'zrender/lib/canvas/Painter';
// import SVGPainter from 'zrender/lib/svg/Painter';
import createCicle from './node/circle';

export default class Editor extends Group {
  zr: ZRenderType;
  nodes: Displayable[];
  groups: Group[];
  link: Line;
  constructor(dom: HTMLCanvasElement) {
    super();
    // zrender v5 需要设置
    registerPainter('canvas', CanvasPainter);
    // registerPainter('svg', SVGPainter);
    this.zr = init(dom);
    this.nodes = [];
    this.groups = [];
    // 测试划线
    this.link = new Line({
      style: {
        lineDash: [6, 8],
      },
    });
    this.link.hide();
    this.add(this.link);

    this.zr.add(this);
    this.add(createCicle().group);
    const c2 = createCicle().group;
    c2.x = 400;
    c2.y = 400;
    this.add(c2);

    this.initEvent();
  }
  initEvent() {
    let dragging = false;
    let startX = 0;
    let startY = 0;
    // todo
    this.zr.on('mousedown', (e) => {
      dragging = true;
      startX = e.offsetX;
      startY = e.offsetY;
      if (e.target) {
      }
    });
    this.zr.on('mousemove', (e) => {
      if (dragging) {
        // draw link
        this.link.attr({
          shape: {
            x1: startX,
            y1: startY,
            x2: e.offsetX,
            y2: e.offsetY,
          },
        });
        this.link.show();
      }
    });
    this.zr.on('mouseup', (e) => {
      dragging = false;
    });
  }
}
