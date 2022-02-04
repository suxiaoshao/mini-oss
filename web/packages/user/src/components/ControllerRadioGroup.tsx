import { FormControl, FormControlLabel, FormLabel, Radio, RadioGroup } from '@mui/material';
import { FocusEventHandler, ReactChild } from 'react';
import * as React from 'react';

export interface ControllerRadioGroupProps<T extends string | number> {
  label: ReactChild;
  name?: string;
  onChange: (event: React.ChangeEvent<HTMLInputElement>, value: string) => void;
  onBlur: FocusEventHandler<HTMLInputElement> | undefined;
  value: T;
  children?: { label: string | number | React.ReactElement; value: T }[];
}

function ControllerRadioGroup<T extends string | number>(
  { label, name, onBlur, value, onChange, children }: ControllerRadioGroupProps<T>,
  ref?: React.Ref<never>,
): JSX.Element {
  return (
    <FormControl required sx={{ marginTop: (theme) => theme.spacing(1) }}>
      <FormLabel>{label}</FormLabel>
      <RadioGroup name={name} value={value} onBlur={onBlur} onChange={onChange} row>
        {children?.map(({ label, value }) => (
          <FormControlLabel value={value} inputRef={ref} control={<Radio />} label={label} key={value} />
        ))}
      </RadioGroup>
    </FormControl>
  );
}

export default React.forwardRef(ControllerRadioGroup) as typeof ControllerRadioGroup;
