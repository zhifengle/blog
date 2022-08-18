import React from 'react';
// https://www.30secondsofcode.org/react/s/use-window-size

const useWindowSize = () => {
  const [windowSize, setWindowSize] = React.useState({
    width: undefined,
    height: undefined,
  });

  React.useEffect(() => {
    const handleResize = () =>
      setWindowSize({ width: window.innerWidth, height: window.innerHeight });

    window.addEventListener('resize', handleResize);

    handleResize();

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, []);

  return windowSize;
};
export default useWindowSize;

/**
 const MyApp = () => {
  const { width, height } = useWindowSize();

  return (
    <p>
      Window size: ({width} x {height})
    </p>
  );
};
 */
