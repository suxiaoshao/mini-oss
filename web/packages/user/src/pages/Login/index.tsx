import { Avatar, Box, Button, TextField, Typography } from '@mui/material';
import { object, string } from 'common';
import { useEffect } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { useAppDispatch, useAppSelector } from '../../app/hooks';
import logo from '../../favicon.svg';
import { login } from '../../features/auth/authSlice';
import { yupResolver } from '@hookform/resolvers/yup';

interface LoginForm {
  password: string;
  name: string;
}

const loginSchema = object({
  name: string().name(),
  password: string().password(),
});

export default function Login(): JSX.Element {
  /** 表单 */
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginForm>({
    resolver: yupResolver(loginSchema),
  });
  const dispatch = useAppDispatch();
  const onSubmit: SubmitHandler<LoginForm> = async (formData) => {
    dispatch(login(formData));
  };

  /** 跳转 */
  const navigate = useNavigate();
  const [urlSearch] = useSearchParams();
  const auth = useAppSelector((state) => state.auth.value);
  useEffect(() => {
    if (auth !== null) {
      const from = urlSearch.get('from');
      if (from === null) {
        navigate('/');
      } else {
        navigate(from);
      }
    }
  }, [auth, navigate, urlSearch]);

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
          登陆
        </Typography>
        {/* register your input into the hook by invoking the "register" function */}
        <TextField
          required
          label="用户名"
          {...register('name', { required: true })}
          sx={{ marginTop: (theme) => theme.spacing(2), width: '100%' }}
          helperText={errors.name?.message}
          error={errors.name !== undefined}
        />

        {/* include validation with required or other standard HTML validation rules */}
        <TextField
          required
          label="密码"
          type="password"
          {...register('password', { required: true })}
          error={errors.password !== undefined}
          helperText={errors.password?.message}
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
