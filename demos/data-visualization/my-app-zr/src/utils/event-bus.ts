import mitt from 'mitt';

export type Events = {
  'editor/edge': { x1: number; x2: number; y1: number; y2: number };
};

const emmiter = mitt<Events>();
export default emmiter;
