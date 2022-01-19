import { useAppDispatch, useAppSelector } from '@/app/hooks';
import { updateUserInfo } from '@/features/userInfo/userInfoSlice';
import { Box, Button, Card, CardActions, CardContent, CardHeader, TextField, TextFieldProps } from '@mui/material';
import { format, useSnackbar } from 'common';
import { useUpdateInfoMutation } from 'graphql';
import { useCallback } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';

export interface UserInfoForm {
  description: string | null;
}

export default function UserInfoSetting(): JSX.Element {
  const { enqueueSnackbar } = useSnackbar();
  /** 获取全局信息 */
  const auth = useAppSelector((state) => state.auth.value);
  const { description, name, createTime, updateTime } = useAppSelector((state) => state.userInfo);
  /** 更新用户信息 */
  const dispatch = useAppDispatch();
  const refetch = useCallback(() => {
    if (auth !== null) {
      dispatch(updateUserInfo(auth));
    }
  }, [auth, dispatch]);
  const [updateInfo] = useUpdateInfoMutation();
  /** 表单相关 */
  const { register, handleSubmit } = useForm<UserInfoForm>({
    defaultValues: { description },
  });
  const onSubmit: SubmitHandler<UserInfoForm> = async (formData) => {
    await updateInfo({ variables: { data: { auth: auth ?? '', description: formData.description || null } } });
    enqueueSnackbar('修改成功', { variant: 'success' });
    await refetch();
  };
  /** 样式  */
  const textFieldProp: TextFieldProps = {
    variant: 'standard',
    fullWidth: true,
    InputProps: {
      readOnly: true,
    },
    sx: { marginTop: (theme) => theme.spacing(1) },
  };
  return (
    <Box sx={(theme) => ({ margin: theme.spacing(3), width: 800, maxWidth: `calc(100% - ${theme.spacing(6)})` })}>
      <Card component="form" onSubmit={handleSubmit(onSubmit)}>
        <CardHeader title="用户信息设置" />
        <CardContent>
          <TextField label="用户名" {...textFieldProp} value={name ?? ''} />
          <TextField label="创建时间" {...textFieldProp} value={format(createTime)} />
          <TextField label="更新时间" {...textFieldProp} value={format(updateTime)} />
          <TextField label="描述" {...textFieldProp} InputProps={{}} {...register('description')} />
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
