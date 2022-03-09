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
} from '@mui/material';
import { object, string } from 'common';
import { BucketAccess, CreateBucketMutationVariables, useCreateBucketMutation } from 'graphql';
import { useState } from 'react';
import { useForm, SubmitHandler, Controller } from 'react-hook-form';
import ControllerRadioGroup from '@/components/ControllerRadioGroup';

export type CreateBucketForm = Omit<CreateBucketMutationVariables['data'], 'auth'>;

export interface CreateBucketFabProps {
  /** 表格重新刷新 */
  reFetch: () => void;
}

const createBucketSchema = object({
  name: string().bucketName(),
});

export default function CreateBucketButton({ reFetch }: CreateBucketFabProps): JSX.Element {
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
    defaultValues: { access: BucketAccess.Private },
    resolver: yupResolver(createBucketSchema),
  });
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const username = useAppSelector((state) => state.userInfo.name);
  const onSubmit: SubmitHandler<CreateBucketForm> = async (formData) => {
    await createBucket({ variables: { data: { auth, ...formData } } });
    reFetch();
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
