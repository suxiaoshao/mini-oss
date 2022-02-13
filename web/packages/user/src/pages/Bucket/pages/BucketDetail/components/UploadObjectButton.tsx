import { Box, Button, Dialog, DialogActions, DialogContent, DialogTitle } from '@mui/material';
import { useState } from 'react';
import { ObjectAccess } from 'graphql';
import { Controller, SubmitHandler, useFieldArray, useForm } from 'react-hook-form';
import ControllerRadioGroup from '@/components/ControllerRadioGroup';
import { axiosInstance, getPath } from '@/utils/axios';
import ControllerFiles from '@/pages/Bucket/components/ControllerFiles';
import { array, object } from 'common';
import { yupResolver } from '@hookform/resolvers/yup';

export type CreateObjectForm = { file: File[]; access: ObjectAccess };

export interface UploadObjectButtonProps {
  path: string;
  bucketName: string;
  /** 表格重新刷新 */
  reFetch: () => void;
}

const uploadObjectSchema = object({
  file: array().min(1, '不能上传空文件'),
});

export default function UploadObjectButton({ reFetch, path, bucketName }: UploadObjectButtonProps): JSX.Element {
  // 表单控制
  const { handleSubmit, control, watch } = useForm<CreateObjectForm>({
    defaultValues: { access: ObjectAccess.InheritanceObject, file: [] },
    resolver: yupResolver(uploadObjectSchema),
  });
  const { append, remove } = useFieldArray({
    control,
    name: 'file',
  });

  const onSubmit: SubmitHandler<CreateObjectForm> = async ({ file: files, access }) => {
    const promises = files.map(async (file) => {
      const data = await file.arrayBuffer();

      return await axiosInstance.put(getPath(bucketName, path, file.name), data, {
        headers: { 'object-access': access },
      });
    });
    await Promise.all(promises);
    reFetch();
    handleClose();
  };
  // 控制 dialog
  const [open, setOpen] = useState(false);
  const handleClose = () => {
    setOpen(false);
    remove();
  };
  return (
    <>
      <Button color="primary" size="large" variant="contained" onClick={() => setOpen(true)}>
        上传文件
      </Button>
      <Dialog
        PaperProps={{ sx: { maxWidth: 700 } }}
        open={open}
        onClose={handleClose}
        onSubmit={handleSubmit(onSubmit)}
      >
        <Box sx={{ width: 700 }} component="form">
          <DialogTitle>上传文件</DialogTitle>
          <DialogContent>
            <ControllerFiles remove={remove} append={append} name="file" value={watch('file')} label="选择文件" />
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
                    { label: '共有读私有写', value: ObjectAccess.ReadOpenObject },
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
