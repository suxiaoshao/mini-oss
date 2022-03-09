import ControllerRadioGroup from '@/components/ControllerRadioGroup';
import { yupResolver } from '@hookform/resolvers/yup';
import { Button, Dialog, Box, DialogTitle, DialogContent, DialogActions, TextField } from '@mui/material';
import { object, string } from 'common';
import { GetObjectQuery, ObjectAccess, UpdateObjectMutationVariables, useUpdateObjectMutation } from 'graphql';
import { useState } from 'react';
import { useForm, SubmitHandler, Controller } from 'react-hook-form';
import { createSearchParams, useNavigate } from 'react-router-dom';
import HeadersInput from './HeadersInput';

export type UpdateObjectForm = Pick<UpdateObjectMutationVariables['data'], 'newFilename' | 'access' | 'headers'>;

export interface UpdateObjectButtonProps {
  objectInfo: GetObjectQuery['objectInfo'];
  /** 表格重新刷新 */
  reFetch: () => void;
}

const updateObjectSchema = object({
  newFilename: string().folderName(),
});

export default function UpdateObjectButton(props: UpdateObjectButtonProps): JSX.Element {
  const navigate = useNavigate();
  // 控制 dialog
  const [open, setOpen] = useState(false);
  const handleClose = (value?: string) => {
    if (value) {
      navigate({ search: createSearchParams({ filename: value, path: props.objectInfo.path }).toString() });
    }
    setOpen(false);
  };
  return (
    <>
      <Button color="primary" size="large" variant="contained" onClick={() => setOpen(true)}>
        修改对象
      </Button>
      <UpdateObjectDialog {...props} open={open} handleClose={handleClose} />
    </>
  );
}

export interface UpdateObjectDialogProps extends UpdateObjectButtonProps {
  open: boolean;
  handleClose: (newFilename?: string) => void;
}

export function UpdateObjectDialog({
  reFetch,
  objectInfo: { bucketName, path, filename, access, headers },
  open,
  handleClose,
}: UpdateObjectDialogProps): JSX.Element {
  // 表单控制
  const {
    handleSubmit,
    control,
    register,
    formState: { errors },
    watch,
  } = useForm<UpdateObjectForm>({
    defaultValues: {
      newFilename: filename,
      access,
      headers: headers.map(({ key, value }) => ({
        key,
        value,
      })),
    },
    resolver: yupResolver(updateObjectSchema),
  });
  const [updateObjectFn] = useUpdateObjectMutation();

  const onSubmit: SubmitHandler<UpdateObjectForm> = async (formData) => {
    const { data } = await updateObjectFn({
      variables: {
        data: {
          bucketName,
          path,
          filename,
          ...formData,
        },
      },
    });
    handleClose(data?.updateObject.filename);
    reFetch();
  };
  return (
    <Dialog
      PaperProps={{ sx: { maxWidth: 700 } }}
      open={open}
      onClose={() => handleClose()}
      onSubmit={handleSubmit(onSubmit)}
    >
      <Box sx={{ width: 700 }} component="form">
        <DialogTitle>修改对象信息</DialogTitle>
        <DialogContent>
          <TextField
            variant="standard"
            required
            sx={{ marginTop: (theme) => theme.spacing(1) }}
            fullWidth
            label="名字"
            {...register('newFilename', { required: true })}
            error={errors.newFilename !== undefined}
            helperText={errors.newFilename?.message}
          />
          <Controller
            name="access"
            control={control}
            rules={{ required: true }}
            render={({ field }) => (
              <ControllerRadioGroup {...field} label={'访问权限'}>
                {[
                  {
                    label: '继承权限',
                    value: ObjectAccess.InheritanceObject,
                  },
                  {
                    label: '私有读写',
                    value: ObjectAccess.PrivateObject,
                  },
                  { label: '公有读私有写', value: ObjectAccess.ReadOpenObject },
                ]}
              </ControllerRadioGroup>
            )}
          />
          <HeadersInput control={control} value={watch('headers')} register={register} />
        </DialogContent>

        <DialogActions>
          <Button onClick={() => handleClose()}>取消</Button>
          <Button type="submit">提交</Button>
        </DialogActions>
      </Box>
    </Dialog>
  );
}
