import { addMethod, string, StringSchema } from 'yup';

declare module 'yup' {
  interface StringSchema {
    password(): StringSchema;
    name(): StringSchema;
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
      if (numExist + otherExist + alphaExist <= 1) {
        return false;
      }
      return true;
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

export * from 'yup';
