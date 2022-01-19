import { Dialog, Box, DialogTitle, DialogContent, DialogActions, Button, TextField } from '@mui/material';
import { UserCreateMutationVariables, useUserCreateMutation } from 'graphql';
import { useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';
import { useAppSelector } from '../../../app/hooks';

export type CreateUserForm = Omit<UserCreateMutationVariables['data'], 'auth'>;

export interface CreateUserFabProps {
  /** 表格重新刷新 */
  refetch: () => void;
}

export default function CreateUserButton({ refetch }: CreateUserFabProps): JSX.Element {
  const [open, setOpen] = useState(false);
  const handleClose = () => {
    setOpen(false);
  };
  const [createUser] = useUserCreateMutation();
  const { register, handleSubmit } = useForm<CreateUserForm>();
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const onSubmit: SubmitHandler<CreateUserForm> = async (formData) => {
    await createUser({ variables: { data: { auth, ...formData, description: formData.description || null } } });
    refetch();
    handleClose();
  };
  return (
    <>
      <Button color="primary" variant="contained" size="large" onClick={() => setOpen(true)}>
        添加新用户
      </Button>
      <Dialog open={open} onClose={handleClose}>
        <Box sx={{ width: 600 }} onSubmit={handleSubmit(onSubmit)} component="form">
          <DialogTitle>新建用户</DialogTitle>
          <DialogContent>
            <TextField
              variant="standard"
              required
              sx={{ marginTop: (theme) => theme.spacing(1) }}
              fullWidth
              label="用户名"
              {...register('name', { required: true })}
            />
            <TextField
              variant="standard"
              required
              type={'password'}
              sx={{ marginTop: (theme) => theme.spacing(1) }}
              fullWidth
              label="密码"
              {...register('password', { required: true })}
            />
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
