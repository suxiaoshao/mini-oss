import { Box, Button, Card, CardActions, CardContent, CardHeader, TextField, TextFieldProps } from '@mui/material';
import { format, useSnackbar } from 'common';
import { useSelfUserInfoQuery, useUpdateInfoMutation } from 'graphql';
import { SubmitHandler, useForm } from 'react-hook-form';
import { useAppSelector } from '../../app/hooks';

export interface UserInfoForm {
  description: string | null;
}

export default function UserInfoSetting(): JSX.Element {
  const { enqueueSnackbar } = useSnackbar();
  const auth = useAppSelector((state) => state.auth.value);
  const { data: { selfUserInfo } = {}, refetch } = useSelfUserInfoQuery({
    variables: { auth: auth ?? '' },
    skip: auth === null,
  });
  const [updateInfo] = useUpdateInfoMutation();
  const { register, handleSubmit } = useForm<UserInfoForm>({
    defaultValues: { description: selfUserInfo?.description },
  });
  const onSubmit: SubmitHandler<UserInfoForm> = async (formData) => {
    await updateInfo({ variables: { auth: auth ?? '', description: formData.description || null } });
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
          <TextField label="用户名" value={selfUserInfo?.name ?? ''} {...textFieldProp} />
          <TextField label="创建时间" value={format(selfUserInfo?.createTime)} {...textFieldProp} />
          <TextField label="更新时间" value={format(selfUserInfo?.updateTime)} {...textFieldProp} />
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
