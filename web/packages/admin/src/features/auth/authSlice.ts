import { client } from '@/utils/graphql';
import { createAsyncThunk, createSlice } from '@reduxjs/toolkit';
import { LoginRequest, ManagerLoginQuery, ManagerLoginQueryVariables, ManagerLoginDocument } from 'graphql';

export const login = createAsyncThunk('login', async (data: LoginRequest) => {
  const {
    data: { managerLogin },
  } = await client.query<ManagerLoginQuery, ManagerLoginQueryVariables>({
    query: ManagerLoginDocument,
    variables: { data },
  });
  return await managerLogin;
});

export const authSlice = createSlice({
  name: 'auth',
  initialState: {
    value: window.localStorage.getItem('admin_auth') as null | string,
  },
  reducers: {
    logout: (state) => {
      window.localStorage.removeItem('admin_auth');
      state.value = null;
    },
  },
  extraReducers: (builder) => {
    builder.addCase(login.fulfilled, (state, { payload }) => {
      window.localStorage.setItem('admin_auth', payload);
      state.value = payload;
    });
  },
});

export const { logout } = authSlice.actions;

export default authSlice.reducer;
