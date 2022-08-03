import type { NextPage } from 'next';
import { Parallax, ParallaxLayer } from '@react-spring/parallax';

// ParallaxLayer speed 负值是从上往下

const ParallaxPage: NextPage = () => {
  return (
    <Parallax pages={2} style={{ top: '0', left: '0' }}>
      <ParallaxLayer
        offset={0}
        speed={2.5}
        style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
        }}
      >
        <p>Scroll down</p>
      </ParallaxLayer>

      {/* speed 越大越慢? */}
      <ParallaxLayer
        offset={1}
        speed={2}
        style={{ backgroundColor: '#ff6d6d' }}
      />

      <ParallaxLayer
        offset={1}
        speed={0.5}
        style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          color: 'white',
        }}
      >
        <p>Scroll up</p>
      </ParallaxLayer>
    </Parallax>
  );
};

export default ParallaxPage;
