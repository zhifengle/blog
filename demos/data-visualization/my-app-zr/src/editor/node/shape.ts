import { Displayable } from 'zrender';

// 鼠标移入 circle 里面？
export function initEvent(el: Displayable) {
  el.on('dragstart', () => {
    //
  });
  el.on('dragend', (e) => {
    console.log(e);
  });
  el.on('mouseover', (e) => {
    console.log(e);
  });
  // el.on('mouseover', function () {
  //   if (!el.dragging) {
  //   }
  //   this.attr({
  //     z: 20000,
  //   });
  // });
}

// ElementEventHandlerProps
export function setShadow(el: Displayable) {
  el.attr({
    style: {
      shadowBlur: 6,
      shadowColor: '#ccc',
    },
  });
}
