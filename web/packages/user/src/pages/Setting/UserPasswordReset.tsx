import { Box, Card, CardHeader, CardContent, TextField, CardActions, TextFieldProps, Button } from '@mui/material';
import { useSnackbar } from 'common';
import { useUpdatePasswordMutation } from 'graphql';
import { useForm, SubmitHandler } from 'react-hook-form';
import { useAppDispatch, useAppSelector } from '../../app/hooks';
import { login } from '../../features/auth/authSlice';

export interface UserPasswordForm {
  newPassword: string;
  oldPassword: string;
}

export default function UserPasswordReset(): JSX.Element {
  const { enqueueSnackbar } = useSnackbar();
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const dispatch = useAppDispatch();
  const [updateInfo] = useUpdatePasswordMutation();
  const { register, handleSubmit } = useForm<UserPasswordForm>();
  const onSubmit: SubmitHandler<UserPasswordForm> = async (formData) => {
    const { data } = await updateInfo({ variables: { data: { auth, ...formData } } });
    enqueueSnackbar('修改成功', { variant: 'success' });
    dispatch(login(data?.updatePassword ?? ''));
  };
  /** 样式  */
  const textFieldProp: TextFieldProps = {
    required: true,
    variant: 'standard',
    fullWidth: true,
    sx: { marginTop: (theme) => theme.spacing(1) },
    type: 'password',
  };
  return (
    <Box sx={(theme) => ({ margin: theme.spacing(3), width: 800, maxWidth: `calc(100% - ${theme.spacing(6)})` })}>
      <Card component="form" onSubmit={handleSubmit(onSubmit)}>
        <CardHeader title="用户信息设置" />
        <CardContent>
          <TextField label="旧密码" {...textFieldProp} {...register('oldPassword', { required: true })} />
          <TextField label="新密码" {...textFieldProp} {...register('newPassword', { required: true })} />
        </CardContent>
        <CardActions>
          <Button variant="contained" sx={{ marginLeft: 'auto' }} type="submit">
            提交
          </Button>
        </CardActions>
      </Card>
    </Box>
  );
}
