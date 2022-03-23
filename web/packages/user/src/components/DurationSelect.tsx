import { DateRangePicker } from '@mui/lab';
import { ToggleButtonGroup, ToggleButton, Box, TextField } from '@mui/material';
import { Dayjs, dayjs } from 'common';
import React from 'react';
import { useState, MouseEvent, useEffect, Dispatch, SetStateAction } from 'react';

export interface DurationSelectProps {
  setStartTime: Dispatch<SetStateAction<number>>;
  setEndTime: Dispatch<SetStateAction<number>>;
}

type DurationLabel = 'today' | 'yesterday' | '7' | '30' | 'other';
export default function DurationSelect({ setStartTime, setEndTime }: DurationSelectProps): JSX.Element {
  const [label, setLabel] = useState<DurationLabel>('today');
  const [[startTime, endTime], setValue] = React.useState<[Dayjs, Dayjs]>([dayjs().startOf('day'), dayjs()]);

  const handleAlignment = (_: MouseEvent<HTMLElement>, newAlignment: DurationLabel) => {
    setLabel(newAlignment);
  };

  useEffect(() => {
    switch (label) {
      case 'today':
        setValue([dayjs().startOf('day'), dayjs()]);
        break;
      case 'yesterday':
        setValue([dayjs().startOf('day').subtract(1, 'day'), dayjs().subtract(1, 'day').endOf('day')]);
        break;
      case '7':
        setValue([dayjs().startOf('day').subtract(7, 'day'), dayjs()]);
        break;
      case '30':
        setValue([dayjs().startOf('day').subtract(30, 'day'), dayjs()]);
        break;
      default:
        break;
    }
  }, [label]);
  useEffect(() => {
    setStartTime(startTime.valueOf());
  }, [setStartTime, startTime]);
  useEffect(() => {
    setEndTime(endTime.valueOf());
  }, [endTime, setEndTime]);

  return (
    <Box sx={{ display: 'flex' }}>
      <ToggleButtonGroup
        sx={{ flex: '1 1 0' }}
        value={label}
        exclusive
        onChange={handleAlignment}
        aria-label="text alignment"
      >
        <ToggleButton value="today">今天</ToggleButton>
        <ToggleButton value="yesterday">昨天</ToggleButton>
        <ToggleButton value="7">近七天</ToggleButton>
        <ToggleButton value="30">近三十天</ToggleButton>
      </ToggleButtonGroup>
      <DateRangePicker
        startText="开始时间"
        endText="截至时间"
        value={[startTime, endTime]}
        onChange={(newValue) => {
          setValue([newValue[0] ?? dayjs(), newValue[1] ?? dayjs()]);
          setLabel('other');
        }}
        renderInput={(startProps, endProps) => (
          <React.Fragment>
            <TextField {...startProps} />
            <Box sx={{ mx: 2 }}> 到 </Box>
            <TextField {...endProps} />
          </React.Fragment>
        )}
      />
    </Box>
  );
}
