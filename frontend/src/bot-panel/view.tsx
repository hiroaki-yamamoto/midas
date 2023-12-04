import { useState, useEffect } from 'react';

import Accordion from '@mui/material/Accordion';
import AccordionSummary from '@mui/material/AccordionSummary';
import AccordionDetails from '@mui/material/AccordionDetails';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import Typography from '@mui/material/Typography';


import { Bot } from '../rpc/bot.zod.ts';

import ProfitGraph from '../profit-graph/view.tsx';
import { PositionTab } from '../position-tab/view.tsx';
import { IData } from '../profit-graph/data.interface.ts';

import { Ctrl } from './controller.ts';

function BotPanel(props: { bot: Bot }) {
  const [data, setData] = useState<IData[]>([]);
  useEffect(() => {
    const ctrl = new Ctrl(props.bot);
    ctrl.getData().then((data) => setData(data));
  }, [setData, props.bot]);
  return (
    <Accordion>
      <AccordionSummary expandIcon={<ExpandMoreIcon />}>
        <Typography>{props.bot.name}</Typography>
      </AccordionSummary>
      <AccordionDetails>
        <section>
          <header>
            <h3>Profit Graph</h3>
          </header>
          <ProfitGraph data={data}></ProfitGraph>
          <section>
            <header>
              <h3>Positions</h3>
            </header>
          </section>
          <PositionTab bot={props.bot} />
        </section>
      </AccordionDetails>
    </Accordion>
  );
}

export default BotPanel;
