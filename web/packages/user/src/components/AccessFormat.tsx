import { Chip } from '@mui/material';
import { BucketAccess, FolderAccess, ObjectAccess } from 'graphql';

export interface AccessFormatProps {
  access: BucketAccess | ObjectAccess | FolderAccess;
}

export default function AccessFormat({ access }: AccessFormatProps): JSX.Element {
  switch (access) {
    case BucketAccess.Open:
      return <Chip label="共有读写" color="primary" />;
    case BucketAccess.Private:
      return <Chip label="私有读写" color="error" />;
    case BucketAccess.ReadOpen:
      return <Chip label="共有读私有写" color="warning" />;
    case ObjectAccess.InheritanceObject:
      return <Chip label="继承" color="primary" />;
    case ObjectAccess.PrivateObject:
      return <Chip label="私有读写" color="error" />;
    case ObjectAccess.ReadOpenObject:
      return <Chip label="共有读私有写" color="warning" />;
    case FolderAccess.InheritanceFolder:
      return <Chip label="继承" color="primary" />;
    case FolderAccess.OpenFolder:
      return <Chip label="共有读写" color="primary" />;
    case FolderAccess.PrivateFolder:
      return <Chip label="私有读写" color="error" />;
    case FolderAccess.ReadOpenFolder:
      return <Chip label="共有读私有写" color="warning" />;
  }
}
