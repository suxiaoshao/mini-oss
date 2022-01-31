import { Chip } from '@mui/material';
import { BucketAccess } from 'graphql';

export interface AccessFormatProps {
  access: BucketAccess;
}
export default function AccessFormat({ access }: AccessFormatProps): JSX.Element {
  if (access === BucketAccess.Open) {
    return <Chip label="共有读写" color="primary" />;
  }
  if (access === BucketAccess.ReadOpen) {
    return <Chip label="共有读私有写" color="warning" />;
  }
  return <Chip label="私有读写" color="error" />;
}
