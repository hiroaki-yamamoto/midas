import {
  useState, useEffect, useMemo, useCallback, useRef,
} from 'react';
import Swiper from 'swiper';

import { Bot } from '../rpc/bot.zod.ts';
import { Position } from '../rpc/position.zod.ts';
import { PositionStatus } from '../rpc/position-status.zod.ts';

import { PositionTable } from '../position-table/view.tsx';
import { Tab, Tabs } from '../tab/view.tsx';

import { Ctrl } from './controller.ts';

export const PositionTab = (input: { bot: Bot }) => {
  // Swipe panel related controls
  const resolveSwiper = useRef<(swiper: Swiper) => void>(() => { });
  const swiper = useMemo(() => {
    return new Promise<Swiper>(
      (resolve) => { resolveSwiper.current = resolve; });
  }, []);
  const onReady = (panelSwiper: Swiper) => {
    resolveSwiper.current(panelSwiper);
  };

  // Position table related controls
  const [openPos, setOpenPos] = useState<Position[]>([]);
  const [closePos, setClosePos] = useState<Position[]>([]);
  const ctrl = useMemo(() => new Ctrl(input.bot), [input.bot]);
  const rerender = useCallback(() => {
    swiper.then((sw) => {
      console.log(sw);
      setTimeout(() => {
        sw.update();
      }, 0);
    });
  }, [swiper]);
  useEffect(() => {
    ctrl.getPositions(PositionStatus.enum.OPEN).then((pos) => {
      setOpenPos(pos);
      rerender();
    });
    ctrl.getPositions(PositionStatus.enum.CLOSE).then(setClosePos);
  }, [setOpenPos, setClosePos, ctrl, rerender]);

  return (
    <Tabs
      labels={['Open Positions', 'Closed Positions']}
      onSwiperReady={onReady}>
      <Tab>
        <PositionTable
          positions={openPos}
          onRowsPerPageChanged={rerender} />
      </Tab>
      <Tab>
        <PositionTable
          positions={closePos}
          onRowsPerPageChanged={rerender} />
      </Tab>
    </Tabs>
  );
};
