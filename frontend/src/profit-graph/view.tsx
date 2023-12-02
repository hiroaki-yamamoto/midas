import { useRef, useEffect } from 'react';
import { Graph } from './graph.ts';
import { IData } from './data.interface';

import style from './style.module.scss';

export default function ProfitGraph(input: { data: IData[] }) {
  const ref = useRef(null);
  useEffect(() => {
    const graph = new Graph(input.data);
    const dispose = graph.draw(ref.current);
    return dispose;
  }, [input, ref]);
  return (
    <div ref={ref} className={style.graph}></div>
  );
}
