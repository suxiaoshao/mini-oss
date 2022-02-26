import { Dialog, DialogContent, DialogTitle, MenuItem } from '@mui/material';
import { useEffect, useState } from 'react';
import InfoItem from '@/pages/ObjectPage/components/InfoItem';
import { useFolderStatisticsLazyQuery } from 'graphql';
import { useAppSelector } from '@/app/hooks';
import prettyBytes from 'pretty-bytes';

export interface ShowStatisticsProps {
  /** action 关闭 */
  menuClose: () => void;
  /** folder 名 */
  folderName: string;
  /** 路径 */
  path: string;
  /** 存储桶名 */
  bucketName: string;
}

export default function ShowStatistics({ menuClose, folderName, bucketName, path }: ShowStatisticsProps): JSX.Element {
  const auth = useAppSelector((state) => state.auth.value);
  const [getStatistics, { data }] = useFolderStatisticsLazyQuery({
    variables: {
      data: {
        auth,
        bucketName,
        path,
      },
    },
  });
  const folderInfo = data?.folderInfo;
  const [open, setOpen] = useState(false);
  useEffect(() => {
    if (open) {
      getStatistics();
    }
  }, [getStatistics, open]);
  const handleClose = () => {
    setOpen(false);
    menuClose();
  };
  return (
    <>
      <MenuItem onClick={() => setOpen(true)}>统计</MenuItem>
      <Dialog fullWidth sx={{ minWidth: 400 }} open={open} onClose={handleClose}>
        <DialogTitle>目录统计</DialogTitle>
        <DialogContent>
          <InfoItem label={'当前目录'} value={folderName} />
          <InfoItem label={'文件夹数量'} value={folderInfo?.folderCount} />
          <InfoItem label={'对象数量'} value={folderInfo?.objectCount} />
          <InfoItem label={'对象大小'} value={prettyBytes(parseInt(folderInfo?.objectSize ?? '0'))} />
        </DialogContent>
      </Dialog>
    </>
  );
}
