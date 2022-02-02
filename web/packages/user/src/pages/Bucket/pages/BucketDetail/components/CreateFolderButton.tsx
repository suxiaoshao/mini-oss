import {
  Box,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  FormControl,
  FormControlLabel,
  FormLabel,
  Radio,
  RadioGroup,
  TextField,
} from '@mui/material';
import { CreateFolderMutationVariables, ObjectAccess, useCreateFolderMutation } from 'graphql';
import { useState } from 'react';
import { Controller, SubmitHandler, useForm } from 'react-hook-form';
import { object, string } from 'common';
import { yupResolver } from '@hookform/resolvers/yup';
import { useAppSelector } from '@/app/hooks';

export type CreateFolderForm = Pick<CreateFolderMutationVariables['data'], 'access' | 'path'>;

export interface CreateFolderButtonProps {
  fatherPath: string;
  bucketName: string;
  /** 表格重新刷新 */
  reFetch: () => void;
}

const createFolderSchema = object({
  path: string().folderName(),
});

export default function CreateFolderButton({ fatherPath, bucketName, reFetch }: CreateFolderButtonProps): JSX.Element {
  const [open, setOpen] = useState(false);
  const handleClose = () => {
    setOpen(false);
  };
  const [createFolder] = useCreateFolderMutation();

  const {
    register,
    handleSubmit,
    control,
    formState: { errors },
  } = useForm<CreateFolderForm>({
    defaultValues: { access: ObjectAccess.BucketObject },
    resolver: yupResolver(createFolderSchema),
  });
  const auth = useAppSelector((state) => state.auth.value) ?? '';
  const onSubmit: SubmitHandler<CreateFolderForm> = async (formData) => {
    await createFolder({ variables: { data: { auth, fatherPath, bucketName, ...formData } } });
    reFetch();
    handleClose();
  };
  return (
    <>
      <Button
        onClick={() => setOpen(true)}
        variant="outlined"
        sx={{ marginLeft: (theme) => theme.spacing(1) }}
        size="large"
      >
        添加文件夹
      </Button>
      <Dialog open={open} onClose={handleClose}>
        <Box sx={{ width: 500 }} onSubmit={handleSubmit(onSubmit)} component="form">
          <DialogTitle>新建文件夹</DialogTitle>
          <DialogContent>
            <TextField
              variant="standard"
              required
              sx={{ marginTop: (theme) => theme.spacing(1) }}
              fullWidth
              label="文件夹名"
              {...register('path', { required: true })}
              error={errors.path !== undefined}
              helperText={errors.path?.message}
            />
            <Controller
              name="access"
              control={control}
              rules={{ required: true }}
              render={({ field: { onChange, onBlur, value, name, ref } }) => (
                <FormControl sx={{ marginTop: (theme) => theme.spacing(1) }}>
                  <FormLabel>访问权限</FormLabel>
                  <RadioGroup name={name} value={value} onBlur={onBlur} onChange={onChange} row>
                    <FormControlLabel
                      inputRef={ref}
                      value={ObjectAccess.BucketObject}
                      control={<Radio />}
                      label="继承权限"
                    />
                    <FormControlLabel
                      inputRef={ref}
                      value={ObjectAccess.PrivateObject}
                      control={<Radio />}
                      label="私有读写"
                    />
                    <FormControlLabel
                      inputRef={ref}
                      value={ObjectAccess.ReadOpenObject}
                      control={<Radio />}
                      label="共有读私有写"
                    />
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
