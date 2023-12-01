import Accordion from '@mui/material/Accordion';
import AccordionSummary from '@mui/material/AccordionSummary';
import AccordionDetails from '@mui/material/AccordionDetails';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import Typography from '@mui/material/Typography';

import { Bot } from '../rpc/bot.zod.ts';

function BotPanel(props: { bot: Bot }) {
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
        </section>
      </AccordionDetails>
    </Accordion>
  );
}

export default BotPanel;
