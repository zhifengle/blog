import { Graph } from '@antv/x6';
import React, { FC, useEffect, useRef } from 'react';
import data from './data.json';

export interface IAppProps {}
// https://x6.antv.vision/zh/examples/showcase/practices#bpmn
Graph.registerNode(
  'event',
  {
    inherit: 'circle',
    attrs: {
      body: {
        strokeWidth: 2,
        stroke: '#5F95FF',
        fill: '#FFF',
      },
    },
  },
  true
);

Graph.registerNode(
  'activity',
  {
    inherit: 'rect',
    markup: [
      {
        tagName: 'rect',
        selector: 'body',
      },
      {
        tagName: 'image',
        selector: 'img',
      },
      {
        tagName: 'text',
        selector: 'label',
      },
    ],
    attrs: {
      body: {
        rx: 6,
        ry: 6,
        stroke: '#5F95FF',
        fill: '#EFF4FF',
        strokeWidth: 1,
      },
      img: {
        x: 6,
        y: 6,
        width: 16,
        height: 16,
        'xlink:href':
          'https://gw.alipayobjects.com/mdn/rms_43231b/afts/img/A*pwLpRr7QPGwAAAAAAAAAAAAAARQnAQ',
      },
      label: {
        fontSize: 12,
        fill: '#262626',
      },
    },
  },
  true
);

Graph.registerNode(
  'gateway',
  {
    inherit: 'polygon',
    attrs: {
      body: {
        refPoints: '0,10 10,0 20,10 10,20',
        strokeWidth: 2,
        stroke: '#5F95FF',
        fill: '#EFF4FF',
      },
      label: {
        text: '+',
        fontSize: 40,
        fill: '#5F95FF',
      },
    },
  },
  true
);

Graph.registerEdge(
  'bpmn-edge',
  {
    inherit: 'edge',
    attrs: {
      line: {
        stroke: '#A2B1C3',
        strokeWidth: 2,
      },
    },
  },
  true
);

const App: FC<IAppProps> = () => {
  const canvasRef = useRef(null);
  useEffect(() => {
    if (canvasRef && canvasRef.current) {
      // new Editor(canvasRef.current);
      const graph = new Graph({
        container: canvasRef.current,
        // width: 800,
        // height: 600,
        connecting: {
          router: 'orth',
        },
      });
      let graphData: any = {
        nodes: [],
        edges: [],
      };
      data.forEach((item: any) => {
        if (item.shape === 'bpmn-edge') {
          graphData.edges.push(item);
        } else {
          graphData.nodes.push(item);
        }
      });
      graph.fromJSON(graphData);
      graph.zoomToFit({ padding: 10, maxScale: 1 });
    }
  }, [canvasRef]);
  return (
    <div
      style={{
        width: '100%',
        height: '100%',
      }}
      ref={canvasRef}
    ></div>
  );
};

export default App;
