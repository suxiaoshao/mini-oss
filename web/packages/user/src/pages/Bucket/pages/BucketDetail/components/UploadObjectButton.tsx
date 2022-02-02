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
  IconButton,
  List,
  ListItem,
  ListItemSecondaryAction,
  ListItemText,
  Radio,
  RadioGroup,
  TextField,
} from '@mui/material';
import { useState } from 'react';
import { CreateObjectMutationVariables, ObjectAccess } from 'graphql';
import { Controller, SubmitHandler, useForm } from 'react-hook-form';
import prettyBytes from 'pretty-bytes';
import { Delete } from '@mui/icons-material';

export type CreateObjectForm = Pick<CreateObjectMutationVariables['data'], 'access'> & { file: FileList };

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
    defaultValues: { access: ObjectAccess.BucketObject },
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
