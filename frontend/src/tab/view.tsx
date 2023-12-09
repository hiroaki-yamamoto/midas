import {
  SyntheticEvent, useState, useRef, useEffect, ReactNode,
} from 'react';

import MatTabs from '@mui/material/Tabs';
import MatTab from '@mui/material/Tab';
import Box from '@mui/material/Box';
import Swiper from 'swiper';
import 'swiper/css';

export const Tab = (props: { children: ReactNode }) => {
  return (
    <div className="swiper-slide">
      {props.children}
    </div>
  );
};

export const Tabs = (props: {
  labels: string[],
  children: ReactNode,
  onSwiperReady?: (swiper: Swiper) => void,
}) => {
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
    if (props.onSwiperReady) {
      props.onSwiperReady(swiper);
    }
    setSwiper(swiper);
  }, [setIndex, swiperRef, setSwiper, props]);

  return (
    <Box>
      <MatTabs value={index} onChange={onChangeIndex}>
        {
          props.labels.map((label) => {
            return (<MatTab label={label} />);
          })
        }
      </MatTabs>
      <div className='swiper' ref={swiperRef}>
        <div className="swiper-wrapper">
          {props.children}
        </div>
      </div>
    </Box>
  );
};
