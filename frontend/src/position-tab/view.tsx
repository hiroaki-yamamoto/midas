import {
  SyntheticEvent, useState, useRef, useEffect, useMemo, useCallback
} from 'react';

import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import Swiper from 'swiper';
import 'swiper/css';

import { Bot } from '../rpc/bot.zod.ts';
import { Position } from '../rpc/position.zod.ts';
import { PositionStatus } from '../rpc/position-status.zod.ts';

import { PositionTable } from '../position-table/view.tsx';

import { Ctrl } from './controller.ts';

export const PositionTab = (input: { bot: Bot }) => {
  // Swipe panel related controls
  const [index, setIndex] = useState(0);
  const [swiper, setSwiper] = useState<Swiper | null>(null);

  const swiperRef = useRef(null);
  const onChangeIndex = (_event: SyntheticEvent, index: number) => {
    setIndex(index);
    swiper?.slideTo(index);
  };
  useEffect(() => {
    if (!swiperRef.current) {
      return;
    }
    const swiper = new Swiper(swiperRef.current, {
      allowTouchMove: false,
      autoHeight: true,
      on: {
        slideChange: (sw) => setIndex(sw.activeIndex),
      },
    });
    setSwiper(swiper);
    return () => { swiper.destroy(); };
  }, [setIndex, swiperRef, setSwiper]);
  const rerender = useCallback(() => {
    setTimeout(() => {
      swiper?.update();
    }, 0);
  }, [swiper]);

  // Position table related controls
  const [openPos, setOpenPos] = useState<Position[]>([]);
  const [closePos, setClosePos] = useState<Position[]>([]);
  const ctrl = useMemo(() => new Ctrl(input.bot), [input.bot]);
  useEffect(() => {
    ctrl.getPositions(PositionStatus.enum.OPEN).then((pos) => {
      setOpenPos(pos);
      rerender();
    });
    ctrl.getPositions(PositionStatus.enum.CLOSE).then(setClosePos);
  }, [setOpenPos, setClosePos, ctrl, rerender]);

  return (
    <Box>
      <Tabs value={index} onChange={onChangeIndex}>
        <Tab label="Open Positions" />
        <Tab label="Closed Positions" />
      </Tabs>
      <div className='swiper' ref={swiperRef}>
        <div className="swiper-wrapper">
          <div className="swiper-slide">
            <PositionTable
              positions={openPos}
              onRowsPerPageChanged={rerender} />
          </div>
          <div className="swiper-slide">
            <PositionTable
              positions={closePos}
              onRowsPerPageChanged={rerender} />
          </div>
        </div>
      </div>
    </Box>
  );
};
