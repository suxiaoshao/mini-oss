import { useAppSelector } from '@/app/hooks';
import { yupResolver } from '@hookform/resolvers/yup';
import {
  Dialog,
  Box,
  DialogTitle,
  DialogContent,
  TextField,
  DialogActions,
  Button,
  InputAdornment,
  FormControl,
  FormControlLabel,
  FormLabel,
  Radio,
  RadioGroup,
} from '@mui/material';
import { object, string } from 'common';
import { Access, CreateBucketMutationVariables, useCreateBucketMutation } from 'graphql';
import { useState } from 'react';
import { useForm, SubmitHandler, Controller } from 'react-hook-form';

export type CreateBucketForm = Omit<CreateBucketMutationVariables['data'], 'auth'>;

export interface CreateBucketFabProps {
  /** 表格重新刷新 */
  refetch: () => void;
}

const createBucketSchema = object({
  name: string().name(),
});

export default function CreateBucketButton({ refetch }: CreateBucketFabProps): JSX.Element {
  const [open, setOpen] = useState(false);
  const handleClose = () => {
    setOpen(false);
  };
  const [createBucket] = useCreateBucketMutation();
  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
  } = useForm<CreateBucketForm>({
    defaultValues: { access: Access.Private },
    resolver: yupResolver(createBucketSchema),
  });
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const username = useAppSelector((state) => state.userInfo.name);
  const onSubmit: SubmitHandler<CreateBucketForm> = async (formData) => {
    await createBucket({ variables: { data: { auth, ...formData } } });
    refetch();
    handleClose();
  };

  return (
    <>
      <Button color="primary" size="large" variant="contained" onClick={() => setOpen(true)}>
        创建存储桶
      </Button>
      <Dialog open={open} onClose={handleClose}>
        <Box sx={{ width: 500 }} onSubmit={handleSubmit(onSubmit)} component="form">
          <DialogTitle>新建存储桶</DialogTitle>
          <DialogContent>
            <TextField
              variant="standard"
              required
              sx={{ marginTop: (theme) => theme.spacing(1) }}
              fullWidth
              label="名字"
              InputProps={{ endAdornment: <InputAdornment position="end"> -{username}</InputAdornment> }}
              {...register('name', { required: true })}
              error={errors.name !== undefined}
              helperText={errors.name?.message}
            />
            <Controller
              name="access"
              control={control}
              rules={{ required: true }}
              render={({ field: { onChange, onBlur, value, name, ref } }) => (
                <FormControl sx={{ marginTop: (theme) => theme.spacing(1) }}>
                  <FormLabel>访问权限</FormLabel>
                  <RadioGroup name={name} value={value} onBlur={onBlur} onChange={onChange} row>
                    <FormControlLabel inputRef={ref} value={Access.Private} control={<Radio />} label="私有读写" />
                    <FormControlLabel inputRef={ref} value={Access.Open} control={<Radio />} label="共有读写" />
                    <FormControlLabel inputRef={ref} value={Access.ReadOpen} control={<Radio />} label="共有读私有写" />
                  </RadioGroup>
                </FormControl>
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
