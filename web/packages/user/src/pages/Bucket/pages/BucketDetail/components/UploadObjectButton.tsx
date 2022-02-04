import {
  Box,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  IconButton,
  List,
  ListItem,
  ListItemSecondaryAction,
  ListItemText,
  TextField,
} from '@mui/material';
import { useState } from 'react';
import { ObjectAccess } from 'graphql';
import { Controller, SubmitHandler, useForm } from 'react-hook-form';
import prettyBytes from 'pretty-bytes';
import { Delete } from '@mui/icons-material';
import ControllerRadioGroup from '@/components/ControllerRadioGroup';

export type CreateObjectForm = { file: FileList; access: ObjectAccess };

export interface UploadObjectButtonProps {
  path: string;
  bucketName: string;
  /** 表格重新刷新 */
  reFetch: () => void;
}

export default function UploadObjectButton({ reFetch }: UploadObjectButtonProps): JSX.Element {
  const [open, setOpen] = useState(false);
  const handleClose = () => {
    setOpen(false);
  };
  const { register, handleSubmit, control, getValues } = useForm<CreateObjectForm>({
    defaultValues: { access: ObjectAccess.InheritanceObject },
  });
  const onSubmit: SubmitHandler<CreateObjectForm> = async () => {
    reFetch();
    handleClose();
  };
  return (
    <>
      <Button color="primary" size="large" variant="contained" onClick={() => setOpen(true)}>
        上传文件
      </Button>
      <Dialog open={open} onClose={handleClose} onSubmit={handleSubmit(onSubmit)}>
        <Box sx={{ width: 500 }} component="form">
          <DialogTitle>上传文件</DialogTitle>
          <DialogContent>
            <label htmlFor="contained-button-file">
              <TextField {...register('file')} sx={{ display: 'none' }} id="contained-button-file" type="file" />
              <Button variant="contained" component="span">
                选择文件
              </Button>
            </label>
            {getValues('file') && (
              <List>
                <ListItem>
                  <ListItemText
                    primary={getValues('file')[0].name}
                    secondary={prettyBytes(getValues('file')[0].size)}
                  />
                  <ListItemSecondaryAction>
                    <IconButton>
                      <Delete />
                    </IconButton>
                  </ListItemSecondaryAction>
                </ListItem>
              </List>
            )}
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
