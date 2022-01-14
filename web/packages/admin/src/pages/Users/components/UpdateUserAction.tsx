import { Box, Button, Dialog, DialogActions, DialogContent, DialogTitle, MenuItem, TextField } from '@mui/material';
import { useUserUpdateMutation } from 'graphql';
import { useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';
import { useAppSelector } from '../../../app/hooks';

export interface UpdateUserActionProps {
  /** 表格重新刷新 */
  refetch: () => void;
  /** action 关闭 */
  menuClose: () => void;
  /** 修改的用户名 */
  name: string;
  /** 之前的描述 */
  description?: string;
}
export interface UpdateUserForm {
  description: string | null;
}

export default function UpdateUserAction({
  refetch,
  menuClose,
  description,
  name,
}: UpdateUserActionProps): JSX.Element {
  const [open, setOpen] = useState(false);
  const handleClose = () => {
    setOpen(false);
    menuClose();
  };
  const { register, handleSubmit } = useForm<UpdateUserForm>({
    defaultValues: {
      description,
    },
  });
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const [updateUser] = useUserUpdateMutation();
  const onSubmit: SubmitHandler<UpdateUserForm> = async (formData) => {
    await updateUser({ variables: { auth, name, description: formData.description || null } });
    refetch();
    handleClose();
  };
  return (
    <>
      <MenuItem onClick={() => setOpen(true)}>修改</MenuItem>
      <Dialog open={open} onClose={handleClose}>
        <Box sx={{ width: 400 }} onSubmit={handleSubmit(onSubmit)} component="form">
          <DialogTitle>修改用户信息</DialogTitle>
          <DialogContent>
            <TextField
              variant="standard"
              sx={{ marginTop: (theme) => theme.spacing(1) }}
              fullWidth
              label="描述"
              {...register('description')}
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
