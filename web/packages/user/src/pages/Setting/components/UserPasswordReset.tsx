import { useAppDispatch } from '@/app/hooks';
import { resetAuth } from '@/features/auth/authSlice';
import { yupResolver } from '@hookform/resolvers/yup';
import { Box, Card, CardHeader, CardContent, TextField, CardActions, TextFieldProps, Button } from '@mui/material';
import { object, string, useSnackbar } from 'common';
import { useUpdatePasswordMutation } from 'graphql';
import { useForm, SubmitHandler } from 'react-hook-form';

export interface UserPasswordForm {
  newPassword: string;
  oldPassword: string;
}

const updatePasswordSchema = object({
  oldPassword: string().password(),
  newPassword: string().password(),
});

export default function UserPasswordReset(): JSX.Element {
  const { enqueueSnackbar } = useSnackbar();
  const dispatch = useAppDispatch();
  const [updateInfo] = useUpdatePasswordMutation();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<UserPasswordForm>({
    resolver: yupResolver(updatePasswordSchema),
  });
  const onSubmit: SubmitHandler<UserPasswordForm> = async (formData) => {
    const { data } = await updateInfo({ variables: { data: { ...formData } } });
    enqueueSnackbar('修改成功', { variant: 'success' });
    dispatch(resetAuth(data?.updatePassword ?? ''));
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
          <TextField
            label="旧密码"
            {...textFieldProp}
            {...register('oldPassword', { required: true })}
            helperText={errors.oldPassword?.message}
            error={errors.oldPassword !== undefined}
          />
          <TextField
            label="新密码"
            {...textFieldProp}
            {...register('newPassword', { required: true })}
            helperText={errors.newPassword?.message}
            error={errors.newPassword !== undefined}
          />
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
