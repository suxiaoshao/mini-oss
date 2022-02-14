import { Box, Typography } from '@mui/material';
import { ReactChild } from 'react';

export default function InfoItem({
  label,
  value,
  last,
}: {
  label: ReactChild;
  value: ReactChild;
  last?: boolean;
}): JSX.Element {
  return (
    <Box sx={{ display: 'flex', alignItems: 'center', marginBottom: (theme) => (last ? 0 : theme.spacing(2)) }}>
      <Typography sx={{ color: (theme) => theme.palette.text.secondary, width: (theme) => theme.spacing(13) }}>
        {label}
      </Typography>
      <Typography>{value}</Typography>
    </Box>
  );
}
