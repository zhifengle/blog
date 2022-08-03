import type { NextPage } from 'next';
import { useState } from 'react';
import { useSpring, config, animated } from '@react-spring/web';

const TextPage: NextPage = () => {
  const [flip, set] = useState(false);
  const props = useSpring({
    to: { opacity: 1 },
    from: { opacity: 0 },
    reset: true,
    reverse: flip,
    delay: 200,
    config: config.molasses,
    onRest: () => set(!flip),
  });

  return <animated.h1 style={props}>hello</animated.h1>;
};

export default TextPage;
