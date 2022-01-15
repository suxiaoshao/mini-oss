import { Box, Link, Typography } from '@mui/material';
import { To, Link as RouterLink } from 'react-router-dom';
import imgSrc from './monkey.png';

export interface ErrorPageProps {
  to: To;
}
export function ErrorPage({ to }: ErrorPageProps): JSX.Element {
  return (
    <Box
      sx={{
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      <img src={imgSrc} />
      <Typography sx={{ marginTop: (theme) => theme.spacing(2) }} variant="subtitle1">
        进入了错误页面
        <Link sx={{ marginX: '0.2em' }} to={to} component={RouterLink}>
          点此
        </Link>
        跳转至首页
      </Typography>
    </Box>
  );
}
