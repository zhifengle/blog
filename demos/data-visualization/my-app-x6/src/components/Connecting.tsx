import { Graph } from '@antv/x6';
import React, { FC, useEffect, useRef, useState } from 'react';

export interface IAppProps {}
const portAttr = {
  circle: {
    r: 6,
    magnet: true,
    stroke: '#31d0c6',
    strokeWidth: 2,
    fill: '#fff',
  },
};

const App: FC<IAppProps> = () => {
  const canvasRef = useRef(null);
  const [graph, setGraph] = useState<Graph>();
  useEffect(() => {
    if (canvasRef && canvasRef.current) {
      setGraph(
        new Graph({
          container: canvasRef.current,
          connecting: {
            allowBlank: false,
            allowMulti: false,
            allowLoop: false,
            allowNode: false,
            allowEdge: false,
            allowPort: true,
          },
        })
      );
    }
  }, [canvasRef]);
  useEffect(() => {
    if (graph) {
      // 生成的是 Cell
      graph.addNode({
        x: 60,
        y: 50,
        width: 120,
        height: 64,
        ports: {
          groups: {
            in: {
              attrs: portAttr,
              position: 'top',
            },
            out: {
              attrs: portAttr,
              position: 'bottom',
            },
          },
          items: [
            {
              id: 'port1',
              group: 'in',
            },
            {
              id: 'port5',
              group: 'out',
            },
          ],
        },
      });

      graph.addNode({
        x: 160,
        y: 200,
        width: 120,
        height: 64,
        ports: {
          groups: {
            in: {
              attrs: portAttr,
              position: 'top',
            },
            out: {
              attrs: portAttr,
              position: 'bottom',
            },
          },
          items: [
            {
              id: 'port1',
              group: 'in',
            },
            {
              id: 'port5',
              group: 'out',
            },
          ],
        },
      });
      graph.zoomToFit({ padding: 10, maxScale: 1 });
      console.log(graph.toJSON());
    }
  }, [graph]);
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
