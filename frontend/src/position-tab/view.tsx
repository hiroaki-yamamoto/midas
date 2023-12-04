import { SyntheticEvent, useState, useRef, useEffect } from 'react';

import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import Swiper from 'swiper';
import 'swiper/css';

import { Bot } from '../rpc/bot.zod.ts';

export const PositionTab = (input: { bot: Bot }) => {
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
      on: {
        slideChange: (sw) => setIndex(sw.activeIndex),
      },
    });
    setSwiper(swiper);
    return () => { swiper.destroy(); };
  }, [setIndex, swiperRef, setSwiper]);
  return (
    <Box>
      <Tabs value={index} onChange={onChangeIndex}>
        <Tab label="Open Positions" />
        <Tab label="Closed Positions" />
      </Tabs>
      <div className='swiper' ref={swiperRef}>
        <div className="swiper-wrapper">
          <div className="swiper-slide">Open</div>
          <div className="swiper-slide">Closed</div>
        </div>
      </div>
    </Box>
  );
};
