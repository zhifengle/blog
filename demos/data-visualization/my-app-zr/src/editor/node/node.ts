import { Circle, Element, Group, GroupProps, Rect } from 'zrender';
import { createAnchor, createBoundingRect } from '../anchor';

export default class Node {
  group: Group;
  el: Element;
  anchors: Circle[];
  // anchorGroup: Group;
  rect: Rect;
  constructor(el: Element) {
    this.el = el;
    this.group = new Group({
      draggable: true,
    });
    // 必须先加入元素
    this.group.add(el);

    //
    this.anchors = createAnchor(el);
    this.rect = createBoundingRect(el);
    this.rect.hide();
    this.group.add(this.rect);

    this.anchors.map((c) => this.group.add(c));
    this.initEvent();
  }
  initEvent() {
    this.group.on('dragstart', (e) => {
      this.hideAnchor();
    });
    this.group.on('dragend', (e) => {
      this.showAnchor();
    });
    this.group.on('mouseover', (e) => {
      if (!this.group.dragging) {
        this.showAnchor();
      }
    });
    this.group.on('mousemove', (e) => {
      if (!this.group.dragging) {
        if (this.anchors.find((c) => c === e.target)) {
          this.group.draggable = false;
        } else {
          this.group.draggable = true;
        }
      }
    });
    this.group.on('mouseout', (e) => {
      this.hideAnchor();
    });
  }
  showAnchor() {
    this.anchors.forEach((c) => c.show());
  }
  hideAnchor() {
    this.anchors.forEach((c) => c.hide());
  }
  // 判断点是否在 anchor 上面。
  getOverAnchorIndex(x: number, y: number) {
    return this.anchors.findIndex((c) => c.contain(x, y));
  }
}
