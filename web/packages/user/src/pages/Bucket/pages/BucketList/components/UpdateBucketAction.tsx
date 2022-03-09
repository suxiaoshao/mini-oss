import { useAppSelector } from '@/app/hooks';
import { MenuItem, Dialog, Box, DialogTitle, DialogContent, DialogActions, Button } from '@mui/material';
import { BucketAccess, useUpdateBucketMutation } from 'graphql';
import { useState } from 'react';
import { Controller, SubmitHandler, useForm } from 'react-hook-form';
import ControllerRadioGroup from '@/components/ControllerRadioGroup';

export interface UpdateBucketActionProps {
  /** 表格重新刷新 */
  refetch: () => void;
  /** action 关闭 */
  menuClose: () => void;
  /** 修改的 bucket 名 */
  name: string;
  /** 之前的描述 */
  access: BucketAccess;
}

export interface UpdateBucketForm {
  access: BucketAccess;
}

export default function UpdateBucketAction({ refetch, menuClose, access, name }: UpdateBucketActionProps): JSX.Element {
  const [open, setOpen] = useState(false);
  const handleClose = () => {
    setOpen(false);
    menuClose();
  };
  const { control, handleSubmit } = useForm<UpdateBucketForm>({
    defaultValues: {
      access,
    },
  });
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const [updateBukcet] = useUpdateBucketMutation();
  const onSubmit: SubmitHandler<UpdateBucketForm> = async (formData) => {
    await updateBukcet({ variables: { data: { auth, name, ...formData } } });
    refetch();
    handleClose();
  };
  return (
    <>
      <MenuItem onClick={() => setOpen(true)}>修改</MenuItem>
      <Dialog open={open} onClose={handleClose}>
        <Box sx={{ width: 500 }} onSubmit={handleSubmit(onSubmit)} component="form">
          <DialogTitle>修改用户信息</DialogTitle>
          <DialogContent>
            <Controller
              name="access"
              control={control}
              rules={{ required: true }}
              render={({ field }) => (
                <ControllerRadioGroup {...field} label={'访问权限'}>
                  {[
                    {
                      label: '私有读写',
                      value: BucketAccess.Private,
                    },
                    { label: '公有读私有写', value: BucketAccess.ReadOpen },
                    {
                      label: '公有读写',
                      value: BucketAccess.Open,
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
