import { addMethod, string, StringSchema } from 'yup';

declare module 'yup' {
  interface StringSchema {
    password(): StringSchema;

    name(): StringSchema;

    bucketName(): StringSchema;

    folderName(): StringSchema;
  }
}

addMethod<StringSchema>(string, 'password', function password() {
  return this.required()
    .min(8)
    .min(8, '最小长度为8')
    .max(25, '最长长度为25')
    .test('password', '必须含有字母,数字,特殊字符其中两种', (value) => {
      let numExist = 0;
      let otherExist = 0;
      let alphaExist = 0;
      for (const char of value ?? '') {
        if ('1234567890'.includes(char)) {
          numExist = 1;
        } else if ('qwertyuiopasdfghjklzxcvbnm'.includes(char)) {
          alphaExist = 1;
        } else if ('`~!@#$%^&*()-=_+[]\\{}|;:\'",<.>/?'.includes(char)) {
          otherExist = 1;
        } else {
          return false;
        }
      }
      return numExist + otherExist + alphaExist > 1;
    });
});

addMethod<StringSchema>(string, 'name', function password() {
  return this.required()
    .required()
    .min(4, '最小长度为4')
    .max(25, '最长长度为25')
    .matches(/^[a-zA-Z]/, '必须以字母开头')
    .matches(/^[a-zA-Z_0-9]{4,25}$/, '只能包含大小写字母,数字,_');
});
addMethod<StringSchema>(string, 'bucketName', function password() {
  return this.required()
    .required()
    .min(1, '最小长度为1')
    .max(60, '最长长度为60')
    .matches(/^[a-z-0-9]+$/, '仅支持小写字母、数字和 - 的组合');
});

addMethod<StringSchema>(string, 'folderName', function password() {
  return this.required()
    .required()
    .min(1, '最小长度为1')
    .max(225, '最长长度为225')
    .test('folder', '不能为 "." ".."', (value) => value !== '.' && value !== '..')
    .test('folder', '不能包括 "/"', (value) => !value?.includes('/'))
    .test('folder', '不能为空', (value) => !/^ *$/.test(value ?? ''));
});

export * from 'yup';
