import { client } from '@/utils/graphql';
import { createAsyncThunk, createSlice, PayloadAction } from '@reduxjs/toolkit';
import { LoginRequest, UserLoginDocument, UserLoginQuery, UserLoginQueryVariables } from 'graphql';

export const login = createAsyncThunk('login', async (data: LoginRequest) => {
  const {
    data: { userLogin },
  } = await client.query<UserLoginQuery, UserLoginQueryVariables>({
    query: UserLoginDocument,
    variables: { data },
  });
  return await userLogin;
});

export const authSlice = createSlice({
  name: 'auth',
  initialState: {
    value: window.localStorage.getItem('user_auth') as null | string,
  },
  reducers: {
    logout: (state) => {
      window.localStorage.removeItem('user_auth');
      state.value = null;
    },
    /** 用于修改密码 */
    resetAuth: (state, { payload }: PayloadAction<string>) => {
      window.localStorage.setItem('user_auth', payload);
      state.value = payload;
    },
  },
  extraReducers: (builder) => {
    builder.addCase(login.fulfilled, (state, { payload }) => {
      window.localStorage.setItem('user_auth', payload);
      state.value = payload;
    });
  },
});

export const { logout, resetAuth } = authSlice.actions;

export default authSlice.reducer;
