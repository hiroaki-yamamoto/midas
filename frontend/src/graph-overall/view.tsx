import { useRef, useEffect } from 'react';

import { Input } from './input.interface';
import { Graph } from './graph.ts';

import style from './style.module.scss';

export default function OverAllGraph(input: Input) {
  const ref = useRef(null);
  useEffect(() => {
    const graph = new Graph(input.legend, input.data);
    const dispose = graph.draw(ref.current);
    return dispose;
  }, [input, ref]);
  return (
    <div ref={ref} className={style.graph}></div>
  );
}
