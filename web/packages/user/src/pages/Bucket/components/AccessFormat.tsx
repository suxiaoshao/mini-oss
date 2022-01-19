import { Chip } from '@mui/material';
import { Access } from 'graphql';

export interface AccessFormatProps {
  access: Access;
}
export default function AccessFormat({ access }: AccessFormatProps): JSX.Element {
  if (access === Access.Open) {
    return <Chip label="共有读写" color="primary" />;
  }
  if (access === Access.ReadOpen) {
    return <Chip label="共有读私有写" color="warning" />;
  }
  return <Chip label="私有读写" color="error" />;
}
