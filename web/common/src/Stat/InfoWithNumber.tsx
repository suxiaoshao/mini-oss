import { Box, Typography } from '@mui/material';
import { ReactNode } from 'react';

export interface InfoWithNumberProps {
  name: ReactNode;
  value: ReactNode;
}

export function InfoWithNumber({ name, value }: InfoWithNumberProps) {
  return (
    <Box sx={{ flex: '1 1 0' }}>
      <Typography>{name}</Typography>
      <Typography variant="h4" sx={{ marginTop: (theme) => theme.spacing(1) }}>
        {value}
      </Typography>
    </Box>
  );
}
