import ControllerRadioGroup from '@/components/ControllerRadioGroup';
import { MenuItem, Dialog, Box, DialogTitle, DialogContent, DialogActions, Button } from '@mui/material';
import { FolderAccess, useUpdateFolderMutation } from 'graphql';
import { useState } from 'react';
import { useForm, SubmitHandler, Controller } from 'react-hook-form';

export interface UpdateFolderActionProps {
  /** 表格重新刷新 */
  refetch: () => void;
  /** action 关闭 */
  menuClose: () => void;
  /** 修改的 bucket 名 */
  path: string;
  /** 之前的描述 */
  access: FolderAccess;
  bucketName: string;
}

export interface UpdateFolderForm {
  access: FolderAccess;
}

export default function UpdateFolderAction({
  menuClose,
  access,
  bucketName,
  path,
  refetch,
}: UpdateFolderActionProps): JSX.Element {
  const [open, setOpen] = useState(false);
  const handleClose = () => {
    setOpen(false);
    menuClose();
  };
  const { control, handleSubmit } = useForm<UpdateFolderForm>({
    defaultValues: {
      access,
    },
  });
  const [updateFolder] = useUpdateFolderMutation();
  const onSubmit: SubmitHandler<UpdateFolderForm> = async (formData) => {
    await updateFolder({ variables: { data: { path, bucketName, ...formData } } });
    refetch();
    handleClose();
  };
  return (
    <>
      <MenuItem onClick={() => setOpen(true)}>修改</MenuItem>
      <Dialog open={open} onClose={handleClose}>
        <Box sx={{ width: 500 }} onSubmit={handleSubmit(onSubmit)} component="form">
          <DialogTitle>修改文件夹信息</DialogTitle>
          <DialogContent>
            <Controller
              name="access"
              control={control}
              rules={{ required: true }}
              render={({ field }) => (
                <ControllerRadioGroup {...field} label={'访问权限'}>
                  {[
                    {
                      label: '继承',
                      value: FolderAccess.InheritanceFolder,
                    },
                    {
                      label: '私有读写',
                      value: FolderAccess.PrivateFolder,
                    },
                    { label: '公有读私有写', value: FolderAccess.ReadOpenFolder },
                    {
                      label: '公有读写',
                      value: FolderAccess.OpenFolder,
                    },
                  ]}
                </ControllerRadioGroup>
              )}
            />
          </DialogContent>
          <DialogActions>
            <Button onClick={handleClose}>取消</Button>
            <Button type="submit">提交</Button>
          </DialogActions>
        </Box>
      </Dialog>
    </>
  );
}
