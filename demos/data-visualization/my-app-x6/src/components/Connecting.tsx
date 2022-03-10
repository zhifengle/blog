import { Graph } from '@antv/x6';
import React, { FC, useEffect, useRef, useState } from 'react';

// https://x6.antv.vision/zh/examples/showcase/faq#validate-connection
export interface IAppProps {}
const portAttr = {
  circle: {
    r: 6,
    magnet: true,
    stroke: '#31d0c6',
    strokeWidth: 2,
    fill: '#fff',
    event: 'port:click',
  },
};

const Connecting: FC<IAppProps> = () => {
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
            // connector: 'rounded',
            validateConnection({ targetView, targetMagnet, sourceMagnet }) {
              if (!targetMagnet) {
                return false;
              }
              // out 连接 in 否则无法连接
              if (targetMagnet.getAttribute('port-group') !== 'in') {
                return false;
              }

              if (targetView && sourceMagnet) {
                const node = targetView.cell;
                // 实体的 svg 节点
                const portId = targetMagnet.getAttribute('port');
                const srcPortId = sourceMagnet.getAttribute('port');
                if (portId === srcPortId) {
                  return false;
                }
              }

              return true;
            },
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
              id: 'port-in',
              group: 'in',
            },
            {
              id: 'port-out',
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
              id: 'port-in',
              group: 'in',
            },
            {
              id: 'port-out',
              group: 'out',
            },
          ],
        },
      });
      graph.zoomToFit({ padding: 10, maxScale: 1 });
      graph.on('edge:connected', (e: any) => {
        // handle
        console.log('eeeeee: ', e);
      });
      console.log(graph);
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

export default Connecting;
