import { useRef } from 'react';

export default function ProfitGraph() {
  const ref = useRef(null);
  // useEffect(() => {
  //   const graph = new Graph(input.legend, input.data);
  //   const dispose = graph.draw(ref.current);
  //   return dispose;
  // }, [input, ref]);
  return (
    <div ref={ref}></div>
  );
}
