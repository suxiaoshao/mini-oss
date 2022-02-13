import { useAppSelector } from '@/app/hooks';
import { TableActions } from 'common';
import { useDeleteFolderMutation, useDeleteObjectMutation } from 'graphql';
import { FolderTableData } from '..';
import UpdateFolderAction from './UpdateFolderAction';

export interface FolderListActionsProps {
  item: FolderTableData;
  refetch: () => void;
}
export default function FolderListAction({ item, refetch }: FolderListActionsProps): JSX.Element {
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const [deleteFolder] = useDeleteFolderMutation();
  const [deleteObject] = useDeleteObjectMutation();
  if (item.__typename === 'FolderInfo') {
    return (
      <TableActions>
        {(onClose) => [
          {
            text: '删除',
            onClick: async () => {
              await deleteFolder({ variables: { data: { auth, bucketName: item.bucketName, path: item.path } } });
              refetch();
              onClose();
            },
          },
          <UpdateFolderAction key={2} menuClose={onClose} refetch={refetch} {...item} />,
        ]}
      </TableActions>
    );
  }
  return (
    <TableActions>
      {(onClose) => [
        {
          text: '删除',
          onClick: async () => {
            await deleteObject({
              variables: { data: { auth, bucketName: item.bucketName, path: item.path, filename: item.filename } },
            });
            refetch();
            onClose();
          },
        },
      ]}
    </TableActions>
  );
}
