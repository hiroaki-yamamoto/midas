import Accordion from '@mui/material/Accordion';
import AccordionSummary from '@mui/material/AccordionSummary';
import AccordionDetails from '@mui/material/AccordionDetails';
import Typo from '@mui/material/Typography';
import { ExpandMoreIcon } from '@mui/icons-material/ExpandMore';

import { Input } from './input.interface';

export default function BotSummaryPanel(input: Input) {
  return (
    <Accordion>
      <AccordionSummary expandIcon={ExpandMoreIcon}>
        <Typo>Hello</Typo>
      </AccordionSummary>
    </Accordion>
  );
}
