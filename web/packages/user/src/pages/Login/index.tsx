import { Avatar, Box, Button, TextField, Typography } from '@mui/material';
import { useUserLoginLazyQuery } from 'graphql';
import { SubmitHandler, useForm } from 'react-hook-form';
import { useNavigate } from 'react-router-dom';
import { useAppDispatch } from '../../app/hooks';
import logo from '../../favicon.svg';
import { login } from '../../features/auth/authSlice';

interface LoginForm {
  password: string;
  name: string;
}

export default function Login(): JSX.Element {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginForm>();
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const [fn] = useUserLoginLazyQuery();
  const onSubmit: SubmitHandler<LoginForm> = async (formData) => {
    const data = await fn({ variables: { ...formData } });
    if (data.data) {
      dispatch(login(data.data?.userLogin));
      navigate(-1);
    }
  };

  return (
    <Box
      sx={{
        width: '100%',
        height: '100%',
        maxWidth: '100%',
        maxHeight: '100%',
        display: 'flex',
        justifyContent: 'center',
      }}
    >
      <Box
        onSubmit={handleSubmit(onSubmit)}
        component="form"
        sx={{ display: 'flex', flexDirection: 'column', width: 400, marginTop: '150px', alignItems: 'center' }}
      >
        <Avatar src={logo} sx={{ m: 1 }} />
        <Typography component="h1" variant="h5">
          管理员登陆
        </Typography>
        {/* register your input into the hook by invoking the "register" function */}
        <TextField
          required
          label="用户名"
          {...register('name', { required: true })}
          sx={{ marginTop: (theme) => theme.spacing(2), width: '100%' }}
          helperText={errors.password && '不能为空'}
        />

        {/* include validation with required or other standard HTML validation rules */}
        <TextField
          required
          label="密码"
          type="password"
          {...register('password', { required: true })}
          error={errors.password !== undefined}
          helperText={errors.password && '不能为空'}
          sx={{ marginTop: (theme) => theme.spacing(2), width: '100%' }}
        />
        <Button
          size="large"
          variant="contained"
          type="submit"
          sx={{ marginTop: (theme) => theme.spacing(2), width: '100%' }}
        >
          登陆
        </Button>
      </Box>
    </Box>
  );
}
